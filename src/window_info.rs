use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

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