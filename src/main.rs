#![feature(alloc_system)]
extern crate alloc_system;

extern crate servo;
extern crate glutin;

pub mod events;
pub mod renderer;

use std::sync::Arc;
use events::GlutinEventLoopWaker;
use glutin::GlContext;
use renderer::AppWindow;
use std::rc::Rc;

use servo::gl;
use servo::script_traits::TouchEventType;
use servo::euclid::{TypedPoint2D, TypedVector2D};
use servo::servo_url::ServoUrl;
use servo::compositing::windowing::WindowEvent;
use servo::ipc_channel::ipc;

fn main() {
    println!("Servo version: {}", servo::config::servo_version());

    let mut event_loop = glutin::EventsLoop::new();

    let event_loop_waker = Box::new(GlutinEventLoopWaker {
        proxy: Arc::new(event_loop.create_proxy())
    });

    let builder = glutin::WindowBuilder::new().with_dimensions(800, 600);
    let gl_version = glutin::GlRequest::Specific(glutin::Api::OpenGl, (3, 2));
    let context = glutin::ContextBuilder::new().with_gl(gl_version).with_vsync(true);
    let window = glutin::GlWindow::new(builder, context, &event_loop).unwrap();

    let gl = unsafe {
        window.context().make_current().expect("Couldn't make window current");
        gl::GlFns::load_with(|s| window.context().get_proc_address(s) as *const _)
    };

    renderer::configure_renderer_startup();

    // window.show(); // ?????

    let window = Rc::new(AppWindow {
        glutin_window: window,
        waker: event_loop_waker,
        gl: gl,
    });

    let mut servo = servo::Servo::new(window.clone());

    // the actual browser
    let url = ServoUrl::parse("https://servo.org").unwrap();
    let (sender, receiver) = ipc::channel().unwrap();
    servo.handle_events(vec![WindowEvent::NewBrowser(url, sender)]);
    let browser_id = receiver.recv().unwrap();
    servo.handle_events(vec![WindowEvent::SelectBrowser(browser_id)]);

    let mut pointer = (0.0, 0.0);

    event_loop.run_forever(|event| {
      // Blocked until user event or until servo unblocks it
      match event {
        glutin::Event::WindowEvent {event: glutin::WindowEvent::MouseMoved{position: (x, y), ..} , ..} => {
          pointer = (x, y);
          let event = WindowEvent::MouseWindowMoveEventClass(TypedPoint2D::new(x as f32, y as f32));
          servo.handle_events(vec![event]);
        },
        glutin::Event::WindowEvent {event: glutin::WindowEvent::MouseWheel{delta, phase, ..} , ..} => {
          let (dx, dy) = match delta {
            glutin::MouseScrollDelta::LineDelta(dx, dy) => (dx, dy * 38.0 /*line height*/),
            glutin::MouseScrollDelta::PixelDelta(dx, dy) => (dx, dy),
          };
          let scroll_location = servo::webrender_api::ScrollLocation::Delta(TypedVector2D::new(dx, dy));
          let phase = match phase {
            glutin::TouchPhase::Started => TouchEventType::Down,
            glutin::TouchPhase::Moved => TouchEventType::Move,
            glutin::TouchPhase::Ended => TouchEventType::Up,
            glutin::TouchPhase::Cancelled => TouchEventType::Up,
          };
          let pointer = TypedPoint2D::new(pointer.0 as i32, pointer.1 as i32);
          let event = WindowEvent::Scroll(scroll_location, pointer, phase);
          servo.handle_events(vec![event]);
        },
        glutin::Event::WindowEvent {
          event: glutin::WindowEvent::KeyboardInput{ input: glutin::KeyboardInput {
            state: glutin::ElementState::Pressed,
            virtual_keycode: Some(glutin::VirtualKeyCode::R),
            ..
            }, ..}, ..} => {
              let event = WindowEvent::Reload(browser_id);
              servo.handle_events(vec![event]);
        },
        _ => {
        }
      }
      glutin::ControlFlow::Continue
    });
}
