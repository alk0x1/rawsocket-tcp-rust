use std::{net::{IpAddr, Ipv4Addr, SocketAddr}, str::FromStr, io::{Write, stdout}, mem::MaybeUninit, fs::File, thread};
use inquire::{Text, validator::Validation};
use socket2::{Socket, Domain, Type, SockAddr};
use tcp_connection::calculate_hash;

fn main() {
  handle_user_input_connection();
}

fn handle_user_input_connection() {
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

          let handle = thread::spawn(move || {
            handle_connection(ip, port);
          });
          
          handle.join().unwrap();
        }
        Err(err) => println!("Error on validating port {}", err),
      }
    },
    Err(err) => println!("Error on validating ip: {}", err),
  }
}

fn handle_connection(ip_string: String, port_string: String) {
  match ip_string.parse::<Ipv4Addr>() {
    Ok(ipv4_addr) => {
      match port_string.parse::<u16>() {
        Ok(port) => {
          let address = SocketAddr::new(IpAddr::V4(ipv4_addr), port);
          let server_socket_addr = SockAddr::from(address);
          let socket = Socket::new(Domain::IPV4, Type::STREAM, None).unwrap();
          match socket.connect(&server_socket_addr.into()) {
            Ok(_) => {
              println!("Connected!");
              let buffer = [MaybeUninit::uninit(); 1024];
              
              loop {
                let stdin = std::io::stdin();
                let mut input = String::new();
                print!("Enter a message: ");
                stdout().flush().unwrap();
                stdin.read_line(&mut input).expect("Failed to read line");
                socket.send(input.as_bytes()).expect("Failed to send data");

                let (_, received_text) = handle_server_data(&socket, buffer);
                
                println!("Resposta: {}", received_text);

                if received_text == "Conexão finalizada." {
                  break;
                }
                else if received_text.starts_with("Starting file transference...") {
                  let (_, filename) = handle_server_data(&socket, buffer);
                  println!("Received Filename: {}", filename);
                  let (_, size) = handle_server_data(&socket, buffer);
                  println!("Received Size: {}", size);
                  let (_, crc) = handle_server_data(&socket, buffer);
                  println!("Received CRC: {}", crc);
                  let (_, file_content) = handle_file_bytes(&socket, buffer);
                  println!("Received Content: {:?}", file_content);

                  let path = &format!("{}{}", "src/clientFiles/", filename);
                  println!("File received and saved successfully");

                  let (_, status) = handle_server_data(&socket, buffer);
                  println!("Received Status: {:?}", status);

                  match File::create(path) {
                    Ok(mut f) => {
                      match f.write_all(file_content) {   
                        Ok(_) => {
                          println!("File received and saved successfully");
                          let file_for_hash = File::open(path).expect("Failed to open file for hash calculation");

                          if crc == calculate_hash(&file_for_hash) {
                            println!("Valid hash");
                          }
                          else {
                            println!("Invalid hash");
                          }
                        },
                        Err(err) => eprintln!("Error on write bytes: {}", err)
                      };
                    }
                    Err(e) => {
                      eprintln!("Error on create file: {}", e);
                    }
                  }
                }
              }
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

fn handle_server_data(s: &Socket, mut buffer: [MaybeUninit<u8>; 1024]) -> (&Socket, &'static str) {
  let (size, _) =  match s.recv_from(&mut buffer) {
    Ok(s) => s,
    Err(e) => {
      eprintln!("Failed to receive data from server: {}", e);
      return (s, "0");
    }
  };

  let received_data = unsafe {
    std::slice::from_raw_parts(buffer.as_ptr() as *const u8, size)
  };
  
  let received_text = match std::str::from_utf8(received_data) {
    Ok(s) => s,
    Err(e) => {
      eprintln!("Failed to format data: {}", e);
      "error - received_text"
    }
  };

  (s, received_text)
}

fn handle_file_bytes(s: &Socket, mut buffer: [MaybeUninit<u8>; 1024]) -> (&Socket, &[u8]) {
  let (size, _) =  s.recv_from(&mut buffer).expect("Failed to receive file bytes from the server");
  
  let data = unsafe {
    std::slice::from_raw_parts(buffer.as_ptr() as *const u8, size)
  };

  (s, data)
}