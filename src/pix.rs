//! PIX key validation
//!
//! Brazilian instant payment system key validation supporting
//! CPF, CNPJ, email, phone, and random key formats.

use crate::error::{BrazilianValidationError, ValidationResult};
use crate::{cpf, cnpj};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    /// Regex for CPF format
    static ref CPF_REGEX: Regex = Regex::new(r"^\d{3}\.?\d{3}\.?\d{3}-?\d{2}$").unwrap();

    /// Regex for CNPJ format
    static ref CNPJ_REGEX: Regex = Regex::new(r"^\d{2}\.?\d{3}\.?\d{3}/?\d{4}-?\d{2}$").unwrap();

    /// Regex for email format
    static ref EMAIL_REGEX: Regex = Regex::new(
        r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$"
    ).unwrap();

    /// Regex for PIX phone format (+55 followed by 11 digits)
    static ref PIX_PHONE_REGEX: Regex = Regex::new(r"^\+55\d{11}$").unwrap();

    /// Regex for random PIX key (UUID format)
    static ref RANDOM_KEY_REGEX: Regex = Regex::new(
        r"^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
    ).unwrap();
}

/// PIX key types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PixKeyType {
    /// CPF (individual taxpayer ID)
    Cpf,
    /// CNPJ (business taxpayer ID)
    Cnpj,
    /// Email address
    Email,
    /// Phone number (+55 format)
    Phone,
    /// Random key (UUID)
    Random,
}

impl std::fmt::Display for PixKeyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PixKeyType::Cpf => write!(f, "CPF"),
            PixKeyType::Cnpj => write!(f, "CNPJ"),
            PixKeyType::Email => write!(f, "E-mail"),
            PixKeyType::Phone => write!(f, "Telefone"),
            PixKeyType::Random => write!(f, "Chave aleatória"),
        }
    }
}

/// Validate a PIX key
///
/// Supports all PIX key types: CPF, CNPJ, email, phone (+55 format), and random (UUID).
///
/// # Arguments
/// * `key` - PIX key string
///
/// # Returns
/// * `Ok(())` - Valid PIX key
/// * `Err(BrazilianValidationError)` - Validation error
///
/// # Examples
/// ```
/// use pleme_brazilian_validators::pix::validate;
///
/// // CPF
/// assert!(validate("123.456.789-09").is_ok());
///
/// // CNPJ
/// assert!(validate("11.222.333/0001-81").is_ok());
///
/// // Email
/// assert!(validate("user@example.com").is_ok());
///
/// // Phone
/// assert!(validate("+5511987654321").is_ok());
///
/// // Random (UUID)
/// assert!(validate("123e4567-e89b-12d3-a456-426614174000").is_ok());
/// ```
pub fn validate(key: &str) -> ValidationResult<()> {
    let key = key.trim();

    // Try each format in order of specificity
    if is_cpf_format(key) {
        cpf::validate(key)?;
        return Ok(());
    }

    if is_cnpj_format(key) {
        cnpj::validate(key)?;
        return Ok(());
    }

    if is_email_format(key) {
        return Ok(());
    }

    if is_phone_format(key) {
        return Ok(());
    }

    if is_random_key_format(key) {
        return Ok(());
    }

    Err(BrazilianValidationError::invalid_pix_key(
        "formato não reconhecido",
    ))
}

/// Alias for validate() for consistent API
pub fn validate_pix_key(key: &str) -> ValidationResult<()> {
    validate(key)
}

