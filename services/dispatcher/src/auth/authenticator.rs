use auth::firebase_keyring::TokenVerifier;
use auth::Token;
use auth::Error;

use futures_cpupool::CpuPool;
use futures::Future;

use std::sync::Arc;

/// CpuPool driven token authentifier
pub struct Authenticator {
    cpupool: CpuPool,
    verifier: Arc<TokenVerifier>,
}

impl Authenticator {
    pub fn new() -> Self {
        Authenticator {
            cpupool: CpuPool::new_num_cpus(),
            verifier: Arc::new(TokenVerifier::new()),
        }
    }

    /// Asynchronously Verify JWT Token
    pub fn authenticate(&self, token: String) -> impl Future<Item=Token, Error=Error> {
        let verifier = self.verifier.clone();
        self.cpupool.spawn_fn(move || verifier.verify_token(token))
    }
}