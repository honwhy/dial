use std::net::{TcpStream, ToSocketAddrs, Shutdown};
use std::time::Duration;
use crossbeam::sync::WaitGroup;
use std::thread;
use std::sync::{Arc, Mutex};

//#[macro_use]
//extern crate clap;
/*
#[derive(clap)]
#[clap(version = "0.1", author="honwhy wang")]
struct opts {
    #[clap(]
    host_name: string,
    start_port: u16,
    end_port: u16,
}*/
fn main() {
    let vec: Vec<u16> = Vec::new();
    let arc_vec = Arc::new(Mutex::new(vec));
    
    // create a new wait group.
    let wg = WaitGroup::new();
    for n in 80..89 {
        // create another reference to the wait group.
        let wg = wg.clone();
        let arc_vec = arc_vec.clone();
        let domain = String::from("google.com");
        thread::spawn(move || {
            let flag = is_open(&domain, n);
            if flag {
                arc_vec.lock().unwrap().push(n);
            }
            drop(arc_vec);
            // Drop the reference to the wait group.
            drop(wg);
        });
    }
    // Block until all threads have finished their work.
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
