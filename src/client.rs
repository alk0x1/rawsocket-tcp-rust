use socket2::{Socket, Domain, Type};

fn main() {
  let address = "127.0.0.1";
  let port = "8080";
  let server_socket_addr = format!("{}:{}", address, port);

  let socket = Socket::new(Domain::IPV4, Type::STREAM, None).unwrap();

  socket.connect(&server_socket_addr.into());




}