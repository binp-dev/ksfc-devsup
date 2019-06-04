mod driver;

use std::time::{Duration};
use std::thread::{self, JoinHandle};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Sender};

use epics::{Scan};

use ksfc_lxi::{KsFc};

use driver::{Driver, Cmd as DrvCmd, Data as DrvData};

#[derive(Clone)]
pub struct DeviceHandle {
    data: Arc<Mutex<DrvData>>,
    chan: Sender<DrvCmd>,
}

pub struct Device {
    handle: DeviceHandle,
    driver: Option<Driver>,
    thread: Option<JoinHandle<()>>,
}

impl Device {
    pub fn new(addr: &str, port: Option<u16>) -> Self {
        let dev = KsFc::new(addr, port, Duration::from_secs(10));

        let (tx, rx) = channel();

        let data = Arc::new(Mutex::new(DrvData::new()));

        let handle = DeviceHandle { data: data.clone(), chan: tx };

        let driver = Driver::new(dev, data, rx);

        Self { handle, driver: Some(driver), thread: None }
    }

    pub fn start(&mut self) -> epics::Result<()> {
        match self.driver.take() {
            Some(driver) => Ok(thread::spawn(move || driver.start())),
            None => Err("device already started".into()),
        }.and_then(|thread| {
            self.thread = Some(thread);
            Ok(())
        })
    }

    pub fn handle(&self) -> DeviceHandle {
        self.handle.clone()
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        self.handle.chan.send(DrvCmd::Stop).unwrap();
    }
}

impl DeviceHandle {
    pub fn idn_set_scan(&self, scan: Scan) {
        self.chan.send(DrvCmd::IdnSetScan(scan)).unwrap();
    }
    pub fn idn_get(&self) -> epics::Result<String> {
        match self.data.lock().unwrap().idn {
            Some(ref idn) => Ok(idn.clone()),
            None => Err("device is not available".into()),
        }
    }
}
