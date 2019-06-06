use log::{info};

use epics::record::*;

use ksfc_lxi::{
    types::{ChannelNo},
    format::{secs_as_dur},
};

use crate::device::DeviceHandle;


/// `IDN`
pub struct IdnHandler {
    dev: DeviceHandle,
}
impl IdnHandler {
    pub fn new(dev: DeviceHandle) -> Self {
        Self { dev }
    }
}
impl Handler<StringinRecord> for IdnHandler {
    fn into_any_box(self) -> AnyHandlerBox {
        AnyHandlerBox::Stringin(Box::new(self))
    }
}
impl ScanHandler<StringinRecord> for IdnHandler {
    fn set_scan(&mut self, _record: &mut StringinRecord, scan: Scan) -> epics::Result<()> {
        self.dev.idn_set_scan(scan);
        Ok(())
    }
}
impl ReadHandler<StringinRecord> for IdnHandler {
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
impl StringinHandler for IdnHandler {}


/// `CHAN_<X>`
pub struct ChanActHandler {
    dev: DeviceHandle,
    chan: ChannelNo,
}
impl ChanActHandler {
    pub fn new(dev: DeviceHandle, chan: ChannelNo) -> Self {
        Self { dev, chan }
    }
}
impl Handler<LongoutRecord> for ChanActHandler {
    fn into_any_box(self) -> AnyHandlerBox {
        AnyHandlerBox::Longout(Box::new(self))
    }
}
impl ScanHandler<LongoutRecord> for ChanActHandler {
    fn set_scan(&mut self, _record: &mut LongoutRecord, _scan: Scan) -> epics::Result<()> {
        unimplemented!()
    }
}
impl WriteHandler<LongoutRecord> for ChanActHandler {
    fn write(&mut self, record: &mut LongoutRecord) -> epics::Result<bool> {
        info!("CHAN_{}.write: {}", self.chan as u8, record.val());
        self.dev.chan_activate(self.chan, record.val() != 0);
        Ok(true)
    }
    fn write_async(&mut self, _record: &mut LongoutRecord) -> epics::Result<()> {
        unimplemented!()
    }
}
impl LongoutHandler for ChanActHandler {}


/// `FREQ_<X>`
pub struct ChanFreqHandler {
    dev: DeviceHandle,
    chan: ChannelNo,
}
impl ChanFreqHandler {
    pub fn new(dev: DeviceHandle, chan: ChannelNo) -> Self {
        Self { dev, chan }
    }
}
impl Handler<AiRecord> for ChanFreqHandler {
    fn into_any_box(self) -> AnyHandlerBox {
        AnyHandlerBox::Ai(Box::new(self))
    }
}
impl ScanHandler<AiRecord> for ChanFreqHandler {
    fn set_scan(&mut self, _record: &mut AiRecord, scan: Scan) -> epics::Result<()> {
        self.dev.chan_freq_set_scan(self.chan, scan);
        Ok(())
    }
}
impl ReadHandler<AiRecord> for ChanFreqHandler {
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
impl AiHandler for ChanFreqHandler {}


/// `GATE_TIME_<X>`
pub struct GateTimeHandler {
    dev: DeviceHandle,
    chan: ChannelNo,
}
impl GateTimeHandler {
    pub fn new(dev: DeviceHandle, chan: ChannelNo) -> Self {
        Self { dev, chan }
    }
}
impl Handler<AoRecord> for GateTimeHandler {
    fn into_any_box(self) -> AnyHandlerBox {
        AnyHandlerBox::Ao(Box::new(self))
    }
}
impl ScanHandler<AoRecord> for GateTimeHandler {
    fn set_scan(&mut self, _record: &mut AoRecord, _scan: Scan) -> epics::Result<()> {
        unimplemented!()
    }
}
impl WriteHandler<AoRecord> for GateTimeHandler {
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
impl AoHandler for GateTimeHandler {}


/// `MEASURE`
pub struct MeasureHandler {
    dev: DeviceHandle,
}
impl MeasureHandler {
    pub fn new(dev: DeviceHandle) -> Self {
        Self { dev }
    }
}
impl Handler<LongoutRecord> for MeasureHandler {
    fn into_any_box(self) -> AnyHandlerBox {
        AnyHandlerBox::Longout(Box::new(self))
    }
}
impl ScanHandler<LongoutRecord> for MeasureHandler {
    fn set_scan(&mut self, _record: &mut LongoutRecord, _scan: Scan) -> epics::Result<()> {
        unimplemented!()
    }
}
impl WriteHandler<LongoutRecord> for MeasureHandler {
    fn write(&mut self, record: &mut LongoutRecord) -> epics::Result<bool> {
        info!("MEASURE.write: {}", record.val());
        self.dev.measure(record.val() != 0);
        Ok(true)
    }
    fn write_async(&mut self, _record: &mut LongoutRecord) -> epics::Result<()> {
        unimplemented!()
    }
}
impl LongoutHandler for MeasureHandler {}
