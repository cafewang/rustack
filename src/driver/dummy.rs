use std::u16;

use crate::utils::lprintf;
use crate::net::NET_DEVICE_TYPE_DUMMY;
use crate::net::device::{NetDevice, NetDeviceOps, BroadcastOrPeer};

pub fn dummy_transmit(_net_dev: &mut NetDevice, _dev_type: u16, _data: &[u8], len: u16, _dst: u8) -> i32 {
    lprintf(format!("dummy transmit just drop data len={}", len).as_str(), file!(), line!());
    return 0;
}

pub fn dmmuy_open(_net_dev: &mut NetDevice) -> i32 {
    return 0;
}

pub fn dummy_close(_net_dev: &mut NetDevice) -> i32 {
    return 0;
}

const OPS: NetDeviceOps = NetDeviceOps {
    open: dmmuy_open,
    close: dummy_close,
    transmit: dummy_transmit,
};

pub fn dummy_init() -> NetDevice<'static> {
    let dev: NetDevice = NetDevice { 
        dev_type: NET_DEVICE_TYPE_DUMMY, 
        flags: 0,
        mtu: u16::MAX, 
        hlen: 0, 
        alen: 0, 
        addr: [0; 16], 
        broadcast_or_peer: BroadcastOrPeer { broadcast: [0; 16] }, 
        ops: &OPS, 
        priv_data: 0,
    };
    return dev;
}