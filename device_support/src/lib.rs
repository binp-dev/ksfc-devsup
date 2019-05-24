use std::str::from_utf8;
use std::sync::Mutex;
use std::collections::hash_map::{HashMap, Entry};
use std::time::{Duration};

use log::{info, error};
use simple_logger;
use lazy_static::lazy_static;

use epics::{
    bind_device_support,
    register_command,
    record::*,
    context::*,
};

use ksfc_lxi::{Fc};


lazy_static! {
    static ref FC: Mutex<HashMap<String, Fc>> = Mutex::new(HashMap::new());
}

fn with_device_mut<T, E, F>(rec_name: &str, f: F) -> Result<T, Option<E>>
where F: FnOnce(&mut Fc, &str) -> Result<T, E> {
    let mut it = rec_name.rsplitn(2, ':');
    let rec = it.next().unwrap();
    match it.next() {
        Some(pref) => {
            match FC.lock().unwrap().get_mut(pref) {
                Some(fc) => f(fc, rec).map_err(|e| Some(e)),
                None => {
                    error!("no such device: {}", pref);
                    Err(None)
                }
            }
        },
        None => {
            error!("wrong record name: {}", rec_name);
            Err(None)
        },
    }
}

fn init(context: &mut InitContext) {
    info!("init");
    simple_logger::init().unwrap();
    register_command!(context, fn connectDevice(addr: &str, prefix: &str) {
        info!("connectDevice(addr={}, prefix={})", addr, prefix);
        match Fc::new(&"10.0.0.9", None, Duration::from_secs(10)) {
            Ok(fc) => match FC.lock().unwrap().entry(String::from(prefix)) {
                Entry::Occupied(_) => error!("device already exists"),
                Entry::Vacant(v) => {
                    v.insert(fc);
                    info!("device connected");
                },
            },
            Err(e) => error!("cannot connect: {:?}", e),
        }
    });
}
fn quit(_context: &mut QuitContext) {
    info!("quit");
}
fn record_init(_context: &mut RecInitContext, record: &mut AnyRecord) {
    info!("record_init {}", from_utf8(record.name()).unwrap());
}
fn record_set_scan(_context: &mut RecScanContext, record: &mut RecordBase, _scan: Scan) {
    info!("record_set_scan {}", from_utf8(record.name()).unwrap());
}
fn record_read(context: &mut RecRdContext, record: &mut ReadRecord) {
    info!("record_read {}", from_utf8(record.name()).unwrap());
    context.request_async(record);
}
fn record_write(context: &mut RecWrContext, record: &mut WriteRecord) {
    info!("record_write {}", from_utf8(record.name()).unwrap());
    context.request_async(record);
}
fn record_read_async(_context: &mut RecRdAContext, record: &mut ReadRecord) {
    let name = String::from(from_utf8(record.name()).unwrap());
    info!("record_read_async {}", name);
    match with_device_mut(name.as_str(), |fc, rec| {
        match rec {
            "IDN" => match fc.api().idn() {
                Ok(idn) => {
                    match record {
                        ReadRecord::Stringin(record) => {
                            record.set_val(&idn);
                            Ok(())
                        },
                        _ => {
                            error!("bad record type");
                            Err(())
                        },
                    }
                },
                Err(e) => {
                    error!("lxi request failed: {:?}", e);
                    Err(())
                }
            },
            _ => {
                error!("unknown record suffix: {}", rec);
                Err(())
            },
        }
    }) {
        Ok(()) => (),
        Err(_) => error!("record_read_async failed"),
    }
}
fn record_write_async(_context: &mut RecWrAContext, record: &mut WriteRecord) {
    info!("record_write_async {}", from_utf8(record.name()).unwrap());
}

bind_device_support!(
    init,
    quit,
    record_init,
    record_set_scan,
    record_read,
    record_write,
    record_read_async,
    record_write_async,
);
