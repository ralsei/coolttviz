use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{self, TryRecvError};
use std::thread::{self, JoinHandle};

use crate::messages::Message;
use crate::syntax::Node;

// FIXME: We should make sure that we shut things down properly
#[allow(dead_code)]
pub struct Server {
    port: u32,
    server_thread: JoinHandle<()>,
    rx: mpsc::Receiver<Message>,
}

impl Server {
    pub fn init(port: u32) -> Server {
        let (tx, rx) = mpsc::channel();

        let server_thread = thread::spawn(move || {
            let listener = TcpListener::bind(format!("127.0.0.1:{}", port))
                .expect("Failed to initialize server");
            println!("[INFO] Coolttviz started, awaiting connections");
            for stream in listener.incoming() {
                let mut stream = stream.expect("Failed to accept");
                println!("[INFO] Connected");
                let mut str = String::new();
                match stream.read_to_string(&mut str) {
                    Result::Ok(0) => (),
                    Result::Ok(_) => match serde_json::from_str(&str) {
                        Result::Ok(msg) => tx.send(msg).expect("Could not send message."),
                        Result::Err(err) => println!("Deserialization Error: {:?}", err),
                    },
                    Result::Err(err) => println!("Read Error: {:?}", err),
                }
            }
        });

        Server { port, server_thread, rx }
    }

    pub fn send(&self, cs: Node) {
        let stream = TcpStream::connect(format!("127.0.0.1:{}", self.port)).expect("oh no 1");
        serde_json::to_writer(stream, &cs).expect("oh no 2");
    }

    pub fn poll(&self) -> Option<Message> {
        match self.rx.try_recv() {
            Ok(msg) => Some(msg),
            Err(TryRecvError::Empty) => None,
            Err(TryRecvError::Disconnected) => panic!("channel disconnected!"),
        }
    }
}
