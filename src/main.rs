mod connection;
mod miner;
mod msg_classifier;

use connection::{Receiver, SenderAdmin, SocketListener};
use futures_util::{future::select_all, stream::StreamExt};
use miner::{affect_minners, MinerProcess};
use msg_classifier::{classify_msg, Event, MessageError};
use std::sync::Arc;
use tokio::{select, sync::Mutex, task};
use tungstenite::Message;

type Miners = Arc<Mutex<Vec<MinerProcess>>>;

#[tokio::main]
async fn main() {
    let miners_processes: Miners = Arc::new(Mutex::new(vec![]));
    let senders = Arc::new(Mutex::new(SenderAdmin::new()));

    // Clients Messages and Connection listener
    let a = task::spawn(async move {
        let mut receivers: Vec<Receiver> = vec![];
        let socket = SocketListener::new("3002").await;

        loop {
            if receivers.len() == 0 {
                let (sender, receiver) = socket.listen().await;
                receivers.push(receiver);
                let mut senders_guard = senders.lock().await;
                (*senders_guard).push(sender);
            }

            let mut receivers_futures = vec![];
            for receiver in &mut receivers {
                receivers_futures.push(receiver.stream.next());
            }

            select! {
             (sender, receiver) = socket.listen() =>{
                receivers.push(receiver);
                let mut senders_guard = senders.lock().await;
                (*senders_guard).push(sender);
             }


             (msg, sender_index, _) = select_all(receivers_futures) =>{


                let msg = msg.unwrap().unwrap();
                let event = classify_msg(msg.clone().into_text().unwrap());
                let mut senders_guard = senders.lock().await;

                match event{
                    Ok(e)=>{
                        match e {
                            Event::MinerEvent(e)=>{
                                let mut miners_processes_guard = miners_processes.lock().await;
                                match affect_minners(e, &mut (*miners_processes_guard)){
                                    Ok(())=>{
                                        senders_guard.broadcast(msg).await;
                                    }
                                    Err(_)=>{
                                        let err = Message::text(String::from("Good request, but something went wrong"));
                                        senders_guard.resend(err, sender_index).await;
                                    }
                                }
                            }
                            Event::ConnectionClose =>{
                                (*senders_guard).remove(sender_index);
                                receivers.remove(sender_index);
                            }
                        }
                    },
                    Err(e)=>{
                        match e {
                            MessageError::CommandError(_)=>{
                                let err = Message::text(String::from("Bad request"));
                                senders_guard.resend(err, sender_index).await;
                            }
                            MessageError::RegionError(_)=>{
                                let err = Message::text(String::from("Bad request"));
                                senders_guard.resend(err, sender_index).await;
                            }
                            MessageError::RegionNotSpecified =>{
                                let err = Message::text(String::from("Bad request"));
                                senders_guard.resend(err, sender_index).await;
                            }
                            MessageError::EmptyMsg=>{
                                let err = Message::text(String::from("Bad request"));
                                senders_guard.resend(err, sender_index).await;
                            }
                        }
                    }
                }
             }
            }
        }
    });

    a.await;
}
