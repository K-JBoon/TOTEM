use std::time::SystemTime;

use hmac::{Hmac, Mac, NewMac};
use sha1::Sha1;

type HmacSha1 = Hmac<Sha1>;

pub struct TOTP<'a> {
    /// A byte representation of the secret to be used for the HMAC hashing algorithm
    pub secret: &'a [u8],
    /// The length of the TOTP to generate
    pub digits: usize,
    /// The timestep to use for generating the TOTP
    pub timestep: u64,
}

impl<'a> TOTP<'a> {
    /// Helper function to construct a new instance of TOTP
    pub fn new(secret: &'a [u8], digits: usize, timestep: u64) -> Self {
        Self {
            secret,
            digits,
            timestep
        }
    }

    /// Signs the given timestamp with the given secret
    pub fn sign(&self, time: u64) -> Vec<u8> {
        // We don't explicitly state T0 = 0, https://tools.ietf.org/html/rfc6238#section-4.2
        let ctr = (time / self.timestep).to_be_bytes();

        // Create a HMAC-SHA1 instance with the provided secret key
        let mut mac = HmacSha1::new_varkey(self.secret).expect("HMAC can take key of any size");

        // Pass in the current timestamp
        mac.update(&ctr);
        mac.finalize().into_bytes().to_vec()
    }

    /// Generate a TOTP token for the given timestamp
    pub fn generate_token_at_time(&self, time: u64) -> String {
        let signed_timestamp = &self.sign(time);

        // sha1 always returns 20 bytes
        // bitwise AND over byte 19 (20th byte) and 0xf, https://tools.ietf.org/html/rfc4226#section-5.4
        // e.g. byte 19 = 0x5a = 0101 1010
        //      over      0xf  =      1111
        //                0xa  = 0000 1010 = 10
        //  So offset value is at byte 10
        let offset = (signed_timestamp[19] & 0xf) as usize;

        // We have a u8, convert to u32 to allow left-shift of 24 bit postions
        let result = (u32::from(signed_timestamp[offset]) & 0x7f) << 24 // Mask first byte with 0x7f, https://tools.ietf.org/html/rfc4226#section-5.4
                    | (u32::from(signed_timestamp[offset+1])) << 16 // In reference implementations these are masked with 0xff, but they are u8 so we can skip this
                    | (u32::from(signed_timestamp[offset+2])) << 8
                    | (u32::from(signed_timestamp[offset+3]));

        // With everything stringed together into a u32, we format by taking module 10^Digit, https://tools.ietf.org/html/rfc4226#section-5.3
        format!("{:01$}",
            (result as u64) % (10_u64).pow(self.digits as u32),
            self.digits
        )
    }

    /// Generate a TOTP token for the current system time
    pub fn generate_token(&self) -> String {
        let time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH).unwrap()
            .as_secs();

            self.generate_token_at_time(time)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_correct_tokens() {
        let totp = TOTP::new(b"potato", 6, 30);
        assert_eq!(totp.generate_token_at_time(1000), String::from("909468"));

        let totp = TOTP::new(b"tomato", 7, 49);
        assert_eq!(totp.generate_token_at_time(123456789), String::from("8759068"));
    }

    #[test]
    fn generates_correct_length_token() {
        for i in 6..12 {
            let totp = TOTP::new(b"carrot", i, 120);
            assert_eq!(totp.generate_token().len(), i);
        }
    }
}
