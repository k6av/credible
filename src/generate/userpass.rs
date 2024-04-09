// These charsets MUST be ASCII, because we use byte indexing
const USERNAME_CHARSET: &str = "qwertyuiopasdfghjklzxcvbnm1234567890";
const PASSWORD_CHARSET: &str = "qwertyuiopasdfghjklzxcvbnm1234567890QWERTYUIOPASDFGHJKLZXVBNM~`!@#$%^&*()-_+=[]{}|;:'<,>./?";

pub fn generate(mut rng: impl rand::RngCore + rand::CryptoRng, (user_l, pass_l): (usize, usize)) -> (String, String) {
    let mut user_a = vec![0 as u8; user_l].into_boxed_slice();
    rng.fill_bytes(&mut user_a);

    let mut user_s = String::new();
    let user_cl = USERNAME_CHARSET.len();
    for c in user_a.into_iter() {
        let c = *c as usize;
        let idx = c % user_cl;

        user_s += USERNAME_CHARSET.as_bytes()[idx]
            .as_ascii().unwrap().as_str();
    }


    let mut pass_a = vec![0 as u8; pass_l].into_boxed_slice();
    rng.fill_bytes(&mut pass_a);

    let mut pass_s = String::new();
    let pass_cl = PASSWORD_CHARSET.len();
    for c in pass_a.into_iter() {
        let c = *c as usize;
        // TODO: does this give a suffeciently uniform distribution?
        let idx = c % pass_cl;

        pass_s += PASSWORD_CHARSET.as_bytes()[idx]
            .as_ascii().unwrap().as_str();
    }

    (user_s, pass_s)
}