use std::{io::Write};

use serde::{Deserialize, Serialize};
use serde_json;
 
pub const HEADER_KEY_LENGTH: &str = "body-length";

pub const MAX_FRAME_LENGTH: usize = 1024 * 4;

pub type Frame = Vec<u8>;

pub fn new_frame(v: Message) -> Frame {
    let body = serde_json::to_vec(&v).unwrap();
    let headers = format!("{}:{}\n", HEADER_KEY_LENGTH, body.len());
    let mut f: Frame = Vec::with_capacity(headers.len() + body.len());

    f.write(headers.as_bytes());
    f.write(&body);

    f
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
    SignUp,
    SignIn,
    SearchOnline,
    SendSingle,
    SendGroup,
    CreateGroup,

    Ok(String),
    UserToken(String),
}

#[derive(Serialize, Deserialize)]
pub struct SignUpReq {
    nickname: String,
    pwd: String,
    email: String,
}

#[derive(Serialize, Deserialize)]
pub struct SignInReq {
    email: String,
    pwd: String,
}

#[derive(Serialize, Deserialize)]
pub struct SearchOnlineReq {
    email: String,
    token: String,
}

#[derive(Serialize, Deserialize)]
pub struct SendSingleReq {
    peer_email: String, // 对端 email
    token: String,
    msg: String,
}

#[derive(Serialize, Deserialize)]
pub struct SendGroupReq {
    group_id: String,
    group_pwd: String,
    token: String,
    msg: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateGroupReq {
    group_pwd: String,
}
