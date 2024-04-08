use std::io::{self, Cursor, ErrorKind};
use std::sync::atomic::AtomicU32;

use tokio::io::{AsyncReadExt, AsyncWriteExt, BufWriter};
use tokio::net::TcpStream;

use bytes::{Buf, BytesMut};

use crate::{Message};

pub type Frame = Vec<u8>;
static FRAME_ID: AtomicU32 = AtomicU32::new(0);

pub const MAX_FRAME_LENGTH: usize = 1024 * 4;
pub const HEADER_KEY_LENGTH: &str = "body-length";
pub const HEADER_FRAME_ID: &str = "frame-id";

// Connection.buffer max size
const MAX_BUFFER_LENGTH: usize = 1024 * 8;

// 从 tcp 连接中读取字节流放到 buffer 中
pub struct Connection {
    conn: BufWriter<TcpStream>,

    buffer: BytesMut,
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
        // 读一行
        let buf = Cursor::new(&self.buffer[..]);
        let buf_length = buf.get_ref().len();
        let line_u8 = Connection::get_line(buf, MAX_FRAME_LENGTH)?;
        let line = String::from_utf8_lossy(line_u8);

        // 解析 header
        let headers: Vec<&str> = line.split(' ').collect();
        let mut body_length: usize = 0;
        for i in 0..headers.len() {
            let item = headers[i];
            if item == HEADER_KEY_LENGTH && i < headers.len() - 1 {
                body_length = headers[i - 1].parse()?
            }
        }

        // 读 body 并反序列化
        let frame_length = line_u8.len() + body_length; // header + body
        if body_length > 0 && buf_length > frame_length {
            let body = &self.buffer[line_u8.len()..body_length];
            let msg: Message = serde_json::from_slice(body)?;

            self.buffer.advance(frame_length);
            return Ok(msg);
        }

        Err(Error::Incomplete)
    }

    fn get_line(src: Cursor<&[u8]>, max_read: usize) -> Result<&[u8], Error> {
        for i in 0..src.get_ref().len() {
            if i == max_read {
                return Err(Error::FrameOverflow);
            }
            if src.get_ref()[i] == b'\n' {
                return Ok(&src.get_ref()[..i]);
            }
        }

        Err(Error::Incomplete)
    }

    // 封装通信包，填充字节流长度，帧id
    fn new_frame(v: Message) -> Result<Frame, io::Error> {
        let body = serde_json::to_vec(&v).unwrap();
        let headers = format!(
            "{}:{} {}:{}\n",
            HEADER_KEY_LENGTH,
            body.len(),
            HEADER_FRAME_ID,
            FRAME_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed), // 自增 id
        );
        let mut f: Frame = Vec::with_capacity(headers.len() + body.len());
    
        let n1 = io::Write::write(&mut f, headers.as_bytes())?;
        let n2 = io::Write::write(&mut f, &body)?;
        if (n1 + n2) > MAX_FRAME_LENGTH {
            return Err(io::Error::new(ErrorKind::InvalidData, "too large frame"));
        }
    
        Ok(f)
    }

    pub async fn write_frame(&mut self, msg: Message) -> io::Result<()> {
        let f = Connection::new_frame(msg)?;
        self.conn.write(&f).await?;

        self.conn.flush().await
    }
}


