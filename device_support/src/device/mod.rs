mod driver;

use std::time::{Duration};
use std::thread::{self, JoinHandle};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Sender};

use epics::{Scan};

use ksfc_lxi::{
    KsFc,
    types::{ChannelNo},
};

use driver::{
    Driver,
    Cmd as DrvCmd, ChanCmd,
    Data as DrvData,
};

#[derive(Clone)]
pub struct DeviceHandle {
    data: Arc<Mutex<DrvData>>,
    tx:   Sender<DrvCmd>,
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

        let handle = DeviceHandle { data: data.clone(), tx };

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
        self.handle.tx.send(DrvCmd::Stop).unwrap();
    }
}

impl DeviceHandle {
    pub fn idn_set_scan(&self, scan: Scan) {
        self.tx.send(DrvCmd::IdnSetScan(scan)).unwrap();
    }
    pub fn idn_get(&self) -> epics::Result<String> {
        Ok(self.data.lock().unwrap().idn.clone())
    }
    pub fn chan_activate(&self, chan: ChannelNo, act: bool) {
        self.tx.send(DrvCmd::Chan(chan, ChanCmd::Activate(act))).unwrap();
    }
    pub fn chan_freq_set_scan(&self, chan: ChannelNo, scan: Scan) {
        self.tx.send(DrvCmd::Chan(chan, ChanCmd::SetScan(scan))).unwrap();
    }
    pub fn chan_freq_get(&self, chan: ChannelNo) -> epics::Result<f64> {
        Ok(self.data.lock().unwrap().channels[chan].freq)
    }
    pub fn chan_gate_time_set(&self, chan: ChannelNo, gate_time: Duration) {
        self.tx.send(DrvCmd::Chan(chan, ChanCmd::SetGateTime(gate_time))).unwrap();
    }
    pub fn measure(&self, meas: bool) {
        self.tx.send(DrvCmd::Measure(meas)).unwrap();
    }
}
