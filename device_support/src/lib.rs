mod device;
mod handlers;

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

use ksfc_lxi::{
    types::{ChannelNo},
};

use device::Device;
use handlers::*;


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
    let full_name = String::from(from_utf8(record.name()).unwrap());
    let (pref, name) = split_name(&full_name).unwrap();
    info!("record_init({})", full_name);
    let handle = match DEVICES.lock().unwrap().get_mut(pref) {
        Some(dev) => Ok(dev.handle()),
        None => Err(format!("no such device: {}", pref)),
    }?;
    match name {
        "IDN" => Ok(IdnHandler::new(handle).into_any_box()),

        "CHAN_1" =>      Ok( ChanActHandler::new(handle, ChannelNo::Ch1).into_any_box()),
        "GATE_TIME_1" => Ok(GateTimeHandler::new(handle, ChannelNo::Ch1).into_any_box()),
        "FREQ_1" =>      Ok(ChanFreqHandler::new(handle, ChannelNo::Ch1).into_any_box()),

        "CHAN_2" =>      Ok( ChanActHandler::new(handle, ChannelNo::Ch2).into_any_box()),
        "GATE_TIME_2" => Ok(GateTimeHandler::new(handle, ChannelNo::Ch2).into_any_box()),
        "FREQ_2" =>      Ok(ChanFreqHandler::new(handle, ChannelNo::Ch2).into_any_box()),

        "MEASURE" => Ok(MeasureHandler::new(handle).into_any_box()),
        _ => Err(format!("no handler for {:?} record '{}'", record.rtype(), name).into()),
    }
}

bind_device_support!(
    init,
    record_init,
);
