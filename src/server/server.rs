pub trait ServerHandle {
    fn sign_up(req: SignUpReq) -> Message::UserToken;
    fn sign_in(req: SignInReq) -> Message::UserToken;
    fn search_online(req: SearchOnlineReq) -> Message::Ok;
    fn send_single(req: SendSingleReq) -> Message::Ok;
    fn send_group(req: SendGroupReq) -> Message::Ok;
    fn create_group(req: CreateGroupReq) -> Message::Ok;
}
