// These charsets MUST be ASCII, because we use byte indexing
const USERNAME_CHARSET: &str = "qwertyuiopasdfghjklzxcvbnm1234567890";
const PASSWORD_CHARSET: &str = "qwertyuiopasdfghjklzxcvbnm1234567890QWERTYUIOPASDFGHJKLZXVBNM~`!@#$%^&*()-_+=[]{}|;:'<,>./?";

pub fn generate(mut rng: impl rand::RngCore + rand::CryptoRng, (user_l, pass_l): (usize, usize)) -> (String, String) {
    // Generate username
    let mut user_a = vec![0 as u8; user_l].into_boxed_slice(); // Initialize buffer slice
    rng.fill_bytes(&mut user_a); // Fill slice with random bytes

    let mut user_s = String::new(); // Initialize string
    let user_cl = USERNAME_CHARSET.len();
    for c in user_a.into_iter() { // Map byte to character
        let c = *c as usize;
        // TODO: find better mapping algorithm
        let idx = c % user_cl;

        // FIXME
        user_s += USERNAME_CHARSET.as_bytes()[idx]
            .as_ascii().unwrap().as_str();
    }

    // Generate password
    let mut pass_a = vec![0 as u8; pass_l].into_boxed_slice();
    rng.fill_bytes(&mut pass_a);

    let mut pass_s = String::new();
    let pass_cl = PASSWORD_CHARSET.len();
    for c in pass_a.into_iter() {
        let c = *c as usize;
        let idx = c % pass_cl;

        pass_s += PASSWORD_CHARSET.as_bytes()[idx]
            .as_ascii().unwrap().as_str();
    }

    (user_s, pass_s)
}

#[cfg(test)]
mod tests {
    use super::generate;

    #[test]
    /// Generated strings must be of the specified length
    fn basic_length() {
        let user_l = 45;
        let pass_l = 82;

        let rng = rand::thread_rng();

        let (user_s, pass_s) = generate(rng, (user_l, pass_l));

        assert_eq!(user_s.len(), 45);
        assert_eq!(pass_s.len(), 82);
    }

    #[test]
    /// Generated strings must only contain valid ASCII characters
    fn ascii_only() {
        let user_l = 45;
        let pass_l = 82;

        let rng = rand::thread_rng();

        let (user_s, pass_s) = generate(rng, (user_l, pass_l));

        assert!(user_s.is_ascii());
        assert!(pass_s.is_ascii());
    }
}