use std::cell::RefCell;
use std::rc::Rc;

use crate::engine::Engine;
use anyhow::Result;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::{Window as WinitWindow, WindowBuilder};
pub trait WindowTrait {
    type WindowRunner;

    fn new(title: &str) -> Result<(Self, Self::WindowRunner)>
    where
        Self: Sized;
    fn set_title(&mut self, title: &str);
}

pub trait WindowRunner {
    fn run<I, U, R, Q>(
        self,
        engine: Engine,
        init_fn: I,
        update_fn: U,
        render_fn: R,
        quit_fn: Q,
    ) -> !
    where
        I: 'static + FnMut(&mut Engine) -> Result<()>,
        U: 'static + FnMut(&mut Engine) -> Result<()>,
        R: 'static + FnMut(&mut Engine) -> Result<()>,
        Q: 'static + FnMut(&mut Engine) -> Result<()>;
}

pub struct Window {
    pub(crate) state: SharedWindowState,
}

pub struct WindowExecutor {
    event_loop: EventLoop<()>,
    state: SharedWindowState,
}

impl WindowRunner for WindowExecutor {
    fn run<I, U, R, Q>(
        self,
        mut engine: Engine,
        mut init_fn: I,
        mut update_fn: U,
        mut render_fn: R,
        mut quit_fn: Q,
    ) -> !
    where
        I: 'static + FnMut(&mut Engine) -> Result<()>,
        U: 'static + FnMut(&mut Engine) -> Result<()>,
        R: 'static + FnMut(&mut Engine) -> Result<()>,
        Q: 'static + FnMut(&mut Engine) -> Result<()>,
    {
        init_fn(&mut engine).unwrap();
        self.event_loop.run(move |event, _, control_flow| {
            control_flow.set_poll();
            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    quit_fn(&mut engine).unwrap();
                    control_flow.set_exit();
                }
                Event::MainEventsCleared => {
                    // Update code goes here
                    update_fn(&mut engine).unwrap();
                    render_fn(&mut engine).unwrap();
                    // We can continuously render here as well
                    self.state.window.request_redraw()
                }
                event => self
                    .state
                    .events
                    .borrow_mut()
                    .push(event.to_static().unwrap()),
            }
        });
    }
}

impl WindowTrait for Window {
    type WindowRunner = WindowExecutor;

    fn new(title: &str) -> Result<(Self, Self::WindowRunner)>
    where
        Self: Sized,
    {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().with_title(title).build(&event_loop)?;
        let state = SharedWindowState {
            window: Rc::new(window),
            events: Rc::new(RefCell::new(Vec::new())),
        };

        Ok((
            Self {
                state: state.clone(),
            },
            Self::WindowRunner { event_loop, state },
        ))
    }
    fn set_title(&mut self, title: &str) {
        self.state.window.set_title(title);
    }
}

#[derive(Clone)]
pub(crate) struct SharedWindowState {
    window: Rc<WinitWindow>,
    events: Rc<RefCell<Vec<Event<'static, ()>>>>,
}
