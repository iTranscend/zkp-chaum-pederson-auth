use num_bigint::{BigInt, RandBigInt, Sign};
use num_traits::{One, Zero};

// chosen from  RFC 3526: https://datatracker.ietf.org/doc/rfc3526/?include_text=1
// 2048-bit MODP group
// Reversed to fit BigInt::new() endianness
const P_VEC: [u32; 64] = [
    0xFFFFFFFF, 0xFFFFFFFF, 0x8AACAA68, 0x15728E5A, 0x98FA0510, 0x15D22618, 0xEA956AE5, 0x3995497C,
    0x95581718, 0xDE2BCBF6, 0x6F4C52C9, 0xB5C55DF0, 0xEC07A28F, 0x9B2783A2, 0x180E8603, 0xE39E772C,
    0x2E36CE3B, 0x32905E46, 0xCA18217C, 0xF1746C08, 0x4ABC9804, 0x670C354E, 0x7096966D, 0x9ED52907,
    0x208552BB, 0x1C62F356, 0xDCA3AD96, 0x83655D23, 0xFD24CF5F, 0x69163FA8, 0x1C55D39A, 0x98DA4836,
    0xA163BF05, 0xC2007CB8, 0xECE45B3D, 0x49286651, 0x7C4B1FE6, 0xAE9F2411, 0x5A899FA5, 0xEE386BFB,
    0xF406B7ED, 0x0BFF5CB6, 0xA637ED6B, 0xF44C42E9, 0x625E7EC6, 0xE485B576, 0x6D51C245, 0x4FE1356D,
    0xF25F1437, 0x302B0A6D, 0xCD3A431B, 0xEF9519B3, 0x8E3404DD, 0x514A0879, 0x3B139B22, 0x20BBEA6,
    0x8A67CC74, 0x29024E08, 0x80DC1CD1, 0xC4C6628B, 0x2168C234, 0xC90FDAA2, 0xFFFFFFFF, 0xFFFFFFFF,
];

// q where q = (p - 1) / 2 and q is a prime
const Q_VEC: [u32; 64] = [
    0xFFFFFFFF, 0x7FFFFFFF, 0x45565534, 0x0AB9472D, 0x4C7D0288, 0x8AE9130C, 0x754AB572, 0x1CCAA4BE,
    0x4AAC0B8C, 0xEF15E5FB, 0x37A62964, 0xDAE2AEF8, 0x7603D147, 0xCD93C1D1, 0x0C074301, 0xF1CF3B96,
    0x171B671D, 0x19482F23, 0x650C10BE, 0x78BA3604, 0x255E4C02, 0xB3861AA7, 0xB84B4B36, 0xCF6A9483,
    0x1042A95D, 0x0E3179AB, 0xEE51D6CB, 0xC1B2AE91, 0x7E9267AF, 0x348B1FD4, 0x0E2AE9CD, 0xCC6D241B,
    0x50B1DF82, 0xE1003E5C, 0xF6722D9E, 0x24943328, 0xBE258FF3, 0xD74F9208, 0xAD44CFD2, 0xF71C35FD,
    0x7A035BF6, 0x85FFAE5B, 0xD31BF6B5, 0x7A262174, 0x312F3F63, 0xF242DABB, 0xB6A8E122, 0xA7F09AB6,
    0xF92F8A1B, 0x98158536, 0xE69D218D, 0xF7CA8CD9, 0xC71A026E, 0x28A5043C, 0x1D89CD91, 0x105DF53,
    0x4533E63A, 0x94812704, 0xC06E0E68, 0x62633145, 0x10B4611A, 0xE487ED51, 0xFFFFFFFF, 0x7FFFFFFF,
];

fn main() {
    // Constants (Public)
    // p
    let p: BigInt = BigInt::new(Sign::Plus, P_VEC.to_vec());
    println!("p: {:?}", &p);

    // q
    let q: BigInt = BigInt::new(Sign::Plus, Q_VEC.to_vec());
    println!("p: {:?}", &q);

    assert_eq!(q, ((&p - 1) / 2));

    // Generators
    // Generator g
    let g: BigInt = BigInt::from(2);
    println!("g: {:?}", &g);

    // h: a random number that generates a group of prime order q
    let mut rng = rand::thread_rng();
    let random_num = rng.gen_bigint_range(&BigInt::one(), &q);
    let h = g.modpow(&random_num, &p);
    println!("h: {:?}", &h);

    // Inputs (Private)
    let password = String::from("dakard123#");
    let x = BigInt::from_bytes_be(Sign::Plus, password.as_bytes());
    println!("x: {:?}", &x);

    // Registration

    // Prover -- Client
    // y1: (g ^ x) mod p
    let y1 = g.modpow(&x, &p);
    println!("y1: {:?}", &y1);

    // y1: (h ^ x) mod p
    let y2 = h.modpow(&x, &p);
    println!("y2: {:?}", &y2);

    // Authentication

    // Prover -- Client
    // k: random k
    let k: BigInt = rng.gen_bigint_range(&BigInt::one(), &BigInt::from(467));
    // let k: BigInt = rng.gen_bigint(467);
    println!("k: {:?}", &k);

    // r1: g ^ k
    let r1: BigInt = g.modpow(&k, &p);
    println!("r1: {:?}", &r1);
    // r2: g ^ k
    let r2: BigInt = h.modpow(&k, &p);
    println!("r2: {:?}", &r2);

    // Verifier -- Server
    // generate c and send to client
    let c = rng.gen_bigint_range(&BigInt::one(), &BigInt::from(467));
    println!("c: {:?}", &c);

    // Prover -- Client
    // compute s and send to server
    // let s = &k - (&c * &x) % &q;
    let mut s = (&k - (&c * &x)).modpow(&BigInt::one(), &q);
    if s >= BigInt::zero() {
        s = s;
    } else {
        s += q;
    }
    println!("s: {:?}", &s);

    // Verifier -- Server
    // Check r1 and r2
    let maybe_r1 = (g.modpow(&s, &p) * y1.modpow(&c, &p)).modpow(&BigInt::one(), &p);
    println!("maybe_r1: {:?}", &maybe_r1);
    let maybe_r2 = (h.modpow(&s, &p) * y2.modpow(&c, &p)).modpow(&BigInt::one(), &p);
    println!("maybe_r2: {:?}", &maybe_r2);

    if r1 == maybe_r1 {
        println!("r1: True");
    } else {
        println!("r1: False");
    }
    if r2 == maybe_r2 {
        println!("r2: True");
    } else {
        println!("r2: False");
    }
}
