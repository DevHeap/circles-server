//! JWT IDToken decoding and verigication
//! @TODO: tests

use jwt;
use jwt::id_token::IDToken;
use base64;
use json;

use firebase::keyring::Keyring;
use firebase::{Result, ErrorKind};

use std::ops::Deref;

/// Google Firebase Token
pub struct Token {
    idtoken: IDToken,
}

impl Token {
    /// Decode and verify the base64 encode JWT Token using provided Keyring
    pub fn decode(token: &str, keyring: &Keyring) -> Result<Token> {
        // Decode and deserialize token keader to retrieve "kid"
        let header = token.split(".").nth(0).ok_or(jwt::Error::JWTInvalid)?;
        let header = base64::decode(header)?;
        let header: TokenHeader = json::from_slice(&header[..])?;

        // Get a Decoder for received "kid" from Google Keyring
        let decoder = keyring.get(&header.kid).ok_or(ErrorKind::UnknownKeyID)?;

        // Construct Self object wrapping a decoded idtoken
        let token = Token { idtoken: decoder.decode(token)? };

        // And then we can verify token's other data correctness
        token.verify_data()?;

        Ok(token)
    }

    /// Get user unique identifier
    pub fn user_id(&self) -> &str {
        self.idtoken.subject_identifier()
    }

    fn verify_data(&self) -> Result<()> {
        let token = &self.idtoken;

        // Check that user_id is not empty
        if token.subject_identifier().is_empty() ||
            token.subject_identifier().chars().all(
                |c| c.is_whitespace(),
            )
        {
            bail!(ErrorKind::EmptyUserID)
        }

        Ok(())
    }
}

impl Deref for Token {
    type Target = IDToken;
    fn deref(&self) -> &Self::Target {
        &self.idtoken
    }
}

#[derive(Debug, Deserialize)]
struct TokenHeader {
    alg: String,
    kid: String,
}
