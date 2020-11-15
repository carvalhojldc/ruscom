mod serial;

use clap::{crate_authors, crate_name, crate_version, App, Arg, ArgMatches};

const SW_ABOUT: &str = "A simple serial port reader by CLI";

const ARG_LIST: &'static [&'static str] =  &["list", "port", "baud"];

fn main() {
    let cli_opt = args();

    println!();

    if cli_opt.is_present("list") {
        serial::show_available_ports()
    }

    if cli_opt.is_present("port") {
        let name = cli_opt.value_of("port").unwrap().to_string();
        let baud_rate = cli_opt.value_of("baud").unwrap().parse::<u32>().unwrap();

        let port = serial::SerialPortDefs {
            name: name,
            baud_rate: baud_rate,
        };

        println!(
            "Using: port = {:}, buad rate = {:}",
            port.name, port.baud_rate
        );

        serial::open_and_read(port)
    }
}

fn args() -> ArgMatches<'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(SW_ABOUT)
        .arg(
            Arg::with_name(ARG_LIST[0])
                .short("l")
                .long("list")
                .conflicts_with_all(&[ARG_LIST[1], ARG_LIST[2]])
                .help("Show list of available serial ports"),
        )
        .arg(
            Arg::with_name(ARG_LIST[1])
                .requires(ARG_LIST[2])
                .short("p")
                .long("port")
                .takes_value(true)
                .help("The device path to a serial port"),
        )
        .arg(
            Arg::with_name(ARG_LIST[2])
                .short("b")
                .long("baud")
                .requires(ARG_LIST[1])
                .takes_value(true)
                .help("The baud rate to connect")
                .validator(valid_baud),
        )
        .get_matches()
}

fn valid_baud(val: String) -> Result<(), String> {
    val.parse::<u32>()
        .map(|_| ())
        .map_err(|_| format!("Invalid baud rate '{}' specified", val))
}
