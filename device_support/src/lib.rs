mod handler;

use std::str::from_utf8;
use std::sync::Mutex;
use std::collections::hash_map::{HashMap, Entry};
use std::time::{Duration};

use log::{info, error};
use simple_logger;
use lazy_static::lazy_static;


use epics::{
    self,
    bind_device_support,
    register_command,
    record::*,
    context::*,
};

use ksfc_lxi::{Fc};

use handler::*;


lazy_static! {
    static ref DEVICES: Mutex<HashMap<String, Fc>> = Mutex::new(HashMap::new());
}

fn split_name(name: &str) -> Result<(&str, &str), ()> {
    let mut it = name.rsplitn(2, ':');
    let rec = it.next().unwrap();
    match it.next() {
        Some(pref) => Ok((pref, rec)),
        None => Err(()),
    }
}

fn with_device<T, E, F>(dev: &str, f: F) -> epics::Result<T>
where F: FnOnce(&mut Fc) -> epics::Result<T> {
    match DEVICES.lock().unwrap().get_mut(dev) {
        Some(fc) => f(fc),
        None => Err(format!("no such device: {}", dev).into()),
    }
}

fn init(context: &mut Context) -> epics::Result<()> {
    simple_logger::init().unwrap();
    info!("init");
    register_command!(context, fn connectDevice(addr: &str, prefix: &str) {
        info!("connectDevice(addr={}, prefix={})", addr, prefix);
        match Fc::new(&"10.0.0.9", None, Duration::from_secs(10)) {
            Ok(fc) => match DEVICES.lock().unwrap().entry(String::from(prefix)) {
                Entry::Occupied(_) => {
                    Err(format!("device '{}' already exists", prefix).into())
                },
                Entry::Vacant(v) => {
                    v.insert(fc);
                    info!("device '{}' ({}) connected", prefix, addr);
                    Ok(())
                },
            },
            Err(e) => Err(format!("cannot connect to {} ({}): {:?}", prefix, addr, e)),
        }
    });
    Ok(())
}

fn record_init(record: &mut AnyRecord) -> epics::Result<AnyHandlerBox> {
    info!("record_init ...");
    let full_name = String::from(from_utf8(record.name()).unwrap());
    let (pref, name) = split_name(&full_name).unwrap();
    info!("... {}", full_name);
    match record {
        AnyRecord::Stringin(_) => {
            match name {
                "IDN" => Ok(Box::new(IdnHandler::new(pref)) as Box<dyn StringinHandler + Send>),
                _ => { error!("no such device"); Err(())},
            }
        }.map(|t| t.into()),
        _ => { error!("unknown record"); Err(()) },
    }.unwrap()
}

bind_device_support!(
    init,
    record_init,
);
