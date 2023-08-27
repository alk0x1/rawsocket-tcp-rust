use std::{net::SocketAddr, mem::MaybeUninit, thread};
use socket2::{Socket, Domain, Type};

fn main() -> Result<(), Box<dyn std::error::Error>> {
  // 1. creation
  let socket = Socket::new(Domain::IPV4, Type::STREAM, None).unwrap();
  let addr = "127.0.0.1:8080".parse::<SocketAddr>().unwrap();

  // 2. binding
  socket.bind(&addr.into()).unwrap();

  // 3. listeing
  socket.listen(128).unwrap();
  println!("Server listening on {}", addr);


  // 4. accepting connections
  loop {
    let (s, _) = match socket.accept() {
      Ok(s) => {
        println!("{:?}, connected", s.1.as_socket_ipv4().unwrap());
        s
      },
      Err(e) => {
        eprintln!("Failed to accept connection: {}", e);
        continue;
      }
    };

    handle_connection(s);
  }
}

fn handle_connection(s: Socket) {
  let mut buffer = [MaybeUninit::uninit(); 1024];

  thread::spawn(move || {
    let (size, src_addr) =  match s.recv_from(&mut buffer) {
      Ok(s) => s,
      Err(e) => {
        eprintln!("Failed to receive data: {}", e);
        return;
      }
    };
    let received_data = unsafe {
      std::slice::from_raw_parts(buffer.as_ptr() as *const u8, size)
    };
    
    let received_text = match std::str::from_utf8(received_data) {
      Ok(s) => s,
      Err(e) => {
        eprintln!("Failed to format data: {}", e);
        return;
      }
    };

    println!("{:?}: {:?}", src_addr, received_text);
  });
}


fn handle_data(data: &str) {
  let response = match data {
    "Sair" => "fecha conexão",
    "Arquivo" => handle_file("path"),
    s => s
  };
}

fn handle_file(path: &str) -> &str {
  "data"
}