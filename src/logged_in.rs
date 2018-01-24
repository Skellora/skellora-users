use user::UserId;

#[derive(Debug, Serialize)]
pub struct LoggedIn {
    user: Option<UserId>,
}
