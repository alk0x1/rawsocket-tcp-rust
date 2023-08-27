use std::{net::{IpAddr, Ipv4Addr, SocketAddr}, str::FromStr};
use socket2::{Socket, Domain, Type, SockAddr};

fn main() {

  handle_user_input();
}

fn handle_user_input() {
  use inquire::{Text, validator::{StringValidator, Validation}};
                                                                      // if let Ok(_) = Ipv4Addr::from_str(input)
  let ip_validator = |input: &str| if let Ok(_) = Ipv4Addr::from_str(input) {
    Ok(Validation::Valid)
  } else {
    Ok(Validation::Invalid("Invalid ip address.".into()))
  };

  let port_validator = |input: &str| {
    if let Ok(port) = input.parse::<u16>() {
        if port >= 1 && port <= 65535 {
            Ok(Validation::Valid)
        } else {
            Ok(Validation::Invalid("Port must be between 1 and 65535.".into()))
        }
    } else {
        Ok(Validation::Invalid("Invalid port format.".into()))
    }
};

  let ip_status = Text::new("Ip address:").with_validator(ip_validator).prompt();

  match ip_status {
    Ok(ip) => {
      let port_status = Text::new("Port:").with_validator(port_validator).prompt();
      match port_status {
        Ok(port) => {
          println!("Trying to connect in {}:{} ...", ip, port);
          handle_connection(ip, port);

        }
        Err(err) => println!("Error on validating port {}", err),
      }
    },
    Err(err) => println!("Error on validating ip: {}", err),
  }

  // let options: Vec<&str> = vec!["Banana", "Apple", "Strawberry", "Grapes"];
  // let opt_answ: Result<&str, InquireError> = Select::new("",options).prompt();
}

fn handle_connection(ip_string: String, port_string: String) {
  match ip_string.parse::<Ipv4Addr>() {
    Ok(ipv4_addr) => {
      match port_string.parse::<u16>() {
        Ok(port) => {
          let address = SocketAddr::new(IpAddr::V4(ipv4_addr), port);
          let server_socket_addr = SockAddr::from(address);
          let socket = Socket::new(Domain::IPV4, Type::STREAM, None).unwrap();
          let connection = socket.connect(&server_socket_addr.into());

          match connection {
            Ok(connection) => {
              println!("connected to server: {:?}", connection);
            } 
            Err(err) => {
              println!("Error connecting to server: {}", err);
            }
          }
        }
        Err(err) => {
          println!("Error on parse port: {:?}", err);
        }
      }
    }
    Err(err) => {
      println!("Error on parse ip: {:?}", err);
    }
  }
}