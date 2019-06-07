use log::{info};

use epics::record::*;

use ksfc_lxi::{
    types::{ChannelNo},
    format::{secs_as_dur},
};

use crate::{DEVICES, device::DeviceHandle};


pub trait Dev {
    fn get_dev(args: &[&str]) -> epics::Result<DeviceHandle> {
        args.get(0).ok_or("too few args".into())
        .and_then(|id| {
            match DEVICES.lock().unwrap().get_mut(*id) {
                Some(dev) => Ok(dev.handle()),
                None => Err(format!("no such device: {}", id).into()),
            }
        })
    }
}

pub trait Chan: Dev {
    fn get_chan(args: &[&str]) -> epics::Result<ChannelNo> {
        args.get(1).ok_or("too few args".into())
        .and_then(|s| s.parse::<i32>().map_err(|e| format!("{}", e).into()))
        .and_then(|no| match no {
            1 => Ok(ChannelNo::Ch1),
            2 => Ok(ChannelNo::Ch2),
            _ => Err(format!("invalid channel {}", no).into()),
        })
    }
}

/// `IDN`
pub struct Idn {
    dev: DeviceHandle,
}
impl Idn {
    pub fn new(dev: DeviceHandle) -> Self {
        Self { dev }
    }
}
impl Dev for Idn {}
impl Handler<StringinRecord> for Idn {}
impl InitHandler<StringinRecord> for Idn {
    fn init(_record: &mut StringinRecord, args: &[&str]) -> epics::Result<Self> {
        Self::get_dev(args)
        .and_then(|dev| Ok(Self::new(dev)))
    }
}
impl ScanHandler<StringinRecord> for Idn {
    fn set_scan(&mut self, _record: &mut StringinRecord, scan: Scan) -> epics::Result<()> {
        self.dev.idn_set_scan(scan);
        Ok(())
    }
}
impl ReadHandler<StringinRecord> for Idn {
    fn read(&mut self, _record: &mut StringinRecord) -> epics::Result<bool> {
        Ok(false)
    }
    fn read_async(&mut self, record: &mut StringinRecord) -> epics::Result<()> {
        self.dev.idn_get()
        .and_then(|idn| {
            info!("IDN.read: '{}'", idn);
            record.set_val(idn.as_bytes());
            Ok(())
        })
    }
}
impl StringinHandler for Idn {}


/// `CHAN_<X>`
pub struct ChanAct {
    dev: DeviceHandle,
    chan: ChannelNo,
}
impl ChanAct {
    pub fn new(dev: DeviceHandle, chan: ChannelNo) -> Self {
        Self { dev, chan }
    }
}
impl Dev for ChanAct {}
impl Chan for ChanAct {}
impl Handler<LongoutRecord> for ChanAct {}
impl InitHandler<LongoutRecord> for ChanAct {
    fn init(_record: &mut LongoutRecord, args: &[&str]) -> epics::Result<Self> {
        Self::get_dev(args)
        .and_then(|dev| Self::get_chan(args).map(|chan| (dev, chan)))
        .and_then(|(dev, chan)| Ok(Self::new(dev, chan)))
    }
}
impl ScanHandler<LongoutRecord> for ChanAct {
    fn set_scan(&mut self, _record: &mut LongoutRecord, _scan: Scan) -> epics::Result<()> {
        unimplemented!()
    }
}
impl WriteHandler<LongoutRecord> for ChanAct {
    fn write(&mut self, record: &mut LongoutRecord) -> epics::Result<bool> {
        info!("CHAN_{}.write: {}", self.chan as u8, record.val());
        self.dev.chan_activate(self.chan, record.val() != 0);
        Ok(true)
    }
    fn write_async(&mut self, _record: &mut LongoutRecord) -> epics::Result<()> {
        unimplemented!()
    }
}
impl LongoutHandler for ChanAct {}


