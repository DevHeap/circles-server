//! Custom Hyper headers

use hyper;
use hyper::header::Header;
use hyper::header::Raw;

use std::fmt;
use std::ops::Deref;
use std::str;

/// Header to pass ID of an authorized user throughout our services
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UserID(pub String);

impl Header for UserID {
    fn header_name() -> &'static str {
        "UserID"
    }

    fn parse_header(raw: &Raw) -> hyper::error::Result<Self> {
        let raw_header = raw.one().ok_or(hyper::Error::Header)?;
        let raw_header = str::from_utf8(&raw_header)?;
        Ok(UserID(raw_header.to_owned()))
    }

    fn fmt_header(&self, f: &mut hyper::header::Formatter) -> fmt::Result {
        f.fmt_line(&self.0)
    }
}

impl Deref for UserID {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::UserID;
    use hyper::header::Header;
    use hyper::header::Headers;
    use hyper::header::Raw;

    #[test]
    fn headers() {
        let user_id_in = UserID("12345".to_owned());
        let mut headers = Headers::new();
        headers.set(user_id_in.clone());
        let user_id_out = headers.get::<UserID>().unwrap();
        assert_eq!(user_id_in, *user_id_out);
    }

    #[test]
    fn parse_header() {
        let user_id = "12345";
        let raw = Raw::from(user_id);
        let header = UserID::parse_header(&raw).unwrap();
        assert_eq!(user_id, &header.0);
    }
}
