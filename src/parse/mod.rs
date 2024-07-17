mod cobs;

mod alt_data;
mod barometer;
mod gps;
mod imu_data;
mod pitot;
mod servo_data;
mod vane;

mod parser;

pub use cobs::decode_cobs;
pub use barometer::BarometerData;
pub use servo_data::ServoData;
pub use alt_data::AltData;
pub use gps::GPSData;
pub use imu_data::IMUData;
pub use pitot::PitotData;
pub use vane::VaneData;
pub use parser::Parser;

pub trait Data{
    fn parse(data: &Vec<u8>) -> Self;
    fn get_size() -> usize;
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test(){
        let mut parser = Parser::new();
        if let Ok(port)=serialport::new("COM9",115200).open(){
            parser.set_port(port);
        }else{
            panic!("Failed to open port");
        }
        loop{
            parser.parse();
        }
    }
}
