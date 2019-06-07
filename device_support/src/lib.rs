mod device;
mod handlers;

use std::sync::Mutex;
use std::collections::hash_map::{HashMap, Entry};

use simple_logger;
use lazy_static::lazy_static;

use epics::{
    self,
    log::{error, info},
    register_command,
    context::Context,
};

use device::Device;


lazy_static! {
    static ref DEVICES: Mutex<HashMap<String, Device>> = Mutex::new(HashMap::new());
}

fn init(context: &mut Context) -> epics::Result<()> {
    simple_logger::init().unwrap();
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

mod bind {
    use crate::handlers::*;

    epics::bind_device_support!(
        super::init,
        {
            Idn,
            Measure,
            ChanAct,
            ChanFreq,
            ChanGateTime,
        },
    );

}
