// Scan PCI bus and print devices

use x86_64::instructions::port::Port;
use crate::println;

fn pci_config_read_word(bus: u8, slot: u8, func: u8, offset: u8) -> u16 {
    let mut address: u32 = 0x80000000;
    address |= ((bus as u32) << 16) | ((slot as u32) << 11) | ((func as u32) << 8) | ((offset as u32) & 0xfc);
    let mut port = Port::new(0xcf8);
    unsafe {
        port.write(address);
        let mut port = Port::new(0xcfc);
        port.read()
    }
}

fn pci_check_vendor(bus: u8, slot: u8) -> u16 {
    let vendor: u16;
    let device: u16;

    vendor = pci_config_read_word(bus, slot, 0, 0);
    if vendor != 0xffff {
        device = pci_config_read_word(bus, slot, 0, 2);
        println!("Found device {} vendor {} at bus {} device {}", device, vendor, bus, slot);
    }

    vendor
}

fn check_function(bus: u8, device: u8, func: &mut u8) {
    let base_class: u8;
    let sub_class: u8;
    let secondary_bus: u8;

    base_class = get_base_class(bus, device, *func);
    sub_class = get_sub_class(bus, device, *func);
    if (base_class == 0x06) && (sub_class == 0x04) {
        secondary_bus = get_secondary_bus(bus, device, *func);
        check_all_functions(secondary_bus);
    }

    println!("Found function {} at bus {} device {} - base class {} sub class {}", *func, bus, device, base_class, sub_class);
}

fn check_all_functions(bus: u8) {
    for device in 0..32 {
        check_function(bus, device, &mut 0);
    }
}

fn get_base_class(bus: u8, device: u8, func: u8) -> u8 {
    let class: u8;
    let vendor_id: u16;

    vendor_id = pci_check_vendor(bus, device);
    if vendor_id == 0xffff {
        return 0xff;
    }

    class = pci_config_read_word(bus, device, func, 0x0b) as u8;
    class
}

fn get_sub_class(bus: u8, device: u8, func: u8) -> u8 {
    let vendor_id: u16;

    vendor_id = pci_check_vendor(bus, device);
    if vendor_id == 0xffff {
        return 0xff;
    }

    pci_config_read_word(bus, device, func, 0x0a) as u8
}

fn get_secondary_bus(bus: u8, device: u8, func: u8) -> u8 {
    let secondary_bus: u8;
    let vendor_id: u16;

    vendor_id = pci_check_vendor(bus, device);
    if vendor_id == 0xffff {
        return 0xff;
    }

    secondary_bus = pci_config_read_word(bus, device, func, 0x19) as u8;
    secondary_bus
}

fn check_device(bus: u8, device: u8) {
    let mut func = 0;

    let vendor_id = pci_check_vendor(bus, device);
    if vendor_id == 0xffff {
        return;
    }

    check_function(bus, device, &mut func);
    let header_type = pci_config_read_word(bus, device, func, 0x0e) as u8;
    if (header_type & 0x80) != 0 {
        for mut func in 1..8 {
            let vendor_id = pci_check_vendor(bus, device);
            if vendor_id != 0xffff {
                check_function(bus, device, &mut func);
            }
        }
    }

    println!("Found device at bus {} device {} function {}", bus, device, func);
}

fn check_all_buses() {
    for bus in 0..255 {
        for device in 0..32 {
            check_device(bus, device);
        }
    }
}

pub fn scan_pci() {
    check_all_buses();
}