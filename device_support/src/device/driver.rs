use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Receiver, TryRecvError, RecvTimeoutError};

use log::{info, error};

use epics::{Scan};

use ksfc_lxi::{KsFc, types::{ChannelNo}};


static RECV_TIMEOUT: Duration = Duration::from_secs(1);
static RECONNECT_PERIOD: Duration = Duration::from_secs(10);

pub enum ChanCmd {
    SetScan(Scan),
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
    active: bool,
}

#[derive(Default)]
struct Par {
    idn: Option<Scan>,
    chans: [ChanPar; 2],
    measure: bool,
}

pub struct Data {
    pub idn: Option<String>,
}

impl Data {
    pub fn new() -> Self {
        Self {
            idn: None,
        }
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

    fn get_idn(&mut self) -> epics::Result<()> {
        self.dev.idn().map_err(|e| format!("{}", e).into())
        .and_then(|idn| {
            self.data.lock().unwrap().idn = Some(idn);
            match self.par.idn {
                Some(ref s) => s.request().map_err(|()| "scan request error".into()),
                None => Err("no scan for IDN".into()),
            }
        })
    }

    fn on_connect(&mut self) -> epics::Result<()> {
        self.get_idn()
    }

    fn connect_if_need(&mut self) -> bool {
        if self.dev.is_connected() {
            !self.par.measure
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
                let ch = match cn {
                    ChannelNo::Ch1 => &mut self.par.chans[0],
                    ChannelNo::Ch2 => &mut self.par.chans[1],
                };
                match chan_cmd {
                    ChanCmd::SetScan(scan) => if ch.freq.replace(scan).is_some() {
                        error!("Reset scan for FREQ_{}", cn as u8);
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

    pub fn start(mut self) {
        'outer: loop {
            let mut wait = self.connect_if_need();

            'inner: loop {
                let cmd = if wait {
                    match self.rx.recv_timeout(RECV_TIMEOUT) {
                        Ok(cmd) => cmd,
                        Err(e) => match e {
                            RecvTimeoutError::Timeout => break 'inner,
                            RecvTimeoutError::Disconnected => unreachable!(),
                        },
                    }
                } else {
                    match self.rx.try_recv() {
                        Ok(cmd) => cmd,
                        Err(e) => match e {
                            TryRecvError::Empty => break 'inner,
                            TryRecvError::Disconnected => unreachable!(),
                        },
                    }
                };
                wait = false;

                if self.handle_cmd(cmd) {
                    break 'outer;
                }
            }

            if self.dev.is_connected() {
                if self.just_conn {
                    self.on_connect().unwrap();
                }
                self.just_conn = false;
            }
        }
    }
}
