#[macro_use]
extern crate may;

use std::net::{TcpStream, ToSocketAddrs, Shutdown};
use std::time::Duration;
use crossbeam::sync::WaitGroup;
use std::sync::{Arc, Mutex};
use structopt::StructOpt;


#[derive(Debug, StructOpt)]
#[structopt(name = "dail example", about = "An example of server ports scanning.")]
struct Opt {
    #[structopt(long = "hostname", default_value = "", help = "hostname to test")]
    host_name: String,
    #[structopt(long = "start-port", default_value = "80", help = "the port on which the scanning starts" )]
    start_port: u16,
    #[structopt(long = "end-port", default_value = "100", help = "the port from which the scanning ends" )]
    end_port: u16,
    #[structopt(long = "timeout", default_value = "200", help = "timeout" )]
    timeout: u32,

}
fn main() {
    let opt = Opt::from_args();
    let vec: Vec<u16> = Vec::new();
    let arc_vec = Arc::new(Mutex::new(vec));
    
    // create a new wait group.
    let wg = WaitGroup::new();
    for n in opt.start_port..opt.end_port{
        // create another reference to the wait group.
        let wg = wg.clone();
        let arc_vec = arc_vec.clone();
        let domain = String::from(&opt.host_name);
        go!(move || {
            let flag = is_open(&domain, n);
            if flag {
                arc_vec.lock().unwrap().push(n);
            }
            drop(arc_vec);
            // Drop the reference to the wait group.
            drop(wg);
        });
    }
    // Block until all coroutines have finished their work.
    wg.wait();
    println!("opened ports: {:?}", arc_vec.lock().unwrap());
}

fn is_open(domain: &str, port: u16) -> bool {
    let host = format!("{}:{}", domain, port);
    let addrs: Vec<_> = host 
                            .to_socket_addrs()
                            .expect("Unable to parse socket address")
                            .collect();

    let timeout = Duration::from_millis(200);
    if let Ok(stream) = TcpStream::connect_timeout(&addrs[0], timeout) {
        //println!("Connected to the server!");
        stream.shutdown(Shutdown::Both).expect("shutdown call failed");
        true
    } else {
        //println!("Couldn't connect to server...");
        false
    }

}
