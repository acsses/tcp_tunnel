use std::{io::Write, net::{TcpListener,TcpStream}};
use std::{thread, time};

fn main() {
    let ten_millis = time::Duration::from_millis(1000);
    let listener = TcpListener::bind("127.0.0.1:8912")
                                            .expect("failed to listen");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut i = 0;
                let message = "hello\r\n";
                println!("{}",stream.peer_addr().unwrap());

                stream.write(message.as_bytes()).unwrap();


                loop {
                    stream.write(message.as_bytes()).unwrap();
                    i+=1;
                    if i==20{
                        break;
                    }
                    thread::sleep(ten_millis);
                    
                }

            },
            Err(e) => println!("failed to listen:{}",e),
        }
    }
    println!("Hello, world!");
}
