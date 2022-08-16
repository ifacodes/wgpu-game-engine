mod engine;
mod state;
mod vertex;
mod window;

use engine::Engine;
use window::{Window, WindowRunner, WindowTrait};

// async fn run() {
//     env_logger::init();
//     let event_loop = EventLoop::new();
//     let window = WindowBuilder::new().build(&event_loop).unwrap();
//     let mut app = App::new(&window).await.unwrap();

//     event_loop.run(move |event, _, control_flow| match event {
//         Event::WindowEvent {
//             ref event,
//             window_id,
//         } if window_id == window.id() => {
//             if app.handle_input(event).is_ok() {
//                 match event {
//                     WindowEvent::CloseRequested
//                     | WindowEvent::KeyboardInput {
//                         input:
//                             KeyboardInput {
//                                 state: ElementState::Pressed,
//                                 virtual_keycode: Some(VirtualKeyCode::Escape),
//                                 ..
//                             },
//                         ..
//                     } => *control_flow = ControlFlow::Exit,
//                     WindowEvent::Resized(physical_size) => {
//                         app.handle_window_resize(*physical_size);
//                     }
//                     WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
//                         app.handle_window_resize(**new_inner_size);
//                     }
//                     _ => {}
//                 }
//             }
//         }
//         Event::RedrawRequested(window_id) if window_id == window.id() => match app.update() {
//             Ok(_) => {}
//             Err(wgpu::SurfaceError::Lost) => app.handle_window_resize(app.window_size),
//             Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
//             Err(e) => eprintln!("{:?}", e),
//         },
//         Event::MainEventsCleared => window.request_redraw(),
//         _ => {}
//     });
// }

fn main() {
    env_logger::init();
    //pollster::block_on(run());
    let (window, exec) = Window::new("title").unwrap();
    let engine = Engine::new(window).unwrap();
    exec.run(
        engine,
        move |engine| {
            engine.counter = 100;
            Ok(())
        },
        move |engine| engine.update(),
        move |engine| engine.render(),
        move |engine| Ok(()),
    );
}
