use crate::{driver::dummy::dummy_init, net::{device::NetDevice, net_device_output, net_device_register, net_down, net_init, net_up}, utils::lprintf};
use std::{thread, time};

mod net;
mod utils;
mod driver;

static mut TERMINATE_FLAG: bool = false;

fn main() {
    let mut dev_list: Vec<NetDevice> = Vec::new();
    if -1 == net_init(&mut dev_list) {
        panic!("init net failed");
    }

    let dummy_dev = dummy_init();
    net_device_register(dummy_dev, &mut dev_list);
    if -1 == net_up(&mut dev_list) {
        panic!("up net failed");
    }

    let handler = || {
        unsafe { TERMINATE_FLAG = true };
    };
    ctrlc::set_handler(handler).expect("error setting ctrl-c handler");

    let data = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10];
    while !unsafe { TERMINATE_FLAG } {
        if -1 == net_device_output(&mut dev_list[0], 0x0800, &data, 16, 0) {
            lprintf("net device output failed", file!(), line!());
            break;
        }
        thread::sleep(time::Duration::from_millis(1000));
    }

    net_down(&mut dev_list);
}
