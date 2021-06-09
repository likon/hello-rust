/*
extern crate futures;
extern crate tokio;
extern crate tokio_uds;
extern crate tempdir;

use tokio_uds::*;
use tokio::io;
use tokio::runtime::current_thread::Runtime;
use futures::{Future,Stream};
use futures::sync::oneshot;
use tempdir::TempDir;

macro_rules! t {
    ($e:expr) => (match $e {
        Ok(e) => e,
        Err(e) => panic!("{} failed with {:?}", stringify!($e), e),
    })
}

fn main(){
    let dir = TempDir::new("tokio-uds-tests").unwrap();
    let sock_path = dir.path().join("connect.sock");
    let mut rt = Runtime::new().unwrap();

    let server = t!(UnixListener::bind(&sock_path));
    let (tx, rx) = oneshot::channel();

    rt.spawn({
        server.incoming()
            .into_future()
            .and_then(move |(sock, _)|{
                tx.send(sock.unwrap()).unwrap();
                Ok(())
            })
            .map_err(|e| panic!("err={:?}", e))
    });

    let client = rt.block_on(UnixStream::connect(&sock_path)).unwrap();
    let server = rt.block_on(rx).unwrap();

    rt.block_on(io::write_all(client, b"hello")).unwrap();
    let (_,buf) = rt.block_on(io::read_to_end(server, vec![])).unwrap();
    println!("{:?}", buf);
}
*/

/*
extern crate pcap;
extern crate futures;
extern crate tokio_core;

fn main(){
    let mut cap = pcap::Device::lookup().unwrap().open().unwrap();

    println!("startup.");

    loop{
        println!("{:?}", cap.next().unwrap().len());
    }
}
*/

use futures::stream::Stream;
use pcap::tokio::PacketCodec;
use pcap::{Capture, Device, Error, Packet};
use tokio_core::reactor::Core;

pub struct SimpleDumpCodec;

impl PacketCodec for SimpleDumpCodec {
    type Type = String;

    fn decode<'p>(&mut self, packet: Packet<'p>) -> Result<Self::Type, Error> {
        Ok(format!("{:?}", packet))
    }
}

fn ma1n() -> Result<(), Error> {
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let cap = Capture::from_device(Device::lookup()?)?
        .open()?
        .setnonblock()?;
    let s = cap.stream(&handle, SimpleDumpCodec {})?;
    println!("startup.");
    let done = s.for_each(move |s| {
        println!("{:?}", s.len());
        Ok(())
    });
    core.run(done).unwrap();
    Ok(())
}

fn main() {
    match ma1n() {
        Ok(()) => (),
        Err(e) => println!("{:?}", e),
    }
}

/*
use std::io::{BufRead, BufReader};
use std::os::unix::net::{UnixStream,UnixListener};
use std::thread;

fn handle_client(stream: UnixStream) {
    let stream = BufReader::new(stream);
    for line in stream.lines() {
        println!("{}", line.unwrap());
    }
}

fn main() {
    let listener = UnixListener::bind("/tmp/rust-uds.sock").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_client(stream));
            }
            Err(err) => {
                println!("Error: {}", err);
                break;
            }
        }
    }
}
*/
