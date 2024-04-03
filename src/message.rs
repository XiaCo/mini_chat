use std::io::{Error, ErrorKind, Write};

use serde::{Deserialize, Serialize};
use serde_json;

pub const HEADER_KEY_LENGTH: &str = "body-length";

pub const MAX_FRAME_LENGTH: usize = 1024 * 4;

pub type Frame = Vec<u8>;

pub fn new_frame(v: Message) -> Result<Frame, Error> {
    let body = serde_json::to_vec(&v).unwrap();
    let headers = format!("{}:{}\n", HEADER_KEY_LENGTH, body.len());
    let mut f: Frame = Vec::with_capacity(headers.len() + body.len());

    let n1 = f.write(headers.as_bytes())?;
    let n2 = f.write(&body)?;
    if (n1 + n2) > MAX_FRAME_LENGTH {
        return Err(Error::new(ErrorKind::InvalidData, "too large frame"));
    }

    Ok(f)
}

/*
注册
登陆
搜索其他在线用户
单聊消息
群聊消息
创建群聊
*/
#[derive(Serialize, Deserialize)]
pub enum Message {
    SignUp(SignUpReq),
    SignIn(SignInReq),
    SearchOnline(SearchOnlineReq),
    SendSingle(SendSingleReq),
    SendGroup(SendGroupReq),
    CreateGroup(CreateGroupReq),

    Ok(String),
    UserToken(String),
}

#[derive(Serialize, Deserialize)]
pub struct SignUpReq {
    pub nickname: String,
    pub pwd: String,
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct SignInReq {
    pub email: String,
    pub pwd: String,
}

#[derive(Serialize, Deserialize)]
pub struct SearchOnlineReq {
    pub email: String,
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct SendSingleReq {
    pub peer_email: String, // 对端 email
    pub token: String,
    pub msg: String,
}

#[derive(Serialize, Deserialize)]
pub struct SendGroupReq {
    pub group_id: String,
    pub group_pwd: String,
    pub token: String,
    pub msg: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateGroupReq {
    pub group_pwd: String,
}

pub trait ServerHandle {
    fn sign_up(req: SignUpReq) -> Message; // UserToken
    fn sign_in(req: SignInReq) -> Message; // UserToken
    fn search_online(req: SearchOnlineReq) -> Message; // Ok
    fn send_single(req: SendSingleReq) -> Message; // Ok
    fn send_group(req: SendGroupReq) -> Message; // Ok
    fn create_group(req: CreateGroupReq) -> Message; // Ok
}
