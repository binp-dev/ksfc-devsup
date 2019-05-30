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
    fn set_scan(&mut self, _record: &mut StringinRecord, _scan: Scan) {
        unimplemented!();
    }
}
impl ReadHandler<StringinRecord> for IdnHandler {
    fn read(&mut self, _record: &mut StringinRecord) -> bool {
        false
    }
    fn read_async(&mut self, record: &mut StringinRecord) {
        let idn = with_device(&self.dev, |fc| {
            fc.api().idn()
        }).unwrap();
        record.set_val(&idn);
    }
}
impl StringinHandler for IdnHandler {}