/// `FREQ_<X>`
pub struct ChanFreq {
    dev: DeviceHandle,
    chan: ChannelNo,
}
impl ChanFreq {
    pub fn new(dev: DeviceHandle, chan: ChannelNo) -> Self {
        Self { dev, chan }
    }
}
impl Dev for ChanFreq {}
impl Chan for ChanFreq {}
impl Handler<AiRecord> for ChanFreq {}
impl InitHandler<AiRecord> for ChanFreq {
    fn init(_record: &mut AiRecord, args: &[&str]) -> epics::Result<Self> {
        Self::get_dev(args)
        .and_then(|dev| Self::get_chan(args).map(|chan| (dev, chan)))
        .and_then(|(dev, chan)| Ok(Self::new(dev, chan)))
    }
}
impl ScanHandler<AiRecord> for ChanFreq {
    fn set_scan(&mut self, _record: &mut AiRecord, scan: Scan) -> epics::Result<()> {
        self.dev.chan_freq_set_scan(self.chan, scan);
        Ok(())
    }
}
impl ReadHandler<AiRecord> for ChanFreq {
    fn read(&mut self, record: &mut AiRecord) -> epics::Result<bool> {
        self.dev.chan_freq_get(self.chan)
        .and_then(|freq| {
            info!("FREQ_{}.read: {}", self.chan as u8, freq);
            record.set_val(freq);
            Ok(true)
        })
    }
    fn read_async(&mut self, _record: &mut AiRecord) -> epics::Result<()> {
        unimplemented!()
    }
}
impl AiHandler for ChanFreq {}


/// `GATE_TIME_<X>`
pub struct ChanGateTime {
    dev: DeviceHandle,
    chan: ChannelNo,
}
impl ChanGateTime {
    pub fn new(dev: DeviceHandle, chan: ChannelNo) -> Self {
        Self { dev, chan }
    }
}
impl Dev for ChanGateTime {}
impl Chan for ChanGateTime {}
impl Handler<AoRecord> for ChanGateTime {}
impl InitHandler<AoRecord> for ChanGateTime {
    fn init(_record: &mut AoRecord, args: &[&str]) -> epics::Result<Self> {
        Self::get_dev(args)
        .and_then(|dev| Self::get_chan(args).map(|chan| (dev, chan)))
        .and_then(|(dev, chan)| Ok(Self::new(dev, chan)))
    }
}
impl ScanHandler<AoRecord> for ChanGateTime {
    fn set_scan(&mut self, _record: &mut AoRecord, _scan: Scan) -> epics::Result<()> {
        unimplemented!()
    }
}
impl WriteHandler<AoRecord> for ChanGateTime {
    fn write(&mut self, record: &mut AoRecord) -> epics::Result<bool> {
        let val = record.val();
        match secs_as_dur(val) {
            Some(dur) => {
                info!("GATE_TIME_{}.write: {}", self.chan as u8, val);
                self.dev.chan_gate_time_set(self.chan, dur);
                Ok(())
            },
            None => Err(format!(
                "GATE_TIME_{}.write: invalid gate time value: {}",
                self.chan as u8, val,
            ).into()),
        }.map(|()| true)
    }
    fn write_async(&mut self, _record: &mut AoRecord) -> epics::Result<()> {
        unimplemented!()
    }
}
impl AoHandler for ChanGateTime {}


/// `MEASURE`
pub struct Measure {
    dev: DeviceHandle,
}
impl Measure {
    pub fn new(dev: DeviceHandle) -> Self {
        Self { dev }
    }
}
impl Dev for Measure {}
impl Handler<LongoutRecord> for Measure {}
impl InitHandler<LongoutRecord> for Measure {
    fn init(_record: &mut LongoutRecord, args: &[&str]) -> epics::Result<Self> {
        Self::get_dev(args)
        .and_then(|dev| Ok(Self::new(dev)))
    }
}
impl ScanHandler<LongoutRecord> for Measure {
    fn set_scan(&mut self, _record: &mut LongoutRecord, _scan: Scan) -> epics::Result<()> {
        unimplemented!()
    }
}
impl WriteHandler<LongoutRecord> for Measure {
    fn write(&mut self, record: &mut LongoutRecord) -> epics::Result<bool> {
        info!("MEASURE.write: {}", record.val());
        self.dev.measure(record.val() != 0);
        Ok(true)
    }
    fn write_async(&mut self, _record: &mut LongoutRecord) -> epics::Result<()> {
        unimplemented!()
    }
}
impl LongoutHandler for Measure {}
