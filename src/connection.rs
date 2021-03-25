use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, WebSocketStream};
use tungstenite::Message;

pub type Sender = SplitSink<WebSocketStream<TcpStream>, Message>;

pub type Receiver = SplitStream<WebSocketStream<TcpStream>>;

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

        let (sender, receiver) = ws.split();

        (sender, receiver)
    }
}

pub async fn broadcast(msg: Message, sender_index: usize, senders: &mut Vec<Sender>) {
    if let Message::Text(_) = msg {
        for index in 0..senders.len() {
            if index != sender_index {
                senders[index].send(msg.clone()).await;
            }
        }
    }
}
