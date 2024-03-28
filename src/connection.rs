use tokio::io::{AsyncReadExt, AsyncWriteExt, BufWriter};
use tokio::net::TcpStream;

use bytes::{Buf, BytesMut};

use crate::Frame;

// 从 tcp 连接中读取字节流放到 buffer 中
pub struct Connection {
    conn: BufWriter<TcpStream>,

    buffer: BytesMut,
}

pub enum Error {
    Shutdown,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Connection {
        Connection {
            conn: BufWriter::new(stream),
            buffer: BytesMut::with_capacity(1024),
        }
    }

    // 从 buffer 解析包
    // 如果非法（未获得完整包）则继续读取
    // todo 需要有个清空 buffer 机制，需要一个 buffer 最大限额
    pub async fn read_frame(&mut self) -> Result<Frame, Error> {
        loop {
            if let Some(f) = self.parse_frame() {
                return Ok(f);
            }

            if let Ok(n) = self.conn.read_buf(&mut self.buffer).await {
                if n == 0 {
                    return Err(Error::Shutdown)
                }
            }else {
                return Err(Error::Shutdown)
            }
        }
    }

    fn parse_frame(&mut self) -> Option<Frame> {
        // 读一行

        // 解析 header

        // 读 body 并反序列化
        if self.buffer.len() >= 1 {
            Some(Frame {
                header: vec![],
                body: crate::Message::SignIn,
            })
        } else {
            None
        }
    }
}
