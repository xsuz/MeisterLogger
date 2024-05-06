// Module for the App struct and its implementation
use serialport::SerialPort;
use crossterm::event::{self, Event, KeyCode};

use crate::parse::decode_cobs;
use byteorder::{ByteOrder, BigEndian};

pub struct App{
    pub port: Box<dyn SerialPort>,
    pub log: Vec<u8>,
    pub servoctrl_data: Option<ServoData>,
    pub alt_data: Option<AltData>,
}

pub struct ServoData{
    pub id: u8,
    pub timestamp: u32,
    pub rudder:f32,
    pub elevator:f32,
    pub voltage:f32,
    pub current_rudder:f32,
    pub current_elevator:f32,
    pub trim:f32,
    pub status:u8,
}

pub struct AltData{
    pub id: u8,
    pub timestamp: u32,
    pub altitude:f32
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
                    if decoded[0]&0xF0 == 0x10 {
                        let data = ServoData{
                            id: decoded[0],
                            timestamp: BigEndian::read_u32(&decoded[4..8]),
                            rudder: BigEndian::read_f32(&decoded[8..13]),
                            elevator: BigEndian::read_f32(&decoded[12..16]),
                            voltage: BigEndian::read_f32(&decoded[16..20]),
                            current_rudder: BigEndian::read_f32(&decoded[20..24]),
                            current_elevator: BigEndian::read_f32(&decoded[24..28]),
                            trim: BigEndian::read_f32(&decoded[28..32]),
                            status: decoded[32],
                        };
                        self.servoctrl_data=Some(data);
                    }
                    if decoded[0]&0xF0 == 0x50 {
                        let data = AltData{
                            id: decoded[0],
                            timestamp: BigEndian::read_u32(&decoded[4..8]),
                            altitude: BigEndian::read_f32(&decoded[8..12]),
                        };
                        self.alt_data=Some(data);
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