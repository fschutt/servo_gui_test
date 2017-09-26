use servo::compositing::windowing::WindowMethods;
use std::rc::Rc;
use servo::compositing::compositor_thread::EventLoopWaker;
use servo::euclid::{Point2D, ScaleFactor, Size2D, TypedPoint2D, TypedRect, TypedSize2D};
use servo::ipc_channel::ipc;
use servo::net_traits::net_error_list::NetError;
use servo::script_traits::LoadData;
use servo::servo_geometry::DeviceIndependentPixel;
use servo::servo_url::ServoUrl;
use servo::style_traits::DevicePixel;
use servo::style_traits::cursor::Cursor;
use servo::msg::constellation_msg::{Key, KeyModifiers};
use servo::BrowserId;

use glutin;
use servo::gl;
use glutin::GlContext;

/// The "AppWindow" is the main browser object
pub struct AppWindow {
    pub glutin_window: glutin::GlWindow,
    pub waker: Box<EventLoopWaker>,
    pub gl: Rc<gl::Gl>,
}

/// Configure servo resources
pub fn configure_renderer_startup() {
    use servo::servo_config::resource_files::set_resources_path;
    use servo::servo_config::opts;
    use std::env;

    let path = env::current_dir().unwrap().join("resources");
    let path = path.to_str().unwrap().to_string();
    set_resources_path(Some(path));

    let mut opts = opts::default_opts();
    opts::set_defaults(opts);
}

impl WindowMethods for AppWindow {
    fn prepare_for_composite(&self, _width: usize, _height: usize) -> bool {
        true
    }

    fn present(&self) {
        self.glutin_window.swap_buffers().unwrap();
    }

    fn supports_clipboard(&self) -> bool {
        false
    }

    fn create_event_loop_waker(&self) -> Box<EventLoopWaker> {
        self.waker.clone()
    }

    fn gl(&self) -> Rc<gl::Gl> {
        self.gl.clone()
    }

    fn hidpi_factor(&self) -> ScaleFactor<f32, DeviceIndependentPixel, DevicePixel> {
        ScaleFactor::new(self.glutin_window.hidpi_factor())
    }

    fn framebuffer_size(&self) -> TypedSize2D<u32, DevicePixel> {
        let (width, height) = self.glutin_window.get_inner_size().unwrap();
        let scale_factor = self.glutin_window.hidpi_factor() as u32;
        TypedSize2D::new(scale_factor * width, scale_factor * height)
    }

    fn window_rect(&self) -> TypedRect<u32, DevicePixel> {
        TypedRect::new(TypedPoint2D::new(0, 0), self.framebuffer_size())
    }

    fn size(&self) -> TypedSize2D<f32, DeviceIndependentPixel> {
        let (width, height) = self.glutin_window.get_inner_size().unwrap();
        TypedSize2D::new(width as f32, height as f32)
    }

    fn client_window(&self, _id: BrowserId) -> (Size2D<u32>, Point2D<i32>) {
        let (width, height) = self.glutin_window.get_inner_size().unwrap();
        let (x, y) = self.glutin_window.get_position().unwrap();
        (Size2D::new(width, height), Point2D::new(x as i32, y as i32))
    }

    fn set_page_title(&self, _id: BrowserId, title: Option<String>) {
        self.glutin_window.set_title(match title {
            Some(ref title) => title,
            None => "",
        });
    }

    fn allow_navigation(&self, _id: BrowserId, _url: ServoUrl, chan: ipc::IpcSender<bool>) {
        chan.send(true).ok();
    }

    fn set_inner_size(&self, _id: BrowserId, _size: Size2D<u32>) {
    }

    fn set_position(&self, _id: BrowserId, _point: Point2D<i32>) {
    }

    fn set_fullscreen_state(&self, _id: BrowserId, _state: bool) {
    }

    fn status(&self, _id: BrowserId, _status: Option<String>) {
    }

    fn load_start(&self, _id: BrowserId) {
    }

    fn load_end(&self, _id: BrowserId) {
    }

    fn load_error(&self, _id: BrowserId, _: NetError, _url: String) {
    }

    fn head_parsed(&self, _id: BrowserId) {
    }

    fn history_changed(&self, _id: BrowserId, _entries: Vec<LoadData>, _current: usize) {
    }

    fn set_cursor(&self, cursor: Cursor) {
      let cursor = match cursor {
        Cursor::Pointer => glutin::MouseCursor::Hand,
        _ => glutin::MouseCursor::Default,
      };
      self.glutin_window.set_cursor(cursor);
    }

    fn set_favicon(&self, _id: BrowserId, _url: ServoUrl) {
    }

    fn handle_key(&self, _id: Option<BrowserId>, _ch: Option<char>, _key: Key, _mods: KeyModifiers) {
    }
}
