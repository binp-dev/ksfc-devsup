mod device;
mod handler;

use std::sync::Mutex;
use std::str::from_utf8;
use std::collections::hash_map::{HashMap, Entry};


use log::{info, error};
//use simple_logger;
use lazy_static::lazy_static;


use epics::{
    self,
    bind_device_support,
    register_command,
    record::*,
    context::*,
};

use device::Device;
use handler::*;


lazy_static! {
    static ref DEVICES: Mutex<HashMap<String, Device>> = Mutex::new(HashMap::new());
}

fn split_name(name: &str) -> Result<(&str, &str), ()> {
    let mut it = name.rsplitn(2, ':');
    let rec = it.next().unwrap();
    match it.next() {
        Some(pref) => Ok((pref, rec)),
        None => Err(()),
    }
}

fn init(context: &mut Context) -> epics::Result<()> {
    //simple_logger::init().unwrap();
    info!("init");
    register_command!(context, fn connectDevice(addr: &str, prefix: &str) -> epics::Result<()> {
        match DEVICES.lock().unwrap().entry(String::from(prefix)) {
            Entry::Occupied(_) => {
                Err(format!("device '{}' already exists", prefix).into())
            },
            Entry::Vacant(v) => {
                let dev = Device::new(&"10.0.0.9", None);
                v.insert(dev);
                info!("device '{}' ({}) added", prefix, addr);
                Ok(())
            },
        }
    });
    register_command!(context, fn startAll() -> epics::Result<()> {
        let mut was_err = false;
        for (name, dev) in DEVICES.lock().unwrap().iter_mut() {
            match dev.start() {
                Ok(()) => info!("device '{}' started", name),
                Err(e) => {
                    error!("device '{}' cannot start: {:?}", name, e);
                    was_err = true;
                }
            }
        }
        if was_err {
            Err("some devices failed to start".into())
        } else {
            Ok(())
        }
    });
    Ok(())
}

fn record_init(record: &mut AnyRecord) -> epics::Result<AnyHandlerBox> {
    info!("record_init ...");
    let full_name = String::from(from_utf8(record.name()).unwrap());
    let (pref, name) = split_name(&full_name).unwrap();
    info!("... {}", full_name);
    let handle = match DEVICES.lock().unwrap().get_mut(pref) {
        Some(dev) => Ok(dev.handle()),
        None => Err(format!("no such device: {}", pref)),
    }?;
    match record {
        AnyRecord::Stringin(_) => {
            match name {
                "IDN" => Ok(Box::new(IdnHandler::new(handle)) as Box<dyn StringinHandler + Send>),
                _ => Err(()),
            }
        }.map(|t| t.into()),
        _ => Err(()),
    }.map_err(|_| format!("no handler for {:?} record '{}'", record.rtype(), name).into())
}

bind_device_support!(
    init,
    record_init,
);
