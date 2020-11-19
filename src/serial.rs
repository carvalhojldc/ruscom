use std::io::{self, Write};
use std::time::Duration;

use serialport::prelude::*;
use serialport::SerialPortType;
use std::process::exit;

pub struct SerialPortDefs {
    pub name: String,
    pub baud_rate: u32,
}

pub fn show_available_ports() {
    match serialport::available_ports() {
        Ok(ports) => {
            match ports.len() {
                0 => println!("  > No ports found."),
                1 => println!("  > Found 1 port:"),
                n => println!("  > Found {} ports:", n),
            };
            println!();
            for p in ports {
                println!("  {}", p.port_name);
                match p.port_type {
                    SerialPortType::UsbPort(info) => {
                        println!("    Type: USB");
                        println!(
                            "    Manufacturer: {}",
                            info.manufacturer.as_ref().map_or("", String::as_str)
                        );
                        println!(
                            "    Product: {}",
                            info.product.as_ref().map_or("", String::as_str)
                        );
                    }
                    SerialPortType::BluetoothPort => {
                        println!("    Type: Bluetooth");
                    }
                    SerialPortType::PciPort => {
                        println!("    Type: PCI");
                    }
                    SerialPortType::Unknown => {
                        println!("    Type: Unknown");
                    }
                }
                println!();
            }
        }
        Err(e) => {
            eprintln!("Error {:?}, {:}", e.kind, e.description);
            eprintln!("Error listing serial ports");
        }
    }
}

pub fn open_and_read(config: SerialPortDefs) {
    let s = SerialPortSettings {
        baud_rate: config.baud_rate,
        data_bits: DataBits::Eight,
        flow_control: FlowControl::None,
        parity: Parity::None,
        stop_bits: StopBits::One,
        timeout: Duration::from_millis(10),
    };

    let mut port = match serialport::open_with_settings(&config.name, &s) {
        Ok(port) => {
            println!("Success when opening port");
            port
        }
        Err(e) => {
            eprintln!("Error {:?}, {:}", e.kind, e.description);
            exit(1);
        }
    };

    println!();

    let mut serial_buf: Vec<u8> = vec![0; 1000];
    loop {
        match port.read(serial_buf.as_mut_slice()) {
            Ok(t) => io::stdout().write_all(&serial_buf[..t]).unwrap(),
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            Err(e) => {
                eprintln!("Error {:?}. {:}", e.kind(), e.to_string());
                exit(2);
            }
        }
    }
}
