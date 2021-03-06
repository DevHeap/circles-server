
//! Token Verifiers built with Google Keyring

use firebase::{Result, Error, ErrorKind};
use firebase::Token;

use json;
use jwt::id_token::IDTokenDecoder;
use openssl::x509::X509;
use reqwest;
use reqwest::StatusCode;

use std::borrow::Cow;
use std::collections::BTreeMap;
use std::io::Read;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

static GOOGLE_APIS_SECURE_TOKEN_URI: &str = "https://www.googleapis.com/robot/v1/metadata/x509/securetoken@system.gserviceaccount.com";

// secs
static KEYRING_RELOAD_REPIOD: u64 = 1200;
static KEYRING_LOAD_RETRY_PERIOD: u64 = 5;

static FIREBASE_AUDIENCE: &str = "dhcircles-fa776";
static FIREBASE_ISSUER: &str = "https://securetoken.google.com/dhcircles-fa776";

/// ID;Key pairs from Google API
/// @NOTE: Google rotates those keys in some period.
///        Need to check if this can cause any pain in the ass with clients authorized before rotation
///        Then, solution might be to store 2 keyrings: current and previous one
pub struct Keyring {
    keys: BTreeMap<KeyID, Decoder>,
    last_update: SystemTime,
}

/// Google Keyring key identifier
pub type KeyID = String;
/// Token decoder build with one of Public Keys from Google Keyring
pub type Decoder = IDTokenDecoder;

impl Keyring {
    /// Constructs an empty Keyring
    pub fn new() -> Self {
        Keyring {
            keys: BTreeMap::new(),
            last_update: UNIX_EPOCH,
        }
    }

    /// Constructs a keyring from a kid:pkey map
    pub fn with_keys(keys: BTreeMap<KeyID, Decoder>) -> Self {
        Keyring {
            keys,
            last_update: SystemTime::now(),
        }
    }

    /// Constructs a keyring from JSON with kid:cert pairs from Google
    /// Converts certs to public keys and created an IDTokenDecoders
    pub fn from_json(json: &str) -> Result<Self> {
        let raw: BTreeMap<KeyID, String> = json::from_str(json)?;

        let mut keys = BTreeMap::new();
        for (kid, cert_pem) in raw {
            let cert_pem = cert_pem.replace("\\n", "\n");
            let x509 = X509::from_pem(cert_pem.as_bytes())?;
            let pkey = x509.public_key()?;
            let decoder = IDTokenDecoder::from_key(pkey, FIREBASE_ISSUER, FIREBASE_AUDIENCE);
            keys.insert(kid, decoder);
        }

        Ok(Keyring::with_keys(keys))
    }

    /// Get token decoder built with key of id `kid`
    pub fn get(&self, kid: &str) -> Option<&Decoder> {
        self.keys.get(kid)
    }
}

/// TokenVerifier maintains Google Keyring up to date
/// and verifies tokens using the keyring
///
/// @TODO avoid lag at server startup when any request ends up unauthorized
///       because the Keyring haven't been loaded yet (or is it really a problem?)
pub struct TokenVerifier {
    keyring: Arc<RwLock<Keyring>>,
}

impl TokenVerifier {
    /// Constructs a TokenVerifier with an automatic Keyring updater thread
    pub fn new() -> Self {
        // Create an empty keyring wrapped in:
        // RwLock -- for multithreaded read/write locking
        // Arc -- for sharing an onject between threads
        let keyring = Arc::new(RwLock::new(Keyring::new()));
        let keyring_c = keyring.clone();

        // Spawn keyring update task
        thread::spawn(|| Self::keyring_update_task(keyring_c));

        TokenVerifier { keyring }
    }

    /// Decode and verify a Firebase IDToken
    pub fn verify_token<T>(&self, token: T) -> Result<Token>
    where
        T: Into<Cow<'static, str>>,
    {
        Token::decode(&token.into(), &*self.keyring.read().unwrap())
    }

    fn keyring_update_task(keyring: Arc<RwLock<Keyring>>) {
        info!("Starting up keys updater thread");

        loop {
            // We do not want to update keyring if it was just created
            // (mainly for testing purposes)
            let need_update = {
                let keyring = keyring.read().unwrap();
                match keyring.last_update.elapsed() {
                    Ok(duration) => duration > Duration::from_secs(KEYRING_RELOAD_REPIOD),
                    Err(error) => {
                        warn!(
                            "SystemTime is in the past by {} seconds",
                            error.duration().as_secs()
                        );
                        true
                    }  
                }
            };

            if need_update {
                // Wrapping code into a closure here so we can use a '?' sign
                // and handle all errors later on
                let updated = || -> Result<()> {
                    let mut resp = reqwest::get(GOOGLE_APIS_SECURE_TOKEN_URI)?;
                    if resp.status() != StatusCode::Ok {
                        bail!(ErrorKind::FailedToRetrieveKeyring(resp.status()));
                    }

                    let mut buffer = String::new();
                    resp.read_to_string(&mut buffer)?;

                    let new_keyring = Keyring::from_json(&buffer)?;
                    *keyring.write().unwrap() = new_keyring;

                    Ok(())
                };

                match updated() {
                    Ok(_) => info!("Updated google keyring"),
                    Err(e) => {
                        error!("Failed to update google keyring: {}\n", e);
                        // Wait a few seconds and try again
                        thread::sleep(Duration::from_secs(KEYRING_LOAD_RETRY_PERIOD));
                        continue;
                    }
                }
            }

            // @TODO check if 20-min timeout between updates is OK
            thread::sleep(Duration::from_secs(KEYRING_RELOAD_REPIOD));
        }
    }
}

use futures::Future;
use futures_cpupool::CpuPool;

/// CpuPool driven token authentifier
pub struct AsyncTokenVerifier {
    cpupool: CpuPool,
    verifier: Arc<TokenVerifier>,
}

impl AsyncTokenVerifier {
    /// Constructs an AsyncTokenVerifier with a TokenVerifier
    /// and a CpuPool to run async verification tasks
    pub fn new() -> Self {
        AsyncTokenVerifier {
            cpupool: CpuPool::new_num_cpus(),
            verifier: Arc::new(TokenVerifier::new()),
        }
    }

    /// Asynchronously Verify JWT Token
    pub fn authenticate(&self, token: String) -> impl Future<Item = Token, Error = Error> {
        let verifier = self.verifier.clone();
        self.cpupool.spawn_fn(move || verifier.verify_token(token))
    }
}
