mod parse;
use parse::Parser;
use serialport;
fn main(){
    let mut parser = Parser::new();
    let mut port_name = "/dev/ttyUSB0".to_string();
    match serialport::available_ports(){
        Ok(ports) => {
            port_name = ports[0].port_name.clone();
        }
        Err(e) => {
            eprintln!("{:?}",e);
        }
    }
    loop{
        if parser.get_port().is_none(){
            match serialport::new(&port_name, 115200).open(){
                Ok(port) => {
                    parser.set_port(port);
                }
                Err(e) => {
                    eprintln!("{:?}",e);
                }
            }
        }
        parser.parse();
    }
}