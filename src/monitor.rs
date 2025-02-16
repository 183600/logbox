use crate::backend::{WindowId, WindowMonitorBackend};
use crate::window_info::WindowInfo;
use std::collections::HashMap;

pub struct WindowMonitor {
    backend: Box<dyn WindowMonitorBackend>,
    window_map: HashMap<WindowId, WindowInfo>,
}

impl WindowMonitor {
    pub fn new() -> Self {
        let backend = detect_backend()
            .expect("Failed to initialize any window system backend");

        WindowMonitor {
            backend,
            window_map: HashMap::new(),
        }
    }

    // 修改原有方法调用backend的接口...
    pub fn update_window_info(&mut self) {
        self.backend.update_window_info(&mut self.window_map);
    }
}

fn detect_backend() -> Option<Box<dyn WindowMonitorBackend>> {
    #[cfg(feature = "wayland")]
    if wayland_detected() {
        return WaylandBackend::new().map(|b| Box::new(b) as _);
    }

    #[cfg(feature = "x11")]
    if x11_detected() {
        return X11Backend::new().map(|b| Box::new(b) as _);
    }

    None
}

// 环境检测函数
fn wayland_detected() -> bool {
    std::env::var("WAYLAND_DISPLAY").is_ok()
}

fn x11_detected() -> bool {
    std::env::var("DISPLAY").is_ok()
}