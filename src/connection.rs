use futures_util::{
    stream::{SplitSink, SplitStream},
    StreamExt,
};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, WebSocketStream};
use tungstenite::Message;
use uuid::Uuid;

struct Sender {
    id: Uuid,
    sink: SplitSink<WebSocketStream<TcpStream>, Message>,
}

struct Receiver {
    id: Uuid,
    stream: SplitStream<WebSocketStream<TcpStream>>,
}

pub struct ConnectionEvent((Sender, Receiver));

pub struct SocketListener {
    listener: TcpListener,
}

impl SocketListener {
    pub async fn new(socket: &str) -> SocketListener {
        let addr = format!("127.0.0.1:{}", socket);

        let listener = TcpListener::bind(addr)
            .await
            .expect("Unable to connect to that socket");

        SocketListener { listener }
    }

    pub async fn listen(&self) -> ConnectionEvent {
        let (stream, _) = self.listener.accept().await.unwrap();
        let ws = accept_async(stream).await.unwrap();

        let id = Uuid::new_v4();
        let (sink, stream) = ws.split();

        let (sender, receiver) = (Sender { id, sink }, Receiver { id, stream });
        ConnectionEvent((sender, receiver))
    }
}