/// Detect the type of a PIX key
///
/// # Arguments
/// * `key` - PIX key string
///
/// # Returns
/// * `Some(PixKeyType)` - Detected key type
/// * `None` - Unknown format
///
/// # Examples
/// ```
/// use pleme_brazilian_validators::pix::{detect_type, PixKeyType};
///
/// assert_eq!(detect_type("123.456.789-09"), Some(PixKeyType::Cpf));
/// assert_eq!(detect_type("user@example.com"), Some(PixKeyType::Email));
/// assert_eq!(detect_type("+5511987654321"), Some(PixKeyType::Phone));
/// ```
pub fn detect_type(key: &str) -> Option<PixKeyType> {
    let key = key.trim();

    if is_cpf_format(key) {
        Some(PixKeyType::Cpf)
    } else if is_cnpj_format(key) {
        Some(PixKeyType::Cnpj)
    } else if is_email_format(key) {
        Some(PixKeyType::Email)
    } else if is_phone_format(key) {
        Some(PixKeyType::Phone)
    } else if is_random_key_format(key) {
        Some(PixKeyType::Random)
    } else {
        None
    }
}

/// Validate and return the type of a PIX key
///
/// # Examples
/// ```
/// use pleme_brazilian_validators::pix::{validate_with_type, PixKeyType};
///
/// let (key_type, _) = validate_with_type("123.456.789-09").unwrap();
/// assert_eq!(key_type, PixKeyType::Cpf);
/// ```
pub fn validate_with_type(key: &str) -> ValidationResult<(PixKeyType, String)> {
    let key = key.trim();

    // Try CPF
    if is_cpf_format(key) {
        let normalized = cpf::validate(key)?;
        return Ok((PixKeyType::Cpf, normalized));
    }

    // Try CNPJ
    if is_cnpj_format(key) {
        let normalized = cnpj::validate(key)?;
        return Ok((PixKeyType::Cnpj, normalized));
    }

    // Try email
    if is_email_format(key) {
        return Ok((PixKeyType::Email, key.to_lowercase()));
    }

    // Try phone
    if is_phone_format(key) {
        return Ok((PixKeyType::Phone, key.to_string()));
    }

    // Try random key
    if is_random_key_format(key) {
        return Ok((PixKeyType::Random, key.to_lowercase()));
    }

    Err(BrazilianValidationError::invalid_pix_key(
        "formato não reconhecido",
    ))
}

/// Check if key matches CPF format
fn is_cpf_format(key: &str) -> bool {
    CPF_REGEX.is_match(key)
}

/// Check if key matches CNPJ format
fn is_cnpj_format(key: &str) -> bool {
    CNPJ_REGEX.is_match(key)
}

/// Check if key matches email format
fn is_email_format(key: &str) -> bool {
    EMAIL_REGEX.is_match(key)
}

/// Check if key matches PIX phone format (+55 with 11 digits)
fn is_phone_format(key: &str) -> bool {
    PIX_PHONE_REGEX.is_match(key)
}

/// Check if key matches random key format (UUID)
fn is_random_key_format(key: &str) -> bool {
    RANDOM_KEY_REGEX.is_match(&key.to_lowercase())
}

/// Normalize a PIX key based on its type
///
/// # Examples
/// ```
/// use pleme_brazilian_validators::pix::normalize;
///
/// assert_eq!(normalize("123.456.789-09"), "12345678909");
/// assert_eq!(normalize("User@Example.COM"), "user@example.com");
/// ```
pub fn normalize(key: &str) -> String {
    let key = key.trim();

    if is_cpf_format(key) {
        cpf::normalize(key)
    } else if is_cnpj_format(key) {
        cnpj::normalize(key)
    } else if is_email_format(key) {
        key.to_lowercase()
    } else if is_random_key_format(key) {
        key.to_lowercase()
    } else {
        key.to_string()
    }
}

/// Mask a PIX key for display (partial reveal)
///
/// # Examples
/// ```
/// use pleme_brazilian_validators::pix::mask;
///
/// assert_eq!(mask("12345678909"), "123.***.***-09"); // CPF
/// assert_eq!(mask("user@example.com"), "u***@example.com"); // Email
/// assert_eq!(mask("+5511987654321"), "+55 (11) *****-4321"); // Phone
/// ```
pub fn mask(key: &str) -> String {
    let key = key.trim();

    if is_cpf_format(key) {
        cpf::mask(key)
    } else if is_cnpj_format(key) {
        cnpj::mask(key)
    } else if is_email_format(key) {
        mask_email(key)
    } else if is_phone_format(key) {
        mask_phone(key)
    } else if is_random_key_format(key) {
        mask_random_key(key)
    } else {
        key.to_string()
    }
}

