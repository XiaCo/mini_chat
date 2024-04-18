use std::array;
use std::io::{self, Cursor, ErrorKind, Read};
use std::sync::atomic::AtomicU32;

use tokio::io::{AsyncReadExt, AsyncWriteExt, BufWriter};
use tokio::net::TcpStream;

use bytes::{Buf, BufMut, BytesMut};

use crate::{Message};

pub type Frame = Vec<u8>;
static FRAME_ID: AtomicU32 = AtomicU32::new(0);

pub const MAX_FRAME_LENGTH: usize = 1024 * 4;

// Connection.buffer max size
const MAX_BUFFER_LENGTH: usize = 1024 * 8;

// 从 tcp 连接中读取字节流放到 buffer 中
pub struct Connection {
    conn: BufWriter<TcpStream>,

    buffer: BytesMut,

    stream_id: AtomicU32,
}

#[derive(PartialEq)]
pub enum Error {
    Shutdown,

    ParseError,

    Incomplete,

    FrameOverflow,
}

impl<T: std::error::Error> From<T> for Error {
    fn from(_: T) -> Self {
        return Error::ParseError;
    }
}

impl Connection {
    pub fn new(stream: TcpStream) -> Connection {
        Connection {
            conn: BufWriter::new(stream),
            buffer: BytesMut::with_capacity(MAX_BUFFER_LENGTH),
            stream_id: AtomicU32::new(0),
        }
    }

    // 从 buffer 解析包
    // 如果非法（未获得完整包）则继续读取
    // todo 需要有个清空 buffer 机制，需要一个 buffer 最大限额
    pub async fn read_frame(&mut self) -> Result<Message, Error> {
        loop {
            match self.parse_frame() {
                Ok(f) => return Ok(f),
                Err(e) => {
                    if e == Error::ParseError {
                        return Err(e);
                    }
                }
            }

            if let Ok(n) = self.conn.read_buf(&mut self.buffer).await {
                if n == 0 {
                    return Err(Error::Shutdown);
                }
            } else {
                return Err(Error::Shutdown);
            }
        }
    }

    fn parse_frame(&mut self) -> Result<Message, Error> {
        // 读协议头
        let mut buf = Cursor::new(&self.buffer[..]);
        let frame_length = buf.get_u32();
        let stream_id = buf.get_u32();
        if buf.remaining() < frame_length as usize {
            return Err(Error::Incomplete)
        }
        
        let frame_payload = &buf.get_ref()[..frame_length as usize];

        // 读 body 并反序列化
        let msg: Message = serde_json::from_slice(frame_payload)?;

        self.buffer.advance(4+4+frame_length as usize);
        return Ok(msg);
    }

    // 封装通信包，填充字节流长度，帧id
    fn new_frame(&mut self, v: Message) -> Result<Frame, io::Error> {
        let body = serde_json::to_vec(&v).unwrap();
        let mut f: Frame = Vec::with_capacity(8 + body.len());
    
        let n1 = io::Write::write(&mut f, body.len().to_be_bytes().as_slice())?;
        let n2 = io::Write::write(&mut f,self.stream_id.fetch_add(2, std::sync::atomic::Ordering::Relaxed).to_be_bytes().as_slice())?;
        let n3 = io::Write::write(&mut f, &body)?;
        if (n1 + n2 + n3) > MAX_FRAME_LENGTH {
            return Err(io::Error::new(ErrorKind::InvalidData, "too large frame"));
        }
    
        Ok(f)
    }

    pub async fn write_frame(&mut self, msg: Message) -> io::Result<()> {
        let f = self.new_frame(msg)?;
        self.conn.write(&f).await?;

        self.conn.flush().await
    }
}


