// Module for the App struct and its implementation
use serialport::SerialPort;
use crossterm::event::{self, Event, KeyCode};

use crate::parse::{decode_cobs, ServoData, AltData, IMUData, Data};

pub struct App{
    pub port: Box<dyn SerialPort>,
    pub log: Vec<u8>,
    pub servoctrl_data: Option<ServoData>,
    pub alt_data: Option<AltData>,
    pub imu_data: Option<IMUData>,
}

impl App{
    pub fn new(port_name: &str, baud_rate: u32) -> Self{
        let port = serialport::new(port_name, baud_rate)
            .timeout(std::time::Duration::from_millis(10))
            .open();
        match port {
            Ok(port) => {
                App{
                    port,
                    log: Vec::new(),
                    servoctrl_data: None,
                    alt_data: None,
                    imu_data: None,
                }
            }
            Err(e) => {
                eprintln!("Failed to open \"{}\". Error: {}", port_name, e);
                ::std::process::exit(1);
            }
        }
    }

    pub fn handle_events(&mut self) -> std::io::Result<bool> {
        let mut serial_buf: Vec<u8> = vec![0; 1000];
        match self.port.read(serial_buf.as_mut_slice()) {
            Ok(t) => {
                self.log.extend_from_slice(&serial_buf[..t]);
                let (decoded, rest) = decode_cobs(self.log.clone());
                if decoded.len() > 0 {
                    self.log = rest;
                    match decoded[0]&0xF0 {
                        0x10 => {
                            self.servoctrl_data=Some(ServoData::parse(&decoded));
                        }
                        0x40 => {
                            self.imu_data=Some(IMUData::parse(&decoded));
                        }
                        0x50 => {
                            for i in 0..5{
                                self.alt_data=Some(AltData::parse(&decoded[i*12..(i+1)*12].to_vec()));
                            }
                        }
                        _ => {}
                    }
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => (),
            Err(e) => eprintln!("{:?}", e),
        }
        if event::poll(std::time::Duration::from_millis(5))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }
}