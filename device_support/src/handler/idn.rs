use epics::record::*;

use crate::device::DeviceHandle;


pub struct IdnHandler {
    dev: DeviceHandle,
}
impl IdnHandler {
    pub fn new(dev: DeviceHandle) -> Self {
        Self { dev }
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
            record.set_val(idn.as_bytes());
            Ok(())
        })
    }
}
impl StringinHandler for IdnHandler {}
