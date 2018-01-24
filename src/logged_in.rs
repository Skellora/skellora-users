use user::UserId;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoggedIn {
    user: Option<UserId>,
}
