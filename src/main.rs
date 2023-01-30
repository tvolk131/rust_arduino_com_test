use std::{collections::HashMap, time::Duration};

use serialport::{SerialPortInfo, SerialPortType, UsbPortInfo};

mod call_response_serial_port;

use call_response_serial_port::CallResponseSerialPort;

fn main() {
    let mut connected_ports: HashMap<String, SerialPortInfo> = HashMap::new();
    loop {
        let ports: HashMap<String, SerialPortInfo> = serialport::available_ports()
            .unwrap()
            .into_iter()
            .map(|port| (port.port_name.clone(), port))
            .collect();
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

        if !newly_connected_ports.is_empty() {
            for port in &newly_connected_ports {
                if let SerialPortType::UsbPort(usb_port_info) = &port.port_type {
                    let board_type = get_board_type(&usb_port_info);
                    if board_type == ArduinoBoardType::Mega2560 {
                        println!("{:#?} plugged into {}", board_type, port.port_name);
                        let port_builder = serialport::new(&port.port_name, 57600)
                            .timeout(Duration::from_millis(1))
                            .data_bits(serialport::DataBits::Eight);
                        match port_builder.open() {
                            Ok(port) => {
                                println!(
                                    "Opened port to {:?} on port {:?}",
                                    board_type,
                                    port.name().unwrap_or("unnamed device".to_string())
                                );

                                match CallResponseSerialPort::new(port) {
                                    Ok(mut call_response_serial_port) => {
                                        println!("{:?}", call_response_serial_port.get_supported_commands());
                                        for _ in 1..100 {
                                            println!("Calling execute_command(stepper0)...");
                                            println!("Response to `execute_command(stepper0)`: {:?}", call_response_serial_port.execute_command("stepper0"));
                                            std::thread::sleep(Duration::from_millis(1000));
                                        }
                                    },
                                    Err(err) => println!("Failed to open call-response serial communication channel: {:?}", err)
                                };
                            }
                            Err(err) => {
                                println!("Unable to open port to {:?}: {}", board_type, err)
                            }
                        };
                    }
                }
            }
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
    Mega2560,
}
