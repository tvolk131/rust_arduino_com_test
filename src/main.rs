use std::{collections::HashMap};

use serialport::{UsbPortInfo, SerialPortInfo};

fn main() {
    let mut connected_ports: HashMap<String, SerialPortInfo> = HashMap::new();
    loop {
        let ports: HashMap<String, SerialPortInfo> = serialport::available_ports().unwrap().into_iter().map(|port| (port.port_name.clone(), port)).collect();
        let mut newly_connected_ports: Vec<SerialPortInfo> = Vec::new();
        for (_, port) in &ports {
            if !connected_ports.contains_key(&port.port_name) {
                newly_connected_ports.push(port.clone());
                connected_ports.insert(port.port_name.clone(), port.clone());
            }
        }

        let mut newly_disconnected_ports: Vec<SerialPortInfo> = Vec::new();
        for (port_name, port) in &connected_ports.clone() {
            if !ports.contains_key(port_name) {
                newly_disconnected_ports.push(port.clone());
                connected_ports.remove(port_name);
            }
        }

        if !newly_connected_ports.is_empty() {
            println!("Newly connected ports: {:#?}", newly_connected_ports);
        }
        if !newly_disconnected_ports.is_empty() {
            println!("Newly disconnected ports: {:#?}", newly_disconnected_ports);
        }
    }
}

fn get_board_type(usb_port_info: &UsbPortInfo) -> ArduinoBoardType {
    // 9025 is the decimal version of 2341.
    // See https://devicehunt.com/view/type/usb/vendor/2341 for board
    // number references, and remember to convert from hex to decimal.
    if usb_port_info.vid != 9025 {
        return ArduinoBoardType::Unknown;
    }

    if usb_port_info.pid == 66 {
        return ArduinoBoardType::Mega2560;
    }

    return ArduinoBoardType::Unknown;
}

#[derive(PartialEq, std::fmt::Debug)]
enum ArduinoBoardType {
    Unknown,
    Mega2560
}
