mod parse;
use parse::Parser;
use serialport;
fn main(){
    let mut parser = Parser::new();
    let mut port_name = "".to_string();
    match serialport::available_ports(){
        Ok(ports) => {
            if ports.len() != 0{
                port_name = ports[0].port_name.clone();
            }
        }
        Err(e) => {
            eprintln!("{:?}",e);
        }
    }
    loop{
        if parser.get_port().is_none(){

            if port_name.eq(""){
                match serialport::available_ports(){
                    Ok(ports) => {
                        if ports.len() != 0{
                            port_name = ports[0].port_name.clone();
                        }
                    }
                    Err(e) => {
                        eprintln!("{:?}",e);
                    }
                }
            }else{

            match serialport::new(&port_name, 115200).open(){
                Ok(port) => {
                    parser.set_port(port);
                }
                Err(e) => {
                    eprintln!("{:?}",e);
                }
            }
            }

        }
        parser.parse();
    }
}