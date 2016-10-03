use std::sync::mpsc::{SyncSender, Receiver};

struct Privilege {
    /// The mask for the address which determines bits that cannot be altered.
    permission: u32,
    /// The address of the communication.
    address: u32,
}

impl Privilege {
    fn new(permission: u32, address: u32) -> Privilege {
        Privilege {
            permission: permission,
            address: address,
        }
    }

    fn network_address(&self) -> u32 {
        self.address & self.permission
    }

    fn local_address(&self) -> u32 {
        self.address & !self.permission
    }
}

struct Com<T> {
    message: T,
    privilege: Privilege,
    id: u32,
}

pub struct U32Bus {
    remote_id: u32,
    interrupt: SyncSender<Com<u32>>,
    kill: SyncSender<Com<()>>,
    incept: SyncSender<Com<Receiver<u32>>>,
    stream: SyncSender<Com<Receiver<u32>>>,
}

pub struct U032 {
    pc: u32,
    dcs: [u32; 4],

    pmem: Vec<u8>,
    mem: Vec<u32>,

    total_buses: u32,

    interrupt_receiver: Receiver<Com<u32>>,
    kill_receiver: Receiver<Com<()>>,
    incept_receiver: Receiver<Com<Receiver<u32>>>,
    stream_receiver: Receiver<Com<Receiver<u32>>>,

    interrupt_sender: SyncSender<Com<u32>>,
    kill_sender: SyncSender<Com<()>>,
    incept_sender: SyncSender<Com<Receiver<u32>>>,
    stream_sender: SyncSender<Com<Receiver<u32>>>,

    out_buses: Vec<U32Bus>,
}

impl U032 {
    pub fn bus_acquirer<'a>(&'a mut self) -> BusAcquirer<'a> {
        BusAcquirer { core: self }
    }
}

pub struct BusAcquirer<'a> {
    core: &'a mut U032,
}

impl<'a> Iterator for BusAcquirer<'a> {
    type Item = U32Bus;
    fn next(&mut self) -> Option<U32Bus> {
        let bus = U32Bus {
            remote_id: self.core.total_buses,
            interrupt: self.core.interrupt_sender.clone(),
            kill: self.core.kill_sender.clone(),
            incept: self.core.incept_sender.clone(),
            stream: self.core.stream_sender.clone(),
        };
        self.core.total_buses += 1;
        Some(bus)
    }
}
