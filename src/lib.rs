use num_bigint::BigUint;
use num_traits::One;

#[allow(non_snake_case)]
pub struct Parameters {
    pub G: BigUint,
    pub P: BigUint,
    pub Q: BigUint,
    pub H: BigUint,
}

impl Parameters {
    // y1 = (G ^ x) mod P
    // y2 = (H ^ x) mod P
    pub fn obfuscate(&self, x: &BigUint) -> (BigUint, BigUint) {
        let y1 = self.G.modpow(&x, &self.P);
        let y2 = self.H.modpow(&x, &self.P);

        (y1, y2)
    }

    // s = (k - (c * x)) mod Q (if k >= c * x) else Q - ((c * x) - k mod Q)
    pub fn solve_challenge(&self, k: &BigUint, c: &BigUint, x: &BigUint) -> BigUint {
        let cx = c * x;
        if k >= &cx {
            (k - cx).modpow(&BigUint::one(), &self.Q)
        } else {
            &self.Q - (cx - k).modpow(&BigUint::one(), &self.Q)
        }
    }

    // r1 ⇔ v1 = (((G ^ s) mod P) * ((y1 ^ c) mod P)) mod P
    // r2 ⇔ v2 = (((H ^ s) mod P) * ((y2 ^ c) mod P)) mod P
    pub fn verify(
        &self,
        (y1, y2): (&BigUint, &BigUint),
        (r1, r2): (&BigUint, &BigUint),
        c: &BigUint,
        s: &BigUint,
    ) -> bool {
        let gsp = self.G.modpow(s, &self.P);
        let y1cp = y1.modpow(c, &self.P);
        let v1 = (gsp * y1cp).modpow(&BigUint::one(), &self.P);

        let hsp = self.H.modpow(s, &self.P);
        let y2cp = y2.modpow(c, &self.P);
        let v2 = (hsp * y2cp).modpow(&BigUint::one(), &self.P);

        r1 == &v1 && r2 == &v2
    }
}

pub mod consts {
    use super::*;

    use lazy_static::lazy_static;

    lazy_static! {
        pub static ref PARAMS: Parameters = Parameters {
            // Chosen from Internet Engineering Task Force RFC 3526: https://datatracker.ietf.org/doc/rfc3526
            // === 2048-bit MODP group ===
            G: BigUint::from(2_u8),
            // Flipped to be compatible with BigUint::new()'s endianness
            P: BigUint::new(vec![
                0xFFFFFFFF, 0xFFFFFFFF, 0x8AACAA68, 0x15728E5A, 0x98FA0510, 0x15D22618, 0xEA956AE5, 0x3995497C,
                0x95581718, 0xDE2BCBF6, 0x6F4C52C9, 0xB5C55DF0, 0xEC07A28F, 0x9B2783A2, 0x180E8603, 0xE39E772C,
                0x2E36CE3B, 0x32905E46, 0xCA18217C, 0xF1746C08, 0x4ABC9804, 0x670C354E, 0x7096966D, 0x9ED52907,
                0x208552BB, 0x1C62F356, 0xDCA3AD96, 0x83655D23, 0xFD24CF5F, 0x69163FA8, 0x1C55D39A, 0x98DA4836,
                0xA163BF05, 0xC2007CB8, 0xECE45B3D, 0x49286651, 0x7C4B1FE6, 0xAE9F2411, 0x5A899FA5, 0xEE386BFB,
                0xF406B7ED, 0x0BFF5CB6, 0xA637ED6B, 0xF44C42E9, 0x625E7EC6, 0xE485B576, 0x6D51C245, 0x4FE1356D,
                0xF25F1437, 0x302B0A6D, 0xCD3A431B, 0xEF9519B3, 0x8E3404DD, 0x514A0879, 0x3B139B22, 0x020BBEA6,
                0x8A67CC74, 0x29024E08, 0x80DC1CD1, 0xC4C6628B, 0x2168C234, 0xC90FDAA2, 0xFFFFFFFF, 0xFFFFFFFF,
            ]),
            // === 2048-bit MODP group ===

            // q where q = (p - 1) / 2 and q is a prime
            Q: BigUint::new(vec![
                0xFFFFFFFF, 0x7FFFFFFF, 0x45565534, 0x0AB9472D, 0x4C7D0288, 0x8AE9130C, 0x754AB572, 0x1CCAA4BE,
                0x4AAC0B8C, 0xEF15E5FB, 0x37A62964, 0xDAE2AEF8, 0x7603D147, 0xCD93C1D1, 0x0C074301, 0xF1CF3B96,
                0x171B671D, 0x19482F23, 0x650C10BE, 0x78BA3604, 0x255E4C02, 0xB3861AA7, 0xB84B4B36, 0xCF6A9483,
                0x1042A95D, 0x0E3179AB, 0xEE51D6CB, 0xC1B2AE91, 0x7E9267AF, 0x348B1FD4, 0x0E2AE9CD, 0xCC6D241B,
                0x50B1DF82, 0xE1003E5C, 0xF6722D9E, 0x24943328, 0xBE258FF3, 0xD74F9208, 0xAD44CFD2, 0xF71C35FD,
                0x7A035BF6, 0x85FFAE5B, 0xD31BF6B5, 0x7A262174, 0x312F3F63, 0xF242DABB, 0xB6A8E122, 0xA7F09AB6,
                0xF92F8A1B, 0x98158536, 0xE69D218D, 0xF7CA8CD9, 0xC71A026E, 0x28A5043C, 0x1D89CD91, 0x0105DF53,
                0x4533E63A, 0x94812704, 0xC06E0E68, 0x62633145, 0x10B4611A, 0xE487ED51, 0xFFFFFFFF, 0x7FFFFFFF,
            ]),

            // ƒ = (2 ^ 127) - 1 = 170141183460469231731687303715884105727 (prime)
            // h = (G ^ ƒ) mod P
            H: BigUint::new(vec![
                0x385370ED, 0xC1B26557, 0x5F1A04B2, 0x54D6CCDA, 0x0B9E9E0A, 0xFB6A1CAB, 0x0FD31F78, 0x74E8FF05,
                0x091D0DE4, 0x36C78E3B, 0x1F4BD125, 0x02D27994, 0x8FE002EA, 0x3B3976AF, 0x6D62BB0D, 0xE6753CA0,
                0x62026788, 0x4416C825, 0x30055036, 0x897A1FAD, 0x88C1CC09, 0x626E15D8, 0x0A31A4FE, 0xD56CE35E,
                0x444AA956, 0x288D266E, 0x8973E0C7, 0x846AFF94, 0x5F817F1F, 0xDA255E98, 0xB1F36B26, 0x2999B5DD,
                0xB3D5C81F, 0x2FECB08E, 0x307BA327, 0xB1B58613, 0x24CD4AEC, 0x1A5BD0B2, 0x2D039C2C, 0x99BF0335,
                0x8B7FD66A, 0xA3CE7D7B, 0xA2119B56, 0xDCF89677, 0x928B2EC0, 0x0597717A, 0x707FC0E7, 0x53015271,
                0xA0F485D2, 0x2D9B492B, 0x8CCDA987, 0xEEA3F80D, 0x3B3D8C75, 0x66F489DD, 0x0EA7DA9D, 0xE46AC006,
                0x5C4917F4, 0xC188973E, 0xD127BDC1, 0x51049549, 0x3BFDB0B6, 0x0DB833C1, 0xB537228E, 0x0B4634B1,
            ]),
        };
    }
}

