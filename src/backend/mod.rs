mod x11;
mod wayland;

use std::collections::HashMap;
use crate::window_info::WindowInfo;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WindowId {
    X11(u64),
    Wayland(u64),
}

pub trait WindowMonitorBackend: Send {
    fn get_active_window(&self) -> Option<WindowId>;
    fn get_window_title(&self, window: &WindowId) -> String;
    fn record_key_press(&mut self, key: String);
    fn run(&mut self);
    fn update_window_info(&mut self, windows: &mut HashMap<WindowId, WindowInfo>);
    fn is_running(&self) -> bool;
}

// 导出具体实现
#[cfg(feature = "x11")]
pub use x11::X11Backend;
#[cfg(feature = "wayland")]
pub use wayland::WaylandBackend;