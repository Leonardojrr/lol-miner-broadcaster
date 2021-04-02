use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, WebSocketStream};
use tungstenite::Message;
use uuid::Uuid;

pub struct Sender {
    pub id: Uuid,
    pub sink: SplitSink<WebSocketStream<TcpStream>, Message>,
}

pub struct Receiver {
    pub id: Uuid,
    pub stream: SplitStream<WebSocketStream<TcpStream>>,
}

pub struct SenderAdmin {
    senders: Vec<Sender>,
}

impl SenderAdmin {
    pub fn new() -> Self {
        Self { senders: vec![] }
    }

    pub fn push(&mut self, sender: Sender) {
        self.senders.push(sender);
    }

    pub fn remove(&mut self, index: usize) {
        self.senders.remove(index);
    }

    pub async fn broadcast(&mut self, msg: Message) {
        for index in 0..self.senders.len() {
            let _ = self.senders[index].sink.send(msg.clone()).await;
        }
    }
    pub async fn resend(&mut self, msg: Message, sender_index: usize) {
        let _ = self.senders[sender_index].sink.send(msg.clone()).await;
    }
}

pub struct SocketListener {
    listener: TcpListener,
}

impl SocketListener {
    pub async fn new(socket: &str) -> SocketListener {
        let addr = format!("127.0.0.1:{}", socket);

        let listener = TcpListener::bind(addr)
            .await
            .expect("Unable to connect to that socket");

        Self { listener }
    }

    pub async fn listen(&self) -> (Sender, Receiver) {
        let (stream, _) = self.listener.accept().await.unwrap();
        let ws = accept_async(stream).await.unwrap();

        let id = Uuid::new_v4();
        let (sink, stream) = ws.split();

        (Sender { id, sink }, Receiver { id, stream })
    }
}
