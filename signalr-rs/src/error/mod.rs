pub struct Error {
    message: String,   
}

impl Error {
    pub fn handshake_error(inner: &str) -> Self {
        Error {
            message: format!("Handshake failed, inner {}", inner),
        }
    }

    pub fn handshake_error_simple() -> Self {
        Error {
            message: format!("Handshake failed")
        }
    }
}