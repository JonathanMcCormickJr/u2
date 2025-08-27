use crate::u2ferror::U2fError;
use base64::{URL_SAFE_NO_PAD, encode_config};
use bytes::Bytes;
use chrono::{TimeDelta, prelude::*};
use openssl::rand;

/// The `Result` type used in this crate.
type Result<T> = ::std::result::Result<T, U2fError>;

pub const U2F_V2: &str = "U2F_V2";

/// Generates a challenge from a secure, random source.
///
/// ```rust
/// use u2::util;
/// use assertables::assert_gt;
///
/// let random_bytes0 = util::generate_challenge_randomness(32).unwrap();
/// let random_challenge_bytes_as_string0 = format!("{:?}", random_bytes0);
/// assert_eq!(random_bytes0.len(), 32);
/// let entropy0 = util::shannon_entropy(&random_bytes0);
/// assert_gt!(entropy0, 4.5);
/// assert!(entropy0 <= 8.0);
///
/// let random_bytes1 = util::generate_challenge_randomness(32).unwrap();
/// let random_challenge_bytes_as_string1 = format!("{:?}", random_bytes1);
/// assert_eq!(random_bytes1.len(), 32);
/// let entropy1 = util::shannon_entropy(&random_bytes1);
/// assert_gt!(entropy1, 4.5);
/// assert!(entropy1 <= 8.0);
///
/// assert_ne!(random_challenge_bytes_as_string0, random_challenge_bytes_as_string1);
/// ```
pub fn generate_challenge_randomness(size: usize) -> Result<Vec<u8>> {
    let mut bytes: Vec<u8> = vec![0; size];
    rand::rand_bytes(&mut bytes).map_err(|_e| U2fError::RandomSecureBytesError)?;
    Ok(bytes)
}

/// Time which has elapsed since a specified timestamp.
///
/// ```rust
/// use chrono::TimeDelta;
/// use u2::util;
/// use assertables::assert_gt;
///
/// let timestamp0 = String::from("2025-08-24T15:30:00.123456Z");
/// let elapsed_time0 = util::elapsed_time(timestamp0);
/// assert_gt!(elapsed_time0, TimeDelta::new(25927, 626454086).unwrap());
///
/// let timestamp1 = String::from("2012-07-19T22:05:41.374829123Z");
/// let elapsed_time1 = util::elapsed_time(timestamp1);
/// assert_gt!(elapsed_time1, TimeDelta::new(413340209, 626454086).unwrap());
/// ```
pub fn elapsed_time(timestamp: String) -> TimeDelta {
    let now: DateTime<Utc> = Utc::now();

    let ts = timestamp.parse::<DateTime<Utc>>();

    now.signed_duration_since(ts.unwrap())
}

/// Decode initial bytes of buffer as ASN and return the length of the encoded structure.
/// [Wikipedia](http://en.wikipedia.org/wiki/X.690)
///
/// VERIFICATION NEEDED ON THIS ONE!! NOT TO BE USED IN PRODUCTION YET!!
///
/// ```rust
/// use u2::util::asn_length;
/// use bytes::Bytes;
///
/// let short_bytes = Bytes::from_static(&[0x30, 0x03, 0x01, 0x02, 0x03]);
/// assert_eq!(asn_length(short_bytes).unwrap(), 3);
///
/// let long_bytes = Bytes::from_static(&[0x30, 0x82, 0x00, 0x04, 0xAA, 0xBB, 0xCC, 0xDD]);
/// assert_eq!(asn_length(long_bytes).unwrap(), 8);
///
/// ```
pub fn asn_length(mem: Bytes) -> Result<usize> {
    let buffer: &[u8] = &mem[..];

    if mem.len() < 2 || buffer[0] != 0x30 {
        // Type
        return Err(U2fError::Asm1DecoderError);
    }

    let len = buffer[1]; // Len
    if len & 0x80 == 0 {
        return Ok((len & 0x7f) as usize);
    }

    let number_of_bytes = len & 0x7f;
    if number_of_bytes == 0 {
        return Err(U2fError::Asm1DecoderError);
    }

    let mut length: usize = 0;
    for num in 0..number_of_bytes {
        length = length * 0x100 + (buffer[(2 + num) as usize] as usize);
    }

    length += number_of_bytes as usize;

    Ok(length + 2) // Add the 2 initial bytes: type and length.
}

/// Encode as URL-safe, no padding base64
///
/// ```rust
/// use u2::util::get_encoded;
///
/// let message0 = b"My string!!";
/// let encoded_string0 = get_encoded(message0);
/// assert_eq!(encoded_string0, String::from("TXkgc3RyaW5nISE"));
///
/// let message1 = b"Weaponized assault penguins";
/// let encoded_string1 = get_encoded(message1);
/// assert_eq!(encoded_string1, String::from("V2VhcG9uaXplZCBhc3NhdWx0IHBlbmd1aW5z"));
/// ```
pub fn get_encoded(data: &[u8]) -> String {
    let encoded: String = encode_config(data, URL_SAFE_NO_PAD);

    encoded.trim_end_matches('=').to_string()
}

/// Computes the Shannon entropy (in bits per byte) of a byte slice.
///
/// The calculation treats the slice as raw bytes, not as UTF‑8 characters.
/// Returns 0.0 for an empty slice.
///
/// ```rust
/// use u2::util::shannon_entropy;
///
/// let se0 = shannon_entropy("Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.".as_bytes());
/// assert_eq!(se0, 4.022379320675357);
///
/// let se1 = shannon_entropy("11111111111".as_bytes());
/// assert_eq!(se1, 0.0);
///
/// let se2 = shannon_entropy("Moderate-complexity text.".as_bytes());
/// assert_eq!(se2, 3.8438561897747237);
///
/// let se3 = shannon_entropy("XuU7372pb8rJ5BZuqCM2tucaNVKhDcv4".as_bytes());
/// assert_eq!(se3, 4.663909765557392);
///
/// ```
///
pub fn shannon_entropy(input: &[u8]) -> f64 {
    if input.is_empty() {
        return 0.0;
    }

    let len = input.len() as f64;

    // Frequency table for all possible byte values (0..=255).
    let mut freq = [0usize; 256];
    for &b in input {
        freq[b as usize] += 1;
    }

    // ‑∑ p·log₂(p)
    let mut entropy = 0.0_f64;
    for &count in &freq {
        if count == 0 {
            continue; // skip symbols that never appear
        }
        let p = count as f64 / len;
        entropy -= p * p.log2();
    }

    entropy
}
