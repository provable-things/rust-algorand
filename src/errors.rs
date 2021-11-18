quick_error! {
    #[derive(Debug)]
    pub enum AppError {
        Custom(err: String) {
            from()
            from(err: &str) -> (err.into())
            display("✘ Rust-Algorand lib error: {}", err)
        }
        Ed25519Err(err: ed25519_dalek::ed25519::Error) {
            from()
            display("✘ Ed25519 cryptography error: {}", err)
        }
    }
}
