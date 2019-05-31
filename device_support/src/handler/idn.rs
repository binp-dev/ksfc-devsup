use epics::record::*;

use crate::{with_device};

pub struct IdnHandler {
    dev: String,
}
impl IdnHandler {
    pub fn new(dev: &str) -> Self {
        Self { dev: dev.into() }
    }
}

impl ScanHandler<StringinRecord> for IdnHandler {
    fn set_scan(&mut self, _record: &mut StringinRecord, _scan: Scan) -> epics::Result<()> {
        unimplemented!();
    }
}
impl ReadHandler<StringinRecord> for IdnHandler {
    fn read(&mut self, _record: &mut StringinRecord) -> epics::Result<bool> {
        Ok(false)
    }
    fn read_async(&mut self, record: &mut StringinRecord) -> epics::Result<()> {
        with_device(&self.dev, |fc| {
            fc.api().idn().map_err(|e| e.into())
        }).and_then(|idn| {
            record.set_val(&idn);
            Ok(())
        })
    }
}
impl StringinHandler for IdnHandler {}
