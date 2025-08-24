use std::error;
use std::fmt;

/// An error for a U2F system.
///
/// ```rust
/// use u2::u2ferror::U2fError;
/// use openssl::error::ErrorStack;
///
/// let err0 = U2fError::Asm1DecoderError;
/// let err1 = U2fError::BadSignature;
/// let err2 = U2fError::RandomSecureBytesError;
/// let err3 = U2fError::InvalidReservedByte;
/// let err4 = U2fError::ChallengeExpired;
/// let err5 = U2fError::WrongKeyHandler;
/// let err6 = U2fError::InvalidClientData;
/// let err7 = U2fError::InvalidSignatureData;
/// let err8 = U2fError::InvalidUserPresenceByte;
/// let err9 = U2fError::BadCertificate;
/// let err10 = U2fError::NotTrustedAnchor;
/// let err11 = U2fError::CounterTooLow;
/// let err13 = U2fError::InvalidPublicKey;
/// let err12 = U2fError::OpenSSLNoCurveName;
///
/// let bogus_pem = std::fs::read_to_string("non_existent.pem")
///     .map_err(|_| ErrorStack::get())   // fabricate an ErrorStack for the demo
///     .unwrap_err();
/// let err14 = U2fError::OpenSSLError(bogus_pem);
/// ```
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
    InvalidPublicKey,
    OpenSSLNoCurveName,
    OpenSSLError(openssl::error::ErrorStack),
}

impl fmt::Display for U2fError {
    /// ```rust
    /// use u2::u2ferror::U2fError;
    /// use openssl::error::ErrorStack;
    ///
    /// let err0_message = format!("{}", U2fError::Asm1DecoderError);
    /// assert_eq!(err0_message, String::from("ASM1 Decoder error"));
    ///
    /// let err1_message = format!("{}", U2fError::BadSignature);
    /// assert_eq!(err1_message, String::from("Not able to verify signature"));
    ///
    /// let err2_message = format!("{}", U2fError::RandomSecureBytesError);
    /// assert_eq!(err2_message, String::from("Not able to generate random bytes"));
    ///
    /// let err3_message = format!("{}", U2fError::InvalidReservedByte);
    /// assert_eq!(err3_message, String::from("Invalid Reserved Byte"));
    ///
    /// let err4_message = format!("{}", U2fError::ChallengeExpired);
    /// assert_eq!(err4_message, String::from("Challenge Expired"));
    ///
    /// let err5_message = format!("{}", U2fError::WrongKeyHandler);
    /// assert_eq!(err5_message, String::from("Wrong Key Handler"));
    ///
    /// let err6_message = format!("{}", U2fError::InvalidClientData);
    /// assert_eq!(err6_message, String::from("Invalid Client Data"));
    ///
    /// let err7_message = format!("{}", U2fError::InvalidSignatureData);
    /// assert_eq!(err7_message, String::from("Invalid Signature Data"));
    ///
    /// let err8_message = format!("{}", U2fError::InvalidUserPresenceByte);
    /// assert_eq!(err8_message, String::from("Invalid User Presence Byte"));
    ///
    /// let err9_message = format!("{}", U2fError::BadCertificate);
    /// assert_eq!(err9_message, String::from("Failed to parse certificate"));
    ///
    /// let err10_message = format!("{}", U2fError::NotTrustedAnchor);
    /// assert_eq!(err10_message, String::from("Not Trusted Anchor"));
    ///
    /// let err11_message = format!("{}", U2fError::CounterTooLow);
    /// assert_eq!(err11_message, String::from("Counter too low"));
    ///
    /// let err13_message = format!("{}", U2fError::InvalidPublicKey);
    /// assert_eq!(err13_message, String::from("Invalid public key"));
    ///
    /// let err12_message = format!("{}", U2fError::OpenSSLNoCurveName);
    /// assert_eq!(err12_message, String::from("OpenSSL no curve name"));
    ///
    /// let bogus_pem = std::fs::read_to_string("non_existent.pem")
    ///     .map_err(|_| ErrorStack::get())   // fabricate an ErrorStack for the demo
    ///     .unwrap_err();
    /// let err14_message = format!("{}", U2fError::OpenSSLError(bogus_pem));
    /// assert_eq!(err14_message, String::from("OpenSSL error"));
    ///
    /// ```
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

impl error::Error for U2fError {}
