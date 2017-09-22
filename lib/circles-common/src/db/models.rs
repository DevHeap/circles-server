use db::schema::users;
use chrono::NaiveDateTime;
use firebase::Token;

/// User model for the "users" table
#[derive(Queryable, Identifiable, Insertable)]
#[table_name="users"]
#[primary_key(uid)]
pub struct User {
    pub uid: String,
    pub username: Option<String>,
    pub picture: Option<String>,
    pub email: Option<String>,
    pub auth_time: NaiveDateTime,
    pub auth_until: NaiveDateTime
}

/// Auth data changeset: issue time and expiration time
#[derive(AsChangeset)]
#[table_name="users"]
pub struct UserAuthData {
    pub auth_time: NaiveDateTime,
    pub auth_until: NaiveDateTime
}

impl<'a> From<&'a Token> for User {
    fn from(token: &'a Token) -> Self {
        let get_string = |key| {
            token.payload.get(key).and_then(|json| {
                json.as_str().map(str::to_owned)
            })
        };

        User {
            uid:        token.user_id().to_owned(),
            username:   get_string("name"),
            picture:    get_string("picture"),
            email:      get_string("email"),
            auth_time:  NaiveDateTime::from_timestamp(token.issued_at() as i64,       0),
            auth_until: NaiveDateTime::from_timestamp(token.expiration_time() as i64, 0)
        }
    }
}

impl User {
    /// Get AuthData of a User
    pub fn auth_data(&self) -> UserAuthData {
        UserAuthData {
            auth_time: self.auth_time,
            auth_until: self.auth_until
        }
    } 
}