# ruscom - A simple serial port reader by CLI

This project was born through the desire to learn Rust. : D


## To compile and run

### To compile:
```
    cargo build
```

PS: Think about using the flag '--release'

### To run:

Command line interface:
```
USAGE:
    ruscom [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -l, --list       Show list of available serial ports
    -V, --version    Prints version information

OPTIONS:
    -b, --baud <baud>    The baud rate to connect
    -p, --port <port>    The device path to a serial port
```

* Example to read serial
```
    ruscom -p COM23 -b 115200 
```

* Example to show list of serial ports
```
    ruscom --list
```
Output
```
  > Found 1 port:
  COM23
    Type: USB
    Manufacturer: Silicon Labs
    Product: Silicon Labs CP210x USB to UART Bridge (COM23)
```