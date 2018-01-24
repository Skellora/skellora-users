use user::UserId;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoggedIn {
    pub user: Option<UserId>,
}
