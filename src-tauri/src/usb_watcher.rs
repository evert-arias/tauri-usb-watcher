use rusb::{Context, Device, UsbContext};
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct UsbWatcher {
    devices: Arc<Mutex<HashSet<String>>>,
}

impl UsbWatcher {
    pub fn new() -> Self {
        UsbWatcher {
            devices: Arc::new(Mutex::new(HashSet::new())),
        }
    }

    pub fn start(&self, callback: impl Fn(&str) + Send + 'static) {
        let devices = self.devices.clone();

        thread::spawn(move || {
            loop {
                if let Ok(context) = Context::new() {
                    if let Ok(device_list) = context.devices() {
                        let current_devices: HashSet<String> = device_list
                            .iter()
                            .filter_map(|device| Self::get_device_info(&device))
                            .collect();

                        let mut stored_devices = devices.lock().unwrap();

                        // Check for new devices
                        for device in current_devices.difference(&stored_devices) {
                            callback(&format!("USB device connected: {}", device));
                        }

                        // Check for removed devices
                        for device in stored_devices.difference(&current_devices) {
                            callback(&format!("USB device disconnected: {}", device));
                        }

                        *stored_devices = current_devices;
                    }
                }
                thread::sleep(Duration::from_secs(1));
            }
        });
    }

    fn get_device_info(device: &Device<Context>) -> Option<String> {
        if let Ok(desc) = device.device_descriptor() {
            Some(format!(
                "ID {:04x}:{:04x}",
                desc.vendor_id(),
                desc.product_id()
            ))
        } else {
            None
        }
    }
}
