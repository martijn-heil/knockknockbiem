use std::net::TcpStream;
use std::net::ToSocketAddrs;
use std::thread::sleep;
use std::time::Duration;

pub fn knock<A>(to: A, delay: u64) where A: IntoIterator, A::Item : ToSocketAddrs {
  for addr in to {
    TcpStream::connect(addr);
    sleep(Duration::from_secs(delay));
  }
}