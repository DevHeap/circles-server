use reqwest::StatusCode;

// Generate error types boilerplate

error_chain! {
    foreign_links {
        Io(::std::io::Error);
        Json(::json::error::Error);
        Hyper(::hyper::Error);
        Utf8(::std::string::FromUtf8Error);
        RustyJwt(::jwt::Error);
        OpenSSL(::openssl::error::Error);
        OpenSSLStack(::openssl::error::ErrorStack);
        Base64(::base64::DecodeError);
        Reqwest(::reqwest::Error);
    }

    errors {
        FailedToRetrieveKeyring(status: StatusCode) {
            description("failed to retrieve google keyring")
            display("failed to retrieve google keyring: {}", status)
        }

        AuthHeaderMissing {
            description("missing Authorization header")
            display("missing Authorization header")
        }
        
        EmptyUserID {
            description("userid is empty")
            display("userid is empty")
        }

        UnknownKeyID {
            description("unknown key id")
            display("unknown key id")
        }
    }
}