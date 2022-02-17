/*
    This file is part of Curv library
    Copyright 2018 by Kzen Networks
    (https://github.com/KZen-networks/curv)
    License MIT: https://github.com/KZen-networks/curv/blob/master/LICENSE
*/

use super::traits::Hash;
use crate::curv::arithmetic::traits::Converter;
use crate::curv::elliptic::curves::traits::{ECPoint, ECScalar};
use crate::curv::BigInt;
use crate::curv::{FE, GE};
use ring::digest::{Context, SHA512};

pub struct HSha512;

impl Hash for HSha512 {
    fn create_hash(big_ints: &[&BigInt]) -> BigInt {
        let mut digest = Context::new(&SHA512);

        for value in big_ints {
            digest.update(&BigInt::to_vec(value));
        }

        BigInt::from(digest.finish().as_ref())
    }

    fn create_hash_from_ge(ge_vec: &[&GE]) -> FE {
        let mut digest = Context::new(&SHA512);

        for value in ge_vec {
            digest.update(&value.pk_to_key_slice());
        }

        let result = BigInt::from(digest.finish().as_ref());
        ECScalar::from(&result)
    }
}

#[cfg(test)]
mod tests {
    use super::HSha512;
    use super::Hash;
    use crate::curv::elliptic::curves::traits::ECPoint;
    use crate::curv::elliptic::curves::traits::ECScalar;
    use crate::curv::BigInt;
    use crate::curv::GE;

    #[test]
    // Very basic test here, TODO: suggest better testing
    fn create_hash_test() {
        HSha512::create_hash(&[]);

        let result = HSha512::create_hash(&[&BigInt::one(), &BigInt::zero()]);
        assert!(result > BigInt::zero());
    }

    #[test]
    fn create_hash_from_ge_test() {
        let point = GE::base_point2();
        let result1 = HSha512::create_hash_from_ge(&[&point, &GE::generator()]);
        assert!(result1.to_big_int().to_str_radix(2).len() > 240);
        let result2 = HSha512::create_hash_from_ge(&[&GE::generator(), &point]);
        assert_ne!(result1, result2);
        let result3 = HSha512::create_hash_from_ge(&[&GE::generator(), &point]);
        assert_eq!(result2, result3);
    }
}