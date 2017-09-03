/// @TODO: tests

use jwt;
use jwt::id_token::{IDToken, IDTokenDecoder};
use base64;
use json;

use auth::firebase_keyring::Keyring;
use auth::{Result, ErrorKind};

use std::ops::Deref;

pub struct Token {
    idtoken: IDToken
}

static FIREBASE_AUDIENCE: &str = "dhcircles-fa776";
static FIREBASE_ISSUER: &str = "https://securetoken.google.com/dhcircles-fa776";

impl Token {
    pub fn decode(token: &str, keyring: &Keyring) -> Result<Token> {
        // Decode and desetialize token keader to retrieve "kid"
        let header = token.split(".").nth(0).ok_or(jwt::Error::JWTInvalid)?;
        let header = base64::decode(header)?;
        let header: TokenHeader = json::from_slice(&header[..])?;

        // Get a Public Key with received "kid" from Google Keyring
        let public_key = keyring.get(&header.kid).ok_or(ErrorKind::UnknownKeyID)?;

        // Construct a decoder that will decode token,
        // verify signature and ISSUES + AUDIENCE
        let decoder = IDTokenDecoder::from_pem(
            public_key,
            FIREBASE_ISSUER,
            FIREBASE_AUDIENCE
        )?;

        // Construct Self object wrapping a decoded idtoken
        let token = Token {
            idtoken: decoder.decode(token)?
        };

        // And then we can verify token's other data correctness
        token.verify_data()?;

        Ok(token)
    }

    pub fn user_id(&self) -> &str {
        self.idtoken.subject_identifier()
    }

    fn verify_data(&self) -> Result<()> {
        let token = &self.idtoken;

        // Check that user_id is not empty
        if token.subject_identifier().is_empty() 
        || token.subject_identifier().chars().all(|c| c.is_whitespace()) 
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
    kid: String
}