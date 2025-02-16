use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use x11_dl::{xlib, xtest};

pub struct WindowInfo {
    pub title: String,
    pub last_open_time: u64,
    pub open_count: u32,
    pub key_press_count: HashMap<String, u32>,
}

impl WindowInfo {
    pub fn new(title: String) -> Self {
        WindowInfo {
            title,
            last_open_time: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            open_count: 1,
            key_press_count: HashMap::new(),
        }
    }

    pub fn update_open_time(&mut self) {
        self.last_open_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        self.open_count += 1;
    }

    pub fn record_key_press(&mut self, key: String) {
        *self.key_press_count.entry(key).or_insert(0) += 1;
    }
}

pub struct WindowMonitor {
    pub xlib: xlib::Xlib,
    pub display: *mut xlib::Display,
    pub root_window: xlib::Window,
    pub window_map: HashMap<xlib::Window, WindowInfo>,
}

impl WindowMonitor {
    pub fn new() -> Self {
        let xlib = xlib::Xlib::open().expect("Failed to open Xlib");
        let display = unsafe { (xlib.XOpenDisplay)(std::ptr::null()) };
        if display.is_null() {
            panic!("Failed to open display");
        }

        let root_window = unsafe { (xlib.XDefaultRootWindow)(display) };

        WindowMonitor {
            xlib,
            display,
            root_window,
            window_map: HashMap::new(),
        }
    }

    pub fn get_active_window(&self) -> Option<xlib::Window> {
        let mut root_return: xlib::Window = 0;
        let mut parent_return: xlib::Window = 0;
        let mut children_return: *mut xlib::Window = std::ptr::null_mut();
        let mut nchildren_return: u32 = 0;

        unsafe {
            (self.xlib.XQueryTree)(
                self.display,
                self.root_window,
                &mut root_return,
                &mut parent_return,
                &mut children_return,
                &mut nchildren_return,
            );
        }

        if children_return.is_null() {
            return None;
        }

        let mut active_window = None;
        for i in 0..nchildren_return {
            let child = unsafe { *children_return.offset(i as isize) };
            let mut window_title = String::new();

            let mut title: [u8; 256] = [0; 256];
            unsafe {
                (self.xlib.XFetchName)(self.display, child, title.as_mut_ptr() as *mut i8);
            }

            if let Ok(title_str) = std::str::from_utf8(&title) {
                window_title = title_str.trim_matches('\0').to_string();
            }

            if !window_title.is_empty() {
                active_window = Some(child);
                break;
            }
        }

        unsafe {
            (self.xlib.XFree)(children_return as *mut _);
        }

        active_window
    }

    pub fn update_window_info(&mut self) {
        if let Some(active_window) = self.get_active_window() {
            if let Some(window_info) = self.window_map.get_mut(&active_window) {
                window_info.update_open_time();
            } else {
                let window_title = self.get_window_title(active_window);
                self.window_map.insert(active_window, WindowInfo::new(window_title));
            }
        }
    }

    pub fn get_window_title(&self, window: xlib::Window) -> String {
        let mut title: [u8; 256] = [0; 256];
        unsafe {
            (self.xlib.XFetchName)(self.display, window, title.as_mut_ptr() as *mut i8);
        }

        if let Ok(title_str) = std::str::from_utf8(&title) {
            title_str.trim_matches('\0').to_string()
        } else {
            String::new()
        }
    }

    pub fn record_key_press(&mut self, key: String) {
        if let Some(active_window) = self.get_active_window() {
            if let Some(window_info) = self.window_map.get_mut(&active_window) {
                window_info.record_key_press(key);
            }
        }
    }

    pub fn run(&mut self) {
        unsafe {
            (self.xlib.XSelectInput)(self.display, self.root_window, xlib::SubstructureNotifyMask);
        }

        let mut event: xlib::XEvent = unsafe { std::mem::zeroed() };
        loop {
            unsafe {
                (self.xlib.XNextEvent)(self.display, &mut event);
            }

            match event.get_type() {
                xlib::MapNotify => {
                    self.update_window_info();
                }
                xlib::KeyPress => {
                    let key = self.get_key_from_event(&event);
                    self.record_key_press(key);
                }
                _ => {}
            }
        }
    }

    fn get_key_from_event(&self, event: &xlib::XEvent) -> String {
        let keycode = unsafe { event.xkey.keycode };
        let keysym = unsafe { (self.xlib.XKeycodeToKeysym)(self.display, keycode, 0) };
        let key = unsafe { (self.xlib.XKeysymToString)(keysym) };

        if !key.is_null() {
            let cstr = unsafe { std::ffi::CStr::from_ptr(key) };
            if let Ok(key_str) = cstr.to_str() {
                return key_str.to_string();
            }
        }

        String::new()
    }
}

impl Drop for WindowMonitor {
    fn drop(&mut self) {
        unsafe {
            (self.xlib.XCloseDisplay)(self.display);
        }
    }
}
