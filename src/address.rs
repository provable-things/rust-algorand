use ed25519_dalek::{Keypair, Signature};
use rand::rngs::OsRng;

#[derive(Debug)]
pub struct Address {
    keypair: Keypair,
}

impl Address {
    pub fn create_random() -> Self {
        Self {
            keypair: Keypair::generate(&mut OsRng {}),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_random_address() {
        Address::create_random();
    }
}
