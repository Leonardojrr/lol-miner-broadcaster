mod connection;
mod miner;
mod msg_classifier;

use connection::{broadcast, Receiver, Sender, SocketListener};
use futures_util::{future::select_all, stream::*};
use miner::MinerProcess;
use std::sync::Arc;
use tokio::{select, sync::Mutex, task};

type Miners = Arc<Mutex<Vec<MinerProcess>>>;
type Senders = Arc<Mutex<Vec<Sender>>>;

#[tokio::main]
async fn main() {
    let mut miners_processes: Miners = Arc::new(Mutex::new(vec![]));
    let mut senders: Senders = Arc::new(Mutex::new(vec![]));

    // Clients Messages and Connection listener
    let a = task::spawn(async move {
        let mut receivers: Vec<Receiver> = vec![];
        let socket = SocketListener::new("3000").await;

        let (sender, receiver) = socket.listen().await;
        receivers.push(receiver);

        {
            let mut senders_guard = senders.lock().await;
            (*senders_guard).push(sender);
        }

        loop {
            let mut receivers_futures = vec![];
            for receiver in &mut receivers {
                receivers_futures.push(receiver.next());
            }

            select! {
             (sender, receiver) = socket.listen() =>{
                receivers.push(receiver);
                let mut senders_guard = senders.lock().await;
                (*senders_guard).push(sender);
             }

             (msg,sender_index,_) = select_all(receivers_futures) =>{


                let msg = msg.unwrap().unwrap();
                let mut senders_guard = senders.lock().await;
                broadcast(msg,sender_index,&mut (*senders_guard)).await;
             }
            }
        }
    });

    a.await;
}
