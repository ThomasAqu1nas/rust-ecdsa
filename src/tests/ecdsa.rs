
#[cfg(test)]
mod tests {
    use crate::ecdsa::{PrivateKey, PublicKey, Signature};

    fn pk_gen() -> PrivateKey {
        PrivateKey::generate()
    }

    #[test]
    fn test_generate_private_key() {
        for _i in 0..100 {
            let pk1 = pk_gen();
            let pk2 = pk_gen();
            assert_ne!(pk1, pk2)
        }
    }

    //паникует, найти ошибку
    #[test]
    fn test_sign_validation() {
        let priv_key = PrivateKey::generate();
        let pub_key = PublicKey::new(&priv_key);
        let msg = "temp msg";
        let sign = Signature::sign_message(msg, &priv_key);
        println!("sign: {:?}", sign);
        let validation = Signature::validate(msg, &pub_key, &sign.0);
        assert!(validation);
    }
}