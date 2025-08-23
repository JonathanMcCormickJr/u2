use std::error;
use std::fmt;

#[derive(Debug)]
pub enum U2fError {
    Asm1DecoderError,
    BadSignature,
    RandomSecureBytesError,
    InvalidReservedByte,
    ChallengeExpired,
    WrongKeyHandler,
    InvalidClientData,
    InvalidSignatureData,
    InvalidUserPresenceByte,
    BadCertificate,
    NotTrustedAnchor,
    CounterTooLow,
    OpenSSLNoCurveName,
    InvalidPublicKey,
    OpenSSLError(openssl::error::ErrorStack),
}

impl fmt::Display for U2fError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            U2fError::Asm1DecoderError => write!(f, "ASM1 Decoder error"),
            U2fError::BadSignature => write!(f, "Not able to verify signature"),
            U2fError::RandomSecureBytesError => write!(f, "Not able to generate random bytes"),
            U2fError::InvalidReservedByte => write!(f, "Invalid Reserved Byte"),
            U2fError::ChallengeExpired => write!(f, "Challenge Expired"),
            U2fError::WrongKeyHandler => write!(f, "Wrong Key Handler"),
            U2fError::InvalidClientData => write!(f, "Invalid Client Data"),
            U2fError::InvalidSignatureData => write!(f, "Invalid Signature Data"),
            U2fError::InvalidUserPresenceByte => write!(f, "Invalid User Presence Byte"),
            U2fError::BadCertificate => write!(f, "Failed to parse certificate"),
            U2fError::NotTrustedAnchor => write!(f, "Not Trusted Anchor"),
            U2fError::CounterTooLow => write!(f, "Counter too low"),
            U2fError::InvalidPublicKey => write!(f, "Invalid public key"),
            U2fError::OpenSSLNoCurveName => write!(f, "OpenSSL no curve name"),
            U2fError::OpenSSLError(e) => e.fmt(f),
        }
    }
}

impl error::Error for U2fError {
    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            U2fError::Asm1DecoderError => None,
            U2fError::BadSignature => None,
            U2fError::RandomSecureBytesError => None,
            U2fError::InvalidReservedByte => None,
            U2fError::ChallengeExpired => None,
            U2fError::WrongKeyHandler => None,
            U2fError::InvalidClientData => None,
            U2fError::InvalidSignatureData => None,
            U2fError::InvalidUserPresenceByte => None,
            U2fError::BadCertificate => None,
            U2fError::NotTrustedAnchor => None,
            U2fError::CounterTooLow => None,
            U2fError::InvalidPublicKey => None,
            U2fError::OpenSSLNoCurveName => None,
            U2fError::OpenSSLError(_) => None,
        }
    }
}
