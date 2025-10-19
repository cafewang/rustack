
pub struct NetDevice<'a> {
    pub dev_type: u16,
    pub mtu: u16,
    pub flags: u16,
    pub hlen: u16, // header length
    pub alen: u16, // address length
    pub addr: [u8; crate::net::NET_DEVICE_ADDR_LEN],
    pub broadcast_or_peer: BroadcastOrPeer,
    pub ops: &'a NetDeviceOps,
    pub priv_data: u8,
}

#[repr(C)]
pub union BroadcastOrPeer {
    pub broadcast: [u8; crate::net::NET_DEVICE_ADDR_LEN],
    pub peer: [u8; crate::net::NET_DEVICE_ADDR_LEN],
}

pub struct NetDeviceOps {
    pub open: fn(net_dev: &mut NetDevice) -> i32,
    pub close: fn(net_dev: &mut NetDevice) -> i32,
    pub transmit: fn(net_dev: &mut NetDevice, dev_type: u16, data: &[u8], len: u16, dst: u8) -> i32,
}