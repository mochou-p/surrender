// mochou-p/surrender/src/lib.rs

use std::num::NonZeroU32;
use std::rc::Rc;
use std::time::Instant;
use softbuffer::{Context, Surface};
use winit::application::ApplicationHandler;
use winit::event::{MouseButton, MouseScrollDelta, StartCause, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop, OwnedDisplayHandle};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowAttributes, WindowId};

pub use winit;


pub struct App<T: Default> {
    event_loop:     Option<EventLoop<()>>,
    context:        Context<OwnedDisplayHandle>,
    window_attrs:   Option<WindowAttributes>,
    state:          AppState,
    timer:          Instant,
    data:           T,
    load:           Option<Box<dyn FnOnce(&mut T)>>,
    quit:           Option<Box<dyn FnOnce(&mut T)>>,
    quit_if:        Box<dyn Fn(&mut T                   ) -> bool>,
    resize:         Box<dyn Fn(&mut T, u32,         u32 )        >,
    focus:          Box<dyn Fn(&mut T, bool             )        >,
    keyboard:       Box<dyn Fn(&mut T, KeyCode,     bool)        >,
    cursor:         Box<dyn Fn(&mut T, f64,         f64 )        >,
    hover:          Box<dyn Fn(&mut T, bool             )        >,
    scroll:         Box<dyn Fn(&mut T, f32,         f32 )        >,
    mouse:          Box<dyn Fn(&mut T, MouseButton, bool)        >,
    update:         Box<dyn Fn(&mut T, f32              )        >,
    draw:           Box<dyn Fn(&mut T, &mut Canvas<'_>  )        >
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
            timer:        Instant::now(),
            data:         T::default(),
            load:         None,
            quit:         None,
            quit_if:      Box::new(|_      | false),
            resize:       Box::new(|_, _, _| {   }),
            focus:        Box::new(|_, _   | {   }),
            keyboard:     Box::new(|_, _, _| {   }),
            cursor:       Box::new(|_, _, _| {   }),
            hover:        Box::new(|_, _   | {   }),
            scroll:       Box::new(|_, _, _| {   }),
            mouse:        Box::new(|_, _, _| {   }),
            update:       Box::new(|_, _   | {   }),
            draw:         Box::new(|_, _   | {   })
        }
    }

    // NOTE: these moves are probably bad, could return T<Wants*> like wtransport,
    //       this also helps with only letting these callbacks be set once,
    //       but also constricts the order without trait bloat

    pub fn window(mut self, attrs: impl FnOnce(&mut T, WindowAttributes) -> WindowAttributes) -> Self {
        self.window_attrs = Some(attrs(&mut self.data, self.window_attrs.take().unwrap()));
        self
    }

    pub fn     load(mut self,     load: impl FnOnce(&mut T                   )         + 'static) -> Self { self.load     = Some(Box::new(load    )); self }
    pub fn     quit(mut self,     quit: impl FnOnce(&mut T                   )         + 'static) -> Self { self.quit     = Some(Box::new(quit    )); self }
    pub fn  quit_if(mut self,  quit_if: impl Fn    (&mut T                   ) -> bool + 'static) -> Self { self.quit_if  =      Box::new(quit_if ) ; self }
    pub fn   resize(mut self,   resize: impl Fn    (&mut T, u32,         u32 )         + 'static) -> Self { self.resize   =      Box::new(resize  ) ; self }
    pub fn    focus(mut self,    focus: impl Fn    (&mut T, bool             )         + 'static) -> Self { self.focus    =      Box::new(focus   ) ; self }
    pub fn keyboard(mut self, keyboard: impl Fn    (&mut T, KeyCode,     bool)         + 'static) -> Self { self.keyboard =      Box::new(keyboard) ; self }
    pub fn   cursor(mut self,   cursor: impl Fn    (&mut T, f64,         f64 )         + 'static) -> Self { self.cursor   =      Box::new(cursor  ) ; self }
    pub fn    hover(mut self,    hover: impl Fn    (&mut T, bool             )         + 'static) -> Self { self.hover    =      Box::new(hover   ) ; self }
    pub fn   scroll(mut self,   scroll: impl Fn    (&mut T, f32,         f32 )         + 'static) -> Self { self.scroll   =      Box::new(scroll  ) ; self }
    pub fn    mouse(mut self,    mouse: impl Fn    (&mut T, MouseButton, bool)         + 'static) -> Self { self.mouse    =      Box::new(mouse   ) ; self }
    pub fn   update(mut self,   update: impl Fn    (&mut T, f32              )         + 'static) -> Self { self.update   =      Box::new(update  ) ; self }
    pub fn     draw(mut self,     draw: impl Fn    (&mut T, &mut Canvas<'_>  )         + 'static) -> Self { self.draw     =      Box::new(draw    ) ; self }

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
            return;
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
            return;
        };

        let window = surface.window().clone();
        self.state = AppState::Suspended { window };
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        let AppState::Running { surface } = &mut self.state else {
            return;
        };

        (self.update)(&mut self.data, self.timer.elapsed().as_secs_f32());
        self.timer = Instant::now();

        if (self.quit_if)(&mut self.data) {
            event_loop.exit();
        } else {
            surface.window().request_redraw();
        }
    }

    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(quit) = self.quit.take() {
            quit(&mut self.data);
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id:  WindowId,
        event:      WindowEvent,
    ) {
        let AppState::Running { surface } = &mut self.state else {
            return;
        };

        if surface.window().id() != window_id {
            return;
        }

        match event {
            WindowEvent::Resized(size) => {
                if let (Some(width), Some(height)) = (NonZeroU32::new(size.width), NonZeroU32::new(size.height)) {
                    surface.resize(width, height).unwrap();
                    (self.resize)(&mut self.data, size.width, size.height);
                }
            },
            WindowEvent::Focused(value) => {
                (self.focus)(&mut self.data, value);
            },
            WindowEvent::KeyboardInput { event, .. } => {
                if let PhysicalKey::Code(key) = event.physical_key {
                    (self.keyboard)(&mut self.data, key, event.state.is_pressed());
                }
            },
            WindowEvent::CursorMoved { position, .. } => {
                (self.cursor)(&mut self.data, position.x, position.y);
            },
            WindowEvent::CursorEntered { .. } => {
                (self.hover)(&mut self.data, true);
            },
            WindowEvent::CursorLeft { .. } => {
                (self.hover)(&mut self.data, false);
            },
            WindowEvent::MouseWheel { delta, .. } => {
                if let MouseScrollDelta::LineDelta(x, y) = delta {
                    (self.scroll)(&mut self.data, x, y);
                }
            },
            WindowEvent::MouseInput { button, state, .. } => {
                (self.mouse)(&mut self.data, button, state.is_pressed());
            },
            WindowEvent::RedrawRequested => {
                let mut buffer = surface.buffer_mut().unwrap();
                let     width  = buffer. width().get();
                let     height = buffer.height().get();
                let mut canvas = Canvas { framebuffer: &mut (*buffer), width, height };

                (self.draw)(&mut self.data, &mut canvas);

                buffer.present().unwrap();
            },
            WindowEvent::CloseRequested => {
                event_loop.exit();
            },
            _ => ()
        }
    }
}

#[derive(Clone, Copy)]
pub enum Mode {
    Fill,
    Line
}

static mut CURRENT_COLOR: u32 = 0x00FFFFFF;

pub struct Canvas<'a> {
    framebuffer: &'a mut [u32],
    width:       u32,
    height:      u32
}

// TODO: y bounds should be fine, x not yet since its 1D
impl Canvas<'_> {
    pub fn set_color(&self, red: u8, green: u8, blue: u8) {
        unsafe { CURRENT_COLOR = u32::from_ne_bytes([blue, green, red, 0]); }
    }

    pub fn point(&mut self, x: f32, y: f32) {
        if x < 0.0 || y < 0.0 || x > self.width as f32 || y > self.height as f32 {
            return;
        }

        self.framebuffer[(x + y * self.width as f32) as usize] = unsafe { CURRENT_COLOR };
    }

    pub fn rectangle(&mut self, mode: Mode, x: f32, y: f32, width: f32, height: f32) {
        let x      =      x as u32;
        let y      =      y as u32;
        let width  =  width as u32;
        let height = height as u32;

        for row in y.clamp(0, self.height)..(y+height).clamp(0, self.height) {
            let y_offset = row * self.width;
            let start    = y_offset + x;
            let end      = start    + width;

            match mode {
                Mode::Fill => {
                    self.framebuffer[start as usize..end as usize].fill(unsafe { CURRENT_COLOR });
                },
                Mode::Line => {
                    if row == y || row == y+height-1 {
                        self.framebuffer[start as usize..end as usize].fill(unsafe { CURRENT_COLOR });
                    } else {
                        self.framebuffer[ start  as usize] = unsafe { CURRENT_COLOR };
                        self.framebuffer[(end-1) as usize] = unsafe { CURRENT_COLOR };
                    }
                }
            }
        }
    }

    pub fn circle(&mut self, mode: Mode, x: f32, y: f32, radius: f32) {
        let x      =      x as u32;
        let y      =      y as u32;
        let radius = radius as u32;

        for row in y.saturating_sub(radius).min(self.height)..(y+radius).clamp(0, self.height) {
            let y_offset = row * self.width;
            let dist     = (row as i64 - y as i64).abs() as u32;
            let scale    = (radius*radius - dist*dist).isqrt();
            let start    = y_offset + x - scale;
            let end      = y_offset + x + scale;

            match mode {
                Mode::Fill => {
                    self.framebuffer[start as usize..end as usize].fill(unsafe { CURRENT_COLOR });
                },
                Mode::Line => {
                    if row == y-radius+1 || row == y+radius-1 {
                        self.framebuffer[start as usize..end as usize].fill(unsafe { CURRENT_COLOR });
                    } else {
                        self.framebuffer[ start  as usize] = unsafe { CURRENT_COLOR };
                        self.framebuffer[(end-1) as usize] = unsafe { CURRENT_COLOR };
                    }
                }
            }
        }
    }
}

