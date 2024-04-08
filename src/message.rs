use serde::{Deserialize, Serialize};

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

pub trait Handle {
    fn sign_up(req: SignUpReq) -> Message; // UserToken
    fn sign_in(req: SignInReq) -> Message; // UserToken
    fn search_online(req: SearchOnlineReq) -> Message; // Ok
    fn send_single(req: SendSingleReq) -> Message; // Ok
    fn send_group(req: SendGroupReq) -> Message; // Ok
    fn create_group(req: CreateGroupReq) -> Message; // Ok
}
