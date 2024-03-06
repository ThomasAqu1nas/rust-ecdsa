#[cfg(test)]
mod tests {    

    use num_bigint::BigInt;
    use state_manager::{Getter, State, StateBuffer, StateManager, StateSetter, error};
    use crate::arithmetic::{Modular, Secp256k1Point};

    pub struct TestStateBuffer;
    pub type BigIntTestData1 = Vec<(BigInt, BigInt, BigInt)>;
    impl StateBuffer for TestStateBuffer{}

    fn setup_gcd() -> (State<BigIntTestData1>, StateSetter<BigIntTestData1>) {
        TestStateBuffer::new_state(Some(
            vec![
                (BigInt::from(1), BigInt::from(1), BigInt::from(1)),
                (BigInt::from(48), BigInt::from(18), BigInt::from(6)),
                (BigInt::from(180), BigInt::from(48), BigInt::from(12)),
                (BigInt::from(270), BigInt::from(192), BigInt::from(6)),
                (BigInt::from(8), BigInt::from(3), BigInt::from(1)),
                (BigInt::from(21), BigInt::from(10), BigInt::from(1)),
                (BigInt::from(180), BigInt::from(48), BigInt::from(12)),
                (BigInt::from(0), BigInt::from(48), BigInt::from(48)),
                (BigInt::from(987654), BigInt::from(123456), BigInt::from(6))
            ]
        ))
    }

    #[test]
    fn test_gcd() {
        let (test_state, _) = setup_gcd();
        for val in test_state.get().unwrap() {
            assert_eq!(Modular::gcd(&val.0, &val.1).0, val.2);
        }
    }

    #[test]
    fn test_invmod() {
        let (test_state, set_test_state) = TestStateBuffer::new_state(Some(
            vec![
                (Some(BigInt::from(3)), Some(BigInt::from(11)), Some(BigInt::from(4))),
                (Some(BigInt::from(10)), Some(BigInt::from(17)), Some(BigInt::from(12))),
                (Some(BigInt::from(2)), Some(BigInt::from(5)), Some(BigInt::from(3))),
            ]
        ));
        for val in test_state.get().unwrap() {
            assert_eq!(val.0.unwrap().invmod(&val.1.unwrap()).unwrap(), val.2.unwrap())
        }
        let _ = set_test_state(Some(vec![(Some(BigInt::from(2)), Some(BigInt::from(4)), None::<BigInt>)]));
        for val in test_state.get().unwrap() {
            assert_eq!(val.0.unwrap().invmod(&val.1.unwrap()), val.2)
        }
    }

    #[test]
    fn test_modulus() {
        let (test_state, _) = TestStateBuffer::new_state(Some(
            vec![
                (Some(BigInt::from(3)), Some(BigInt::from(11)), Some(BigInt::from(3))), // 3 mod 11 = 3
                (Some(BigInt::from(10)), Some(BigInt::from(17)), Some(BigInt::from(10))), // 10 mod 17 = 10
                (Some(BigInt::from(22)), Some(BigInt::from(5)), Some(BigInt::from(2))), // 22 mod 5 = 2
                (Some(BigInt::from(25)), Some(BigInt::from(7)), Some(BigInt::from(4))), // 25 mod 7 = 4
                (Some(BigInt::from(100)), Some(BigInt::from(30)), Some(BigInt::from(10))), // 100 mod 30 = 10
                (Some(BigInt::from(-15)), Some(BigInt::from(4)), Some(BigInt::from(1))), // -15 mod 4 = 1 (зависит от реализации отрицательных чисел)
                (Some(BigInt::from(12345)), Some(BigInt::from(678)), Some(BigInt::from(141))), // 12345 mod 678 = 141
                (Some(BigInt::from(0)), Some(BigInt::from(5)), Some(BigInt::from(0))), // 0 mod 5 = 0
                (Some(BigInt::from(19)), Some(BigInt::from(19)), Some(BigInt::from(0))), // 19 mod 19 = 0
                (Some(BigInt::from(1)), Some(BigInt::from(1)), Some(BigInt::from(0))), // 1 mod 1 = 0
                (Some(BigInt::from(-1)), Some(BigInt::from(2)), Some(BigInt::from(1))), // -1 mod 2 = 1 (или может быть другое значение в зависимости от реализации)
                (Some(BigInt::from(6)), Some(BigInt::from(3)), Some(BigInt::from(0))), // 6 mod 3 = 0
            ]
        ));
        for val in test_state.get().unwrap() {
            assert_eq!(val.0.unwrap().modulus(&val.1.unwrap()), val.2.unwrap());
        }
    }

    #[test]
    #[should_panic(expected = "attempt to divide by zero")]
    fn test_modulus_panic() {
        let (test_state, _) = TestStateBuffer::new_state(Some(
            vec![
                (Some(BigInt::from(3)), Some(BigInt::from(0))),
                (Some(BigInt::from(10)), Some(BigInt::from(0))),
                (Some(BigInt::from(22)), Some(BigInt::from(0))),
                (Some(BigInt::from(25)), Some(BigInt::from(0))),
                (Some(BigInt::from(100)), Some(BigInt::from(0))),
                (Some(BigInt::from(-15)), Some(BigInt::from(0))),
                (Some(BigInt::from(12345)), Some(BigInt::from(0))),
                (Some(BigInt::from(0)), Some(BigInt::from(0))),
                (Some(BigInt::from(19)), Some(BigInt::from(0))),
                (Some(BigInt::from(1)), Some(BigInt::from(0))),
                (Some(BigInt::from(-1)), Some(BigInt::from(0))),
                (Some(BigInt::from(6)), Some(BigInt::from(0))), 
            ]
        ));
        for val in test_state.get().unwrap() {
            val.0.unwrap().modulus(&val.1.unwrap());
        }
    }

    // #[test]
    // fn test_mod() {
    //     let x = BigInt::from(-4);
    //     let y = BigInt::from(13);
    //     println!("mod: ", x.)
    // }

}