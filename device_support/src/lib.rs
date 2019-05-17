use std::io;
use std::thread;
use std::time::Duration;

use epics_binding::{
    bind_device_support, 
    DeviceSupport,
    record::*,
};

struct MyDevSup {
    jh: Option<thread::JoinHandle<()>>,
}

impl MyDevSup {
    fn new() -> Self {
        println!("[devsup] new");
        Self { jh: None }
    }
}

impl DeviceSupport for MyDevSup {
    fn init(&mut self, record: &mut AnyRecord) -> io::Result<()> {
        println!("[devsup] init {}", record.name());
        Ok(())
    }
    fn read(&mut self, record: &mut ReadRecord) -> io::Result<()> {
        println!("[devsup] read {}", record.name());
        thread::sleep(Duration::from_millis(500));
        Ok(())
    }
    fn write(&mut self, record: &mut WriteRecord) -> io::Result<()> {
        println!("[devsup] write {}", record.name());
        thread::sleep(Duration::from_millis(500));
        Ok(())
    }
    fn set_scan(&mut self, record: &mut Record, scan: Scan) -> io::Result<()> {
        println!("[devsup] set_scan {}", record.name());
        if self.jh.is_none() {
            self.jh = Some(thread::spawn(move || {
                thread::sleep(Duration::from_millis(2000));
                for _ in 0..3 {
                    thread::sleep(Duration::from_millis(1000));
                    scan.request().unwrap();
                }
            }));
        }
        Ok(())
    }
}

impl Drop for MyDevSup {
    fn drop(&mut self) {
        match self.jh.take() {
            Some(jh) => jh.join().unwrap(),
            None => (),
        }
    }
}

bind_device_support!(MyDevSup::new);