/// Mask an email address
fn mask_email(email: &str) -> String {
    if let Some(at_pos) = email.find('@') {
        let local = &email[..at_pos];
        let domain = &email[at_pos..];

        if local.len() > 1 {
            format!("{}***{}", &local[0..1], domain)
        } else {
            format!("***{}", domain)
        }
    } else {
        email.to_string()
    }
}

/// Mask a phone number
fn mask_phone(phone: &str) -> String {
    if phone.len() >= 14 {
        // +5511987654321 -> +55 (11) *****-4321
        format!(
            "+55 ({}) *****-{}",
            &phone[3..5],
            &phone[phone.len() - 4..]
        )
    } else {
        phone.to_string()
    }
}

/// Mask a random key (UUID)
fn mask_random_key(key: &str) -> String {
    if key.len() >= 8 {
        format!("{}****-****-****-****-****", &key[0..4])
    } else {
        key.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_cpf_key() {
        assert!(validate("123.456.789-09").is_ok());
        assert!(validate("12345678909").is_ok());
        assert!(validate("111.111.111-11").is_err()); // Invalid CPF
    }

    #[test]
    fn test_validate_cnpj_key() {
        assert!(validate("11.222.333/0001-81").is_ok());
        assert!(validate("11222333000181").is_ok());
    }

    #[test]
    fn test_validate_email_key() {
        assert!(validate("user@example.com").is_ok());
        assert!(validate("test.user+tag@domain.co.uk").is_ok());
        assert!(validate("invalid@").is_err());
    }

    #[test]
    fn test_validate_phone_key() {
        assert!(validate("+5511987654321").is_ok());
        assert!(validate("+5521912345678").is_ok());
        assert!(validate("11987654321").is_err()); // Missing +55
    }

    #[test]
    fn test_validate_random_key() {
        assert!(validate("123e4567-e89b-12d3-a456-426614174000").is_ok());
        assert!(validate("123E4567-E89B-12D3-A456-426614174000").is_ok()); // Uppercase OK
        assert!(validate("not-a-uuid").is_err());
    }

    #[test]
    fn test_detect_type() {
        assert_eq!(detect_type("12345678909"), Some(PixKeyType::Cpf));
        assert_eq!(detect_type("11222333000181"), Some(PixKeyType::Cnpj));
        assert_eq!(detect_type("user@example.com"), Some(PixKeyType::Email));
        assert_eq!(detect_type("+5511987654321"), Some(PixKeyType::Phone));
        assert_eq!(
            detect_type("123e4567-e89b-12d3-a456-426614174000"),
            Some(PixKeyType::Random)
        );
        assert_eq!(detect_type("invalid"), None);
    }

    #[test]
    fn test_validate_with_type() {
        let (key_type, normalized) = validate_with_type("123.456.789-09").unwrap();
        assert_eq!(key_type, PixKeyType::Cpf);
        assert_eq!(normalized, "12345678909");

        let (key_type, normalized) = validate_with_type("User@Example.COM").unwrap();
        assert_eq!(key_type, PixKeyType::Email);
        assert_eq!(normalized, "user@example.com");
    }

    #[test]
    fn test_normalize() {
        assert_eq!(normalize("123.456.789-09"), "12345678909");
        assert_eq!(normalize("11.222.333/0001-81"), "11222333000181");
        assert_eq!(normalize("User@Example.COM"), "user@example.com");
    }

    #[test]
    fn test_mask() {
        assert_eq!(mask("12345678909"), "123.***.***-09");
        assert_eq!(mask("user@example.com"), "u***@example.com");
        assert_eq!(mask("+5511987654321"), "+55 (11) *****-4321");
        assert_eq!(
            mask("123e4567-e89b-12d3-a456-426614174000"),
            "123e****-****-****-****-****"
        );
    }
}
