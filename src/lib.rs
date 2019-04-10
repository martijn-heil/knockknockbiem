use std::net::TcpStream;
use std::net::ToSocketAddrs;
use std::thread::sleep;
use std::time::Duration;

pub fn knock(to: &[&ToSocketAddrs], delay: u64) {
  for addr in to {
    TcpStream::connect(addr);
    sleep(Duration::from_secs(delay));
  }
}