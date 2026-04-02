// mochou-p/surrender/src/lib.rs

use std::num::NonZeroU32;
use std::rc::Rc;
use softbuffer::{Buffer, Context, Surface};
use winit::application::ApplicationHandler;
use winit::event::{StartCause, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop, OwnedDisplayHandle};
use winit::window::{Window, WindowAttributes, WindowId};

pub use winit;


pub struct App<T: Default> {
    event_loop:   Option<EventLoop<()>>,
    context:      Context<OwnedDisplayHandle>,
    window_attrs: Option<WindowAttributes>,
    state:        AppState,
    data:         T,
    load:         Option<Box<dyn FnOnce(&mut T)>>,
    quit:         Option<Box<dyn FnOnce(&mut T)>>,
    update:       Box<dyn Fn(&mut T)>,
    draw:         Box<dyn Fn(&mut T, Canvas<'_>)>
}

impl<T: Default> App<T> {
    pub fn new() -> Self {
        let event_loop = EventLoop::new().unwrap();
        let context    = Context::new(event_loop.owned_display_handle()).unwrap();
        
        Self {
            event_loop:   Some(event_loop),
            context,
            window_attrs: Some(Window::default_attributes().with_title("surrender")),
            state:        AppState::Initial,
            data:         T::default(),
            load:         None,
            quit:         None,
            update:       Box::new(|_   | {}),
            draw:         Box::new(|_, _| {})
        }
    }

    // NOTE: these moves are probably bad, could return T<Wants*> like wtransport,
    //       this also helps with only letting these callbacks be set once,
    //       but also constricts the order without trait bloat

    pub fn window(mut self, attrs: impl FnOnce(&mut T, WindowAttributes) -> WindowAttributes) -> Self {
        self.window_attrs = Some(attrs(&mut self.data, self.window_attrs.take().unwrap()));
        self
    }

    pub fn   load(mut self,   load: impl FnOnce(&mut T)             + 'static) -> Self { self.load   = Some(Box::new(load  )); self }
    pub fn   quit(mut self,   quit: impl FnOnce(&mut T)             + 'static) -> Self { self.quit   = Some(Box::new(quit  )); self }
    pub fn update(mut self, update: impl Fn    (&mut T)             + 'static) -> Self { self.update =      Box::new(update);  self }
    pub fn   draw(mut self,   draw: impl Fn    (&mut T, Canvas<'_>) + 'static) -> Self { self.draw   =      Box::new(draw  );  self }

    pub fn run(mut self) {
        if let Some(load) = self.load.take() {
            load(&mut self.data);
        }

        self.event_loop.take().unwrap().run_app(&mut self).unwrap();
    }
}

enum AppState {
    Initial,
    Suspended { window:  Rc<Window> },
    Running   { surface: Surface<OwnedDisplayHandle, Rc<Window>> }
}

impl<T: Default> ApplicationHandler for App<T> {
    fn new_events(&mut self, event_loop: &ActiveEventLoop, cause: StartCause) {
        if cause == StartCause::Init {
            let window = event_loop.create_window(self.window_attrs.take().unwrap()).unwrap();
            self.state = AppState::Suspended { window: Rc::new(window) };
        }
    }

    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {
        let AppState::Suspended { window } = &mut self.state else {
            unreachable!("got resumed event while not suspended");
        };

        let mut surface = Surface::new(&self.context, window.clone()).unwrap();

        let size = window.inner_size();
        if let (Some(width), Some(height)) = (NonZeroU32::new(size.width), NonZeroU32::new(size.height)) {
            surface.resize(width, height).unwrap();
        }

        self.state = AppState::Running { surface };
    }

    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
        let AppState::Running { surface } = &mut self.state else {
            unreachable!("got resumed event while not running");
        };

        let window = surface.window().clone();
        self.state = AppState::Suspended { window };
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        (self.update)(&mut self.data);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id:  WindowId,
        event:      WindowEvent,
    ) {
        let AppState::Running { surface } = &mut self.state else {
            unreachable!("got window event while suspended");
        };

        if surface.window().id() != window_id {
            return;
        }

        match event {
            WindowEvent::Resized(size) => {
                if let (Some(width), Some(height)) = (NonZeroU32::new(size.width), NonZeroU32::new(size.height)) {
                    surface.resize(width, height).unwrap();
                }
            },
            WindowEvent::RedrawRequested => {
                let mut buffer = surface.buffer_mut().unwrap();
                let     canvas = Canvas(&mut buffer);

                (self.draw)(&mut self.data, canvas);

                buffer.present().unwrap();
            },
            WindowEvent::CloseRequested => {
                if let Some(quit) = self.quit.take() {
                    quit(&mut self.data);
                }

                event_loop.exit();
            },
            _ => ()
        }
    }
}

#[derive(Clone, Copy)]
pub struct Canvas<'a>(*mut Buffer<'a, OwnedDisplayHandle, Rc<Window>>);

impl Canvas<'_> {
}

