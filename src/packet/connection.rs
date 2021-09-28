use crate::packet::Packet;
use futures_util::SinkExt;
use std::io;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpStream;
use tokio::sync::{
    mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
    Mutex,
};
use tokio_stream::{Stream, StreamExt};

use bytes::{Bytes, BytesMut};
use tokio_util::codec::{FramedRead, FramedWrite, LengthDelimitedCodec};

type Decoder = LengthDelimitedCodec;

pub fn spawn_framed(
    read: impl AsyncRead + Unpin + 'static,
    write: impl AsyncWrite + Unpin + 'static,
) -> (UnboundedSender<Bytes>, UnboundedReceiver<BytesMut>) {
    let mut read = FramedRead::new(read, Decoder::new());
    let mut write = FramedWrite::new(write, Decoder::new());
    let (tx_send, mut rx_send) = unbounded_channel();
    let (tx_receive, rx_receiver) = unbounded_channel();
    // Read task
    tokio::task::spawn_local(async move {
        loop {
            match read.try_next().await {
                Ok(Some(bytes)) => tx_receive.send(bytes).expect("Error sending read result"),
                Ok(None) => todo!("No read bytes"),
                Err(err) => {
                    todo!("Handle read error: {:?}", err);
                }
            }
        }
    });
    tokio::task::spawn_local(async move {
        while let Some(bytes) = rx_send.recv().await {
            write.send(bytes).await.expect("Unexpect send bytes error");
        }
    });
    (tx_send, rx_receiver)
}
pub struct Connection {
    sender: UnboundedSender<Vec<u8>>,
    receiver: Mutex<UnboundedReceiver<Vec<u8>>>,
}
impl Connection {
    fn new(sender: UnboundedSender<Vec<u8>>, receiver: UnboundedReceiver<Vec<u8>>) -> Connection {
        Connection {
            sender,
            receiver: Mutex::new(receiver),
        }
    }
}
