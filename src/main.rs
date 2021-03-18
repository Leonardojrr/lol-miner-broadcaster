use futures_util::{stream::SplitSink, SinkExt, StreamExt};
use regex::Regex;
use std::{
    process::{Child, Command},
    sync::{Arc, Mutex},
};
use sysinfo::{ProcessExt, System, SystemExt};
use tokio::{
    net::{TcpListener, TcpStream},
    task,
};

use tokio_tungstenite::{accept_async, WebSocketStream};
use tungstenite::Message;

type Client = SplitSink<WebSocketStream<TcpStream>, Message>;

#[derive(Debug)]
struct Miner {
    region: String,
    process: Child,
}

struct MinerInfo {
    memory: u64,
    cpu: f32,
}

enum WsEvent {
    StartMiner(String),
    CloseAll,
    CloseMiner(String),
}

#[derive(Debug)]
struct WsEventError;

fn classify_msg(msg: String) -> Result<WsEvent, WsEventError> {
    let mut split_message = msg.as_str().split(":");

    let regions_regex = Regex::new("^(br1|eun1|euw1|jp1|kr|la1|la2|na1|oc1|tr1|ru)$").unwrap();

    match split_message.next() {
        Some(slice) => match slice {
            "start" => match split_message.next() {
                Some(slice) => {
                    if regions_regex.is_match(slice) {
                        Ok(WsEvent::StartMiner(slice.to_owned()))
                    } else {
                        Err(WsEventError)
                    }
                }
                None => Err(WsEventError),
            },
            "close" => match split_message.next() {
                Some(slice) => match slice {
                    "all" => Ok(WsEvent::CloseAll),
                    _ => {
                        if regions_regex.is_match(slice) {
                            Ok(WsEvent::CloseMiner(slice.to_owned()))
                        } else {
                            Err(WsEventError)
                        }
                    }
                },
                None => Err(WsEventError),
            },
            _ => Err(WsEventError),
        },
        None => Err(WsEventError),
    }
}

#[tokio::main]
async fn main() {
    let clients_connections: Arc<Mutex<Vec<Client>>> = Arc::new(Mutex::new(vec![]));
    let opened_miners: Arc<Mutex<Vec<Miner>>> = Arc::new(Mutex::new(vec![]));

    //Atomic reference of clients list and minners
    let clients_reference = clients_connections.clone();
    let miners_reference = opened_miners.clone();

    // Connections listener
    let handler = task::spawn(async move {
        const socket_addr: &str = "127.0.0.1:3000";

        let listener = TcpListener::bind(socket_addr)
            .await
            .expect(&format!("Unable to connect to socket: {}", socket_addr));

        while let (stream, addr) = listener.accept().await.unwrap() {
            println!("Someone connect!!");

            //An atomic reference of clients and miners per connection
            let clients = clients_reference.clone();
            let miners = miners_reference.clone();

            //A thread per connection
            let _ = task::spawn(async move {
                //Accept handshake
                let ws = accept_async(stream)
                    .await
                    .expect("The handshake couldn't be completed");

                let (mut sender, mut receiver) = ws.split();

                //Push add connection to the clients list
                {
                    let mut clients_guard = clients.lock().unwrap();
                    let _ = (*clients_guard).push(sender);
                }

                loop {
                    match receiver.next().await {
                        Some(msg) => {
                            let msg = msg.unwrap();
                            if let Message::Text(string) = msg {
                                let mut miners_guard = miners.lock().unwrap();
                                match classify_msg(string).unwrap() {
                                    WsEvent::StartMiner(region) => {
                                        let process = Command::new("lol-project").spawn().unwrap();

                                        (*miners_guard).push(Miner { region, process });
                                    }
                                    WsEvent::CloseMiner(region) => {
                                        let lenght = (*miners_guard).len();

                                        for i in (0..lenght) {
                                            if (*miners_guard)[i].region == region {
                                                (*miners_guard).remove(i);
                                            }
                                        }
                                    }
                                    WsEvent::CloseAll => {
                                        (*miners_guard).clear();
                                    }
                                }
                            };
                        }
                        None => {}
                    }
                }
            });
        }
    });

    let mut sys = System::new();

    loop {
        sys.refresh_all();
        let miners = opened_miners.lock().unwrap();
        let mut clients = clients_connections.lock().unwrap();

        for miner in &(*miners) {
            let sys_process = sys.get_process(miner.process.id() as usize).unwrap();
            let miner_info = MinerInfo {
                memory: sys_process.memory(),
                cpu: sys_process.cpu_usage(),
            };

            for client in &mut (*clients) {
                client.send(Message::text("xd"));
            }
        }
    }
}