#[rustfmt::skip]
mod zkp_auth;

// Controlled re-export of generated proto code
pub mod proto {
    pub use super::zkp_auth::{
        auth_client::AuthClient,
        auth_server::{Auth, AuthServer},
        AuthenticationAnswerRequest, AuthenticationAnswerResponse, AuthenticationChallengeRequest,
        AuthenticationChallengeResponse, RegisterRequest, RegisterResponse,
    };
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use super::*;
    use num_bigint::BigUint;
    use zkp_utils::{random, string};

    #[test]
    fn secret_obfuscation() {
        let P = BigUint::from(23_u8);
        let Q = BigUint::from(11_u8);
        let G = BigUint::from(4_u8);
        let H = BigUint::from(9_u8);

        let params = Parameters { G, P, Q, H };

        let x = BigUint::from(6_u8);

        let y1 = BigUint::from(2_u8);
        let y2 = BigUint::from(3_u8);

        assert_eq!(params.obfuscate(&x), (y1, y2));
    }

    #[test]
    fn challenge_solution() {
        let P = BigUint::from(23_u8);
        let Q = BigUint::from(11_u8);
        let G = BigUint::from(4_u8);
        let H = BigUint::from(9_u8);

        let params = Parameters { G, P, Q, H };

        let k = BigUint::from(7_u8);
        let c = BigUint::from(4_u8);
        let x = BigUint::from(6_u8);

        assert_eq!(BigUint::from(5_u8), params.solve_challenge(&k, &c, &x))
    }

    #[test]
    fn verification() {
        let P = BigUint::from(23_u8);
        let Q = BigUint::from(11_u8);
        let G = BigUint::from(4_u8);
        let H = BigUint::from(9_u8);

        let params = Parameters { G, P, Q, H };

        let y1 = BigUint::from(2_u8);
        let y2 = BigUint::from(3_u8);
        let r1 = BigUint::from(8_u8);
        let r2 = BigUint::from(4_u8);

        let c = BigUint::from(4_u8);
        let s = BigUint::from(5_u8);

        assert!(params.verify((&y1, &y2), (&r1, &r2), &c, &s));
    }

    #[test]
    fn test_example() {
        // Adopted from https://crypto.stackexchange.com/a/99265/64369
        let P = BigUint::from(23_u8);
        let Q = BigUint::from(11_u8);
        let G = BigUint::from(4_u8);
        let H = BigUint::from(9_u8);

        let params = Parameters { G, P, Q, H };

        let x = BigUint::from(6_u8);

        let (y1, y2) = params.obfuscate(&x);

        let k = BigUint::from(7_u8);
        let (r1, r2) = params.obfuscate(&k);

        let c = BigUint::from(4_u8);
        let s = params.solve_challenge(&k, &c, &x);

        assert!(params.verify((&y1, &y2), (&r1, &r2), &c, &s));
    }

    #[test]
    fn authentication() {
        // Registration
        let x = string::as_biguint("oppenheimer");
        let (y1, y2) = consts::PARAMS.obfuscate(&x);

        // Authentication attempt 1
        let k = random::biguint(&consts::PARAMS.Q);
        let (r1, r2) = consts::PARAMS.obfuscate(&k);

        let c = random::biguint(&consts::PARAMS.Q);
        let s = consts::PARAMS.solve_challenge(&k, &c, &x);

        assert!(consts::PARAMS.verify((&y1, &y2), (&r1, &r2), &c, &s));

        // Authentication attempt 2: Should fail (Incorrect password)
        let x = string::as_biguint("barbie");

        let k = random::biguint(&consts::PARAMS.Q);
        let (r1, r2) = consts::PARAMS.obfuscate(&k);

        let c = random::biguint(&consts::PARAMS.Q);
        let s = consts::PARAMS.solve_challenge(&k, &c, &x);

        assert!(!consts::PARAMS.verify((&y1, &y2), (&r1, &r2), &c, &s));
    }
}
