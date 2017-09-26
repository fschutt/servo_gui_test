use glutin;
use std::sync::Arc;
use servo::compositing::compositor_thread::EventLoopWaker;

pub struct GlutinEventLoopWaker {
    pub proxy: Arc<glutin::EventsLoopProxy>
}

impl EventLoopWaker for GlutinEventLoopWaker {
    // Use by servo to share the "event loop waker" across threads
    fn clone(&self) -> Box<EventLoopWaker + Send> {
        Box::new(GlutinEventLoopWaker { proxy: self.proxy.clone() })
    }
    // Called by servo when the main thread needs to wake up
    fn wake(&self) {
        self.proxy.wakeup().expect("wakeup eventloop failed");
    }
}
