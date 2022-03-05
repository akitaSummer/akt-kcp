#[macro_use]
use log::{trace, error};

use tokio::{net::UdpSocket, sync::mpsc};

use std::{
    io::{ErrorKind, Result as ioResult, Write},
    net::SocketAddr,
    sync::Arc,
};

// 数据包被底层udp接受处理
pub struct KcpOutput {
    socket: Arc<UdpSocket>, // 线程间共享
    target_addr: SocketAddr,
    delay_tx: mpsc::UnboundedSender<Vec<u8>>, // 发送端
}

impl KcpOutput {
    pub fn new(socket: Arc<UdpSocket>, target_addr: SocketAddr) -> KcpOutput {
        let (delay_tx, mut delay_rx) = mpsc::unbounded_channel::<Vec<u8>>();

        let socket_clone = socket.clone();
        tokio::spawn(async move {
            loop {
                // 接收
                match delay_rx.recv().await {
                    Some(buf) => {
                        // 得到数据后向socket中传递数据
                        if let Err(err) = socket_clone.send_to(&buf, target_addr).await {
                            error!("[SEND] UDP delayed send failed, error: {}", err);
                        }
                    }
                    None => {}
                }
            }
        });

        KcpOutput {
            socket,
            target_addr,
            delay_tx,
        }
    }
}

impl Write for KcpOutput {
    fn write(&mut self, buf: &[u8]) -> ioResult<usize> {
        match self.socket.try_send_to(buf, self.target_addr) {
            Ok(n) => Ok(n),
            Err(err) => {
                // 传输中丢失包
                if err.kind() == ErrorKind::WouldBlock {
                    trace!(
                        "[SEND] UDP send EAGAIN, packet.size: {} bytes, delayed send",
                        buf.len()
                    );
                    // 再次发送
                    self.delay_tx
                        .send(buf.to_owned())
                        .expect("channel closed unexpectly");

                    return Ok(buf.len());
                }
                Err(err)
            }
        }
    }

    fn flush(&mut self) -> ioResult<()> {
        Ok(())
    }
}
