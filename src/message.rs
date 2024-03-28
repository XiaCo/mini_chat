pub struct Frame {
    pub header: Vec<(String, String)>, // [ (k1,v1), (k2,v2) ... ]
    pub body: Message,
}

/*
注册
登陆
搜索其他在线用户
单聊消息
群聊消息
创建群聊
*/
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

pub struct SignUpReq {
    nickname: String,
    pwd: String,
    email: String,
}

pub struct SignInReq {
    email: String,
    pwd: String,
}

pub struct SearchOnlineReq {
    email: String,
    token: String,
}

pub struct SendSingleReq {
    peer_email: String, // 对端 email
    token: String,
    msg: String,
}

pub struct SendGroupReq {
    group_id: String,
    group_pwd: String,
    token: String,
    msg: String,
}

pub struct CreateGroupReq {
    group_pwd: String,
}
