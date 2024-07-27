use rand_chacha::{
    rand_core::{RngCore, SeedableRng},
    ChaCha20Core, ChaCha20Rng,
};
use std::env;

fn main() {
    // add all arguments (minus program name) to a single &str
    let args: Vec<String> = env::args().collect();
    let mut string = String::new();
    for i in args.iter().skip(1) {
        string.push_str(i);
    }
    // generate a port number from the string
    let port = port_gen(&string);
    println!("{}", port);
}

/// converts a string to a u16 port number
fn port_gen(string: &str) -> u16 {
    // convert string to u64
    let mut hash: u64 = 0;
    hash_string(string, &mut hash);
    // run u64 through chacha20
    ChaCha20Rng::from(ChaCha20Core::seed_from_u64(hash)).next_u32() as u16
}

fn hash_string(string: &str, hash: &mut u64) {
    for c in string.chars() {
        *hash = (c as u64)
            .wrapping_add(*hash << 6)
            .wrapping_add(*hash << 16)
            .wrapping_sub(*hash);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_port_gen_will_not_change() {
        assert_eq!(port_gen("string"), 22933);
    }

    #[test]
    fn test_hash_will_not_change() {
        let mut hash: u64 = 0;
        hash_string("string", &mut hash);
        assert_eq!("2985983639226296369", hash.to_string());
    }

    #[test]
    fn test_long_names_dont_crash() {
        assert_ne!(
            port_gen("the ones you will not know have already come",),
            22933
        )
    }
}
