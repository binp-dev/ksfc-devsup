use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Receiver, TryRecvError, RecvTimeoutError};

use log::{info, error};

use enum_map::{EnumMap};

use epics::{Scan};

use ksfc_lxi::{
    self as ksfc,
    KsFc,
    types::{ChannelNo, TriggerSource},
};


const RECV_TIMEOUT: Duration = Duration::from_secs(1);
const RECONNECT_PERIOD: Duration = Duration::from_secs(10);

pub enum ChanCmd {
    SetScan(Scan),
    SetGateTime(Duration),
    Activate(bool),
}

pub enum Cmd {
    Stop,
    IdnSetScan(Scan),
    Chan(ChannelNo, ChanCmd),
    Measure(bool),
}

#[derive(Default)]
struct ChanPar {
    freq: Option<Scan>,
    gate_time: Duration,
    active: bool,
}

#[derive(Default)]
struct Par {
    idn: Option<Scan>,
    channels: EnumMap<ChannelNo, ChanPar>,
    measure: bool,
}

#[derive(Default)]
pub struct ChanData {
    pub freq: f64,
}

#[derive(Default)]
pub struct Data {
    pub idn: String,
    pub channels: EnumMap<ChannelNo, ChanData>,
}

impl Data {
    pub fn new() -> Self {
        Self::default()
    }
}

pub struct Driver {
    dev:       KsFc,
    data:      Arc<Mutex<Data>>,
    rx:        Receiver<Cmd>,
    par:       Par,
    last_conn: Option<Instant>,
    just_conn: bool,
}

impl Driver {
    pub fn new(dev: KsFc, data: Arc<Mutex<Data>>, rx: Receiver<Cmd>) -> Self {
        Self {
            dev, data, rx,
            par: Par::default(),
            last_conn: None,
            just_conn: false,
        }
    }

    fn get_idn(&mut self) -> ksfc::Result<()> {
        self.dev.idn()
        .and_then(|idn| {
            self.data.lock().unwrap().idn = idn;
            match self.par.idn {
                Some(ref s) => s.request().map_err(|()| "scan request error".into()),
                None => Err("no scan for IDN".into()),
            }
        })
    }

    fn reset(&mut self) -> ksfc::Result<()> {
        self.dev.rst()
        .and_then(|()| self.dev.trigger_source_set(TriggerSource::Immediate))
    }

    fn on_connect(&mut self) -> ksfc::Result<()> {
        self.get_idn()
        .and_then(|()| self.reset())
    }

    fn connect_if_need(&mut self) -> bool {
        if self.dev.is_connected() {
            !(self.par.measure && self.par.channels.iter().any(|(_, ch)| ch.active))
        } else {
            let retry = match self.last_conn {
                Some(inst) => inst.elapsed() >= RECONNECT_PERIOD,
                None => true,
            };
            if retry {
                match self.dev.connect() {
                    Ok(()) => {
                        self.just_conn = true;
                        info!("connected");
                        !self.par.measure
                    }
                    Err(e) => {
                        self.last_conn = Some(Instant::now());
                        error!("cannot connect: {:?}", e);
                        true
                    }
                }
            } else {
                true
            }
        }
    }

    fn handle_cmd(&mut self, cmd: Cmd) -> bool {
        match cmd {
            Cmd::Stop => return true,
            Cmd::IdnSetScan(scan) => {
                if self.par.idn.replace(scan).is_some() {
                    error!("Reset scan for IDN");
                }
            },
            Cmd::Chan(cn, chan_cmd) => {
                let ch = &mut self.par.channels[cn];
                match chan_cmd {
                    ChanCmd::SetScan(scan) => if ch.freq.replace(scan).is_some() {
                        error!("Reset scan for FREQ_{}", cn as u8);
                    },
                    ChanCmd::SetGateTime(gate_time) => {
                        ch.gate_time = gate_time;
                    },
                    ChanCmd::Activate(act) => {
                        ch.active = act;
                    },
                }
                
            },
            Cmd::Measure(meas) => {
                self.par.measure = meas;
            },
        }
        false
    }

    fn receive_cmd(&mut self, wait: bool) -> Option<Cmd> {
        if wait {
            match self.rx.recv_timeout(RECV_TIMEOUT) {
                Ok(cmd) => Some(cmd),
                Err(e) => match e {
                    RecvTimeoutError::Timeout => None,
                    RecvTimeoutError::Disconnected => unreachable!(),
                },
            }
        } else {
            match self.rx.try_recv() {
                Ok(cmd) => Some(cmd),
                Err(e) => match e {
                    TryRecvError::Empty => None,
                    TryRecvError::Disconnected => unreachable!(),
                },
            }
        }
    }

    pub fn start(mut self) {
        self.run_loop();
    }

    fn run_loop(&mut self) {
        'outer: loop {
            let mut wait = self.connect_if_need();

            'inner: loop {
                let cmd = match self.receive_cmd(wait) {
                    Some(cmd) => {
                        wait = false;
                        cmd
                    },
                    None => break 'inner,
                };

                if self.handle_cmd(cmd) {
                    break 'outer;
                }
            }
            
            if self.dev.is_connected() {
                match {
                    if self.just_conn {
                        self.just_conn = false;
                        self.on_connect()
                    } else {
                        Ok(())
                    }
                }.and_then(|()| {
                    if self.par.measure {
                        let mut res = Ok(());
                        let par = &self.par;
                        let dev = &mut self.dev;
                        let data = &mut self.data;
                        for (no, ch) in &par.channels {
                            if ch.active {
                                match dev.configure_frequency(no)
                                .and_then(|()| dev.sense_frequency_gate_time_set(ch.gate_time))
                                .and_then(|()| dev.initiate())
                                .and_then(|()| dev.fetch())
                                .and_then(|val| {
                                    let mut guard = data.lock().unwrap();
                                    guard.channels[no].freq = val;
                                    match par.channels[no].freq {
                                        Some(ref s) => s.request().map_err(|()| "scan request error".into()),
                                        None => Err(format!("no scan for chan_{}", no as u8).into()),
                                    }
                                }) {
                                    Ok(()) => (),
                                    Err(e) => {
                                        res = Err(e);
                                        break;
                                    }
                                }
                            }
                        }
                        res
                    } else {
                        Ok(())
                    }
                }) {
                    Ok(()) => (),
                    Err(err) => {
                        self.dev.disconnect();
                        error!("error occured while measurement, reconnecting: {}", err);
                    },
                }
            }
        }
    }
}
