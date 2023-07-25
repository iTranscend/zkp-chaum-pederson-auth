pub mod logger {
    use std::io::Write;

    use env_logger::{fmt::Color, Builder, Env};
    use log::Level;

    pub fn setup() {
        Builder::from_env(Env::default().default_filter_or("warn,info"))
            .format(|buf, record| {
                writeln!(
                    buf,
                    "{}{} {}",
                    match (record.level(), buf.style().set_bold(true)) {
                        (Level::Warn, style) => style.set_color(Color::Yellow).value("warning"),
                        (Level::Error, style) => style.set_color(Color::Red).value("error"),
                        (Level::Info, style) => style.set_color(Color::Green).value("info"),
                        (level, style) => style.value(level.as_str()),
                    },
                    buf.style().set_bold(true).value(":"),
                    record.args()
                )
            })
            .init();
    }
}

pub mod random {
    use num_bigint::{BigUint, RandBigInt};
    use num_traits::One;
    use rand::Rng;

    pub fn alphanumeric(n: usize) -> String {
        rand::thread_rng()
            .sample_iter(rand::distributions::Alphanumeric)
            .take(n)
            .map(char::from)
            .collect()
    }

    /// Generate a random number in the range [1, n)
    pub fn biguint(ubound: &BigUint) -> BigUint {
        rand::thread_rng().gen_biguint_range(&BigUint::one(), ubound)
    }
}

pub mod biguint {
    use num_bigint::BigUint;

    pub fn serialize(val: BigUint) -> Vec<u8> {
        val.to_bytes_be()
    }

    pub fn deserialize(val: &[u8]) -> BigUint {
        BigUint::from_bytes_be(val)
    }
}

pub mod string {
    use num_bigint::BigUint;

    pub fn as_biguint(s: &str) -> BigUint {
        super::biguint::deserialize(s.as_bytes())
    }
}

pub mod style {
    pub const BOLD: &str = "\x1b[1m";

    pub mod fg {
        pub const RED: &str = "\x1b[31m";
        pub const GREEN: &str = "\x1b[32m";
        pub const YELLOW: &str = "\x1b[33m";
        pub const CYAN: &str = "\x1b[36m";

        pub const RESET: &str = "\x1b[39m";
    }

    pub const RESET: &str = "\x1b[0m";
}
