quick_error! {
    #[derive(Debug)]
    pub enum AppError { // FIXME Rename to `AlgorandError`?
        Custom(err: String) {
            from()
            from(err: &str) -> (err.into())
            display("✘ Rust-Algorand lib error: {}", err)
        }
        Ed25519Err(err: ed25519_dalek::ed25519::Error) {
            from()
            display("✘ Ed25519 cryptography error: {}", err)
        }
        Base64DecodeError(err: base64::DecodeError) {
            from()
            display("✘ Base64 decoder error: {}", err)
        }
        IOError(err: std::io::Error) {
            from()
            display("✘ I/O error: {}", err)
        }
        TryFromSliceError(err: std::array::TryFromSliceError) {
            from()
            display("✘ Try from slice error: {}", err)
        }
        RustMessagePackError(err: rmp_serde::encode::Error) {
            from()
            display("✘ Rust message pack error: {}", err)
        }
        InfallibleError(err: std::convert::Infallible) {
            from()
            display("✘ Infallible error: {}", err)
        }
        SerdeJsonError(err: serde_json::Error) {
            from()
            display("✘ Serde json error: {}", err)
        }
    }
}
