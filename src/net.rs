pub mod device;

pub const NET_DEVICE_ADDR_LEN: usize = 16;

pub const NET_DEVICE_TYPE_DUMMY: u16 = 0x00;
pub const NET_DEVICE_TYPE_LOOPBACK: u16 = 0x01;
pub const NET_DEVICE_TYPE_ETHERNET: u16  = 0x02;

pub const NET_DEVICE_FLAG_UP: u16 = 0x01;
pub const NET_DEVICE_FLAG_LOOPBACK: u16 = 0x010;
pub const NET_DEVICE_FLAG_BROADCAST: u16 = 0x020;
pub const NET_DEVICE_FLAG_P2P: u16 = 0x040;
pub const NET_DEVICE_FLAG_MULTICAST: u16 = 0x100;

use device::{ NetDevice };
use crate::utils::{lprintf, dump};
pub fn net_device_register<'a>(dev: NetDevice<'a>, dev_list: &mut Vec<NetDevice<'a>>) {
    lprintf(format!("Register net device type: {}", dev.dev_type).as_str(), file!(), line!());
    dev_list.push(dev);
}

pub fn net_device_is_up(dev: &NetDevice) -> bool {
    return (dev.flags & NET_DEVICE_FLAG_UP) != 0;
}

pub fn net_device_open(dev: &mut NetDevice) -> i32 {
    if net_device_is_up(dev) {
        lprintf(format!("Device {} is already up", dev.dev_type).as_str(), file!(), line!());
        return -1;
    }


    if (dev.ops.open)(dev) == -1 {
        lprintf(format!("Open device {} failed", dev.dev_type).as_str(), file!(), line!());
        return -1;
    }

    dev.flags |= NET_DEVICE_FLAG_UP;
    lprintf(format!("Open device {} success", dev.dev_type).as_str(), file!(), line!());
    return 0;
}

pub fn net_device_close(dev: &mut NetDevice) -> i32 {
    if !net_device_is_up(dev) {
        lprintf(format!("Device {} is already down", dev.dev_type).as_str(), file!(), line!());
        return -1;
    }

    if (dev.ops.close)(dev) == -1 {
        lprintf(format!("Close device {} failed", dev.dev_type).as_str(), file!(), line!());
        return -1;
    }

    dev.flags &= !NET_DEVICE_FLAG_UP;
    lprintf(format!("Close device {} success", dev.dev_type).as_str(), file!(), line!());
    return 0;
}

pub fn net_device_output(dev: &mut NetDevice, dev_type: u16, data: &[u8], len: u16, dst: u8) -> i32 {
    if !net_device_is_up(dev) {
        lprintf(format!("Device {} is down", dev.dev_type).as_str(), file!(), line!());
        return -1;
    }

    if len > dev.mtu {
        lprintf(format!("Packet size {} is larger than mtu {}", len, dev.mtu).as_str(), file!(), line!());
        return -1;
    }
    dump(data, len);
    if -1 == (dev.ops.transmit)(dev, dev_type, data, len, dst) {
        lprintf(format!("Output packet failed").as_str(), file!(), line!());
        return -1;
    }
    return 0;
}

pub fn net_device_input_handler(dev_type: u16, data: &[u8], len: u16, dev: &mut NetDevice) -> i32 {
    lprintf(format!("data input, len={}", len).as_str(), file!(), line!());
    dump(data, len);
    return 0;
}

pub fn net_up(dev_list: &mut Vec<NetDevice>) -> i32 {
    lprintf("open all devices...", file!(), line!());
    for dev in dev_list {
        net_device_open(dev);
    }
    lprintf("up and running...", file!(), line!());
    return 0;
}

pub fn net_down(dev_list: &mut Vec<NetDevice>) -> i32 {
    lprintf("close all devices...", file!(), line!());
    for dev in dev_list {
        net_device_close(dev);
    }
    lprintf("shutdown...", file!(), line!());
    return 0;
}

pub fn net_init(dev_list: &mut Vec<NetDevice>) -> i32 {
    lprintf("init all devices...", file!(), line!());
    return 0;
}

