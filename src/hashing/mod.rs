use harsh::HarshBuilder;

pub trait Hasher {
    // new returns a new hasher from salt.
    fn generate(salt: &str, input: &str) -> String;
}

// HashIDHasher implements the Hasher trait.
pub struct HashIDHasher {}

impl Hasher for HashIDHasher {
    fn generate(salt: &str, input: &str) -> String {
        let hb = HarshBuilder::new()
            .salt(salt.to_owned())
            .length(8)
            .init()
            .unwrap();
        hb.encode_hex(input).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::{HashIDHasher, Hasher};

    #[test]
    fn hashing_works() {
        let time = "604573000";
        let hash = HashIDHasher::generate("https://google.com", time);
        assert_eq!("m89l18bG", hash);
    }
}
