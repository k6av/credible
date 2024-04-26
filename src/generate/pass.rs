// These charsets MUST be ASCII, because we use byte indexing
const PASSWORD_CHARSET: &str = "qwertyuiopasdfghjklzxcvbnm1234567890QWERTYUIOPASDFGHJKLZXVBNM~`!@#$%^&*()-_+=[]{}|;:'<,>./?";

// TODO: make this DRY
pub fn generate(mut rng: impl rand::RngCore + rand::CryptoRng, (pass_l): (usize)) -> (String) {
    // Generate password
    let mut pass_a = vec![0 as u8; pass_l].into_boxed_slice(); // Initialize buffer slice
    rng.fill_bytes(&mut pass_a); // Fill slice with random bytes

    let mut pass_s = String::new(); // Initialize string
    let pass_cl = PASSWORD_CHARSET.len();
    for c in pass_a.into_iter() { // Map byte to character
        let c = *c as usize;
        // TODO: find better mapping algorithm
        let idx = c % pass_cl;

        // FIXME
        pass_s += PASSWORD_CHARSET.as_bytes()[idx]
            .as_ascii().unwrap().as_str();
    }

    (pass_s)
}

#[cfg(test)]
mod tests {
    use super::generate;

    #[test]
    /// Generated strings must be of the specified length
    fn basic_length() {
        let pass_l = 82;

        let rng = rand::thread_rng();

        let (pass_s) = generate(rng, (pass_l));

        assert_eq!(pass_s.len(), 82);
    }

    #[test]
    /// Generated strings must only contain valid ASCII characters
    fn ascii_only() {
        let pass_l = 82;

        let rng = rand::thread_rng();

        let (pass_s) = generate(rng, (pass_l));

        assert!(pass_s.is_ascii());
    }
}