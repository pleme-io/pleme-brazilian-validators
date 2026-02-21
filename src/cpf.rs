//! CPF (Cadastro de Pessoas Físicas) validation and formatting
//!
//! Brazilian individual taxpayer identification number with 11 digits
//! and two check digits calculated using modulo 11.

use crate::error::{BrazilianValidationError, ValidationResult};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    /// Regex for CPF format (with or without punctuation)
    static ref CPF_REGEX: Regex = Regex::new(r"^\d{3}\.?\d{3}\.?\d{3}-?\d{2}$").unwrap();
}

/// Known invalid CPFs (all same digits)
const INVALID_CPFS: [&str; 10] = [
    "00000000000",
    "11111111111",
    "22222222222",
    "33333333333",
    "44444444444",
    "55555555555",
    "66666666666",
    "77777777777",
    "88888888888",
    "99999999999",
];

/// Validate a Brazilian CPF number
///
/// Validates format, length, check digits, and rejects known invalid sequences.
///
/// # Arguments
/// * `cpf` - CPF string (with or without punctuation)
///
/// # Returns
/// * `Ok(String)` - Normalized CPF (11 digits only)
/// * `Err(BrazilianValidationError)` - Validation error
///
/// # Examples
/// ```
/// use pleme_brazilian_validators::cpf::validate;
///
/// assert!(validate("123.456.789-09").is_ok());
/// assert!(validate("12345678909").is_ok());
/// assert!(validate("111.111.111-11").is_err()); // All same digits
/// assert!(validate("123.456.789-00").is_err()); // Invalid check digits
/// ```
pub fn validate(cpf: &str) -> ValidationResult<String> {
    let cleaned = normalize(cpf);

    // Check length
    if cleaned.len() != 11 {
        return Err(BrazilianValidationError::InvalidLength {
            expected: 11,
            actual: cleaned.len(),
        });
    }

    // Ensure all characters are digits
    if !cleaned.chars().all(|c| c.is_ascii_digit()) {
        return Err(BrazilianValidationError::InvalidCharacters);
    }

    // Check for known invalid CPFs
    if INVALID_CPFS.contains(&cleaned.as_str()) {
        return Err(BrazilianValidationError::invalid_cpf(
            "sequência de dígitos repetidos",
        ));
    }

    // Validate check digits
    if !validate_check_digits(&cleaned) {
        return Err(BrazilianValidationError::InvalidCheckDigits {
            document_type: "CPF".to_string(),
        });
    }

    Ok(cleaned)
}

/// Alias for validate() for consistent API
pub fn validate_cpf(cpf: &str) -> ValidationResult<String> {
    validate(cpf)
}

/// Normalize a CPF string by removing all non-digit characters
///
/// # Examples
/// ```
/// use pleme_brazilian_validators::cpf::normalize;
///
/// assert_eq!(normalize("123.456.789-09"), "12345678909");
/// assert_eq!(normalize("12345678909"), "12345678909");
/// ```
pub fn normalize(cpf: &str) -> String {
    cpf.chars().filter(|c| c.is_ascii_digit()).collect()
}

/// Alias for normalize() for consistent API
pub fn normalize_cpf(cpf: &str) -> String {
    normalize(cpf)
}

/// Format a CPF string with standard punctuation (XXX.XXX.XXX-XX)
///
/// # Arguments
/// * `cpf` - CPF string (normalized or formatted)
///
/// # Returns
/// Formatted CPF string. Returns input unchanged if not 11 digits.
///
/// # Examples
/// ```
/// use pleme_brazilian_validators::cpf::format;
///
/// assert_eq!(format("12345678909"), "123.456.789-09");
/// assert_eq!(format("123.456.789-09"), "123.456.789-09");
/// ```
pub fn format(cpf: &str) -> String {
    let cleaned = normalize(cpf);

    if cleaned.len() == 11 {
        format!(
            "{}.{}.{}-{}",
            &cleaned[0..3],
            &cleaned[3..6],
            &cleaned[6..9],
            &cleaned[9..11]
        )
    } else {
        cpf.to_string()
    }
}

/// Alias for format() for consistent API
pub fn format_cpf(cpf: &str) -> String {
    format(cpf)
}

/// Check if a string matches CPF format (does not validate check digits)
///
/// # Examples
/// ```
/// use pleme_brazilian_validators::cpf::is_cpf_format;
///
/// assert!(is_cpf_format("123.456.789-09"));
/// assert!(is_cpf_format("12345678909"));
/// assert!(!is_cpf_format("1234567890")); // 10 digits
/// ```
pub fn is_cpf_format(cpf: &str) -> bool {
    CPF_REGEX.is_match(cpf)
}

/// Validate CPF check digits using modulo 11 algorithm
fn validate_check_digits(cpf: &str) -> bool {
    let digits: Vec<u32> = cpf
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect();

    if digits.len() != 11 {
        return false;
    }

    // Calculate first check digit
    let mut sum = 0;
    for i in 0..9 {
        sum += digits[i] * (10 - i as u32);
    }
    let check1 = if sum % 11 < 2 { 0 } else { 11 - (sum % 11) };

    if check1 != digits[9] {
        return false;
    }

    // Calculate second check digit
    sum = 0;
    for i in 0..10 {
        sum += digits[i] * (11 - i as u32);
    }
    let check2 = if sum % 11 < 2 { 0 } else { 11 - (sum % 11) };

    check2 == digits[10]
}

/// Mask a CPF for display (shows first 3 and last 2 digits)
///
/// # Examples
/// ```
/// use pleme_brazilian_validators::cpf::mask;
///
/// assert_eq!(mask("12345678909"), "123.***.***-09");
/// ```
pub fn mask(cpf: &str) -> String {
    let cleaned = normalize(cpf);

    if cleaned.len() == 11 {
        format!("{}.***.***-{}", &cleaned[0..3], &cleaned[9..11])
    } else {
        cpf.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_valid_cpf() {
        // Valid CPF with formatting
        assert!(validate("123.456.789-09").is_ok());
        // Valid CPF without formatting
        assert!(validate("12345678909").is_ok());
    }

    #[test]
    fn test_validate_invalid_cpf() {
        // All same digits
        assert!(validate("111.111.111-11").is_err());
        assert!(validate("00000000000").is_err());

        // Invalid check digits
        assert!(validate("123.456.789-00").is_err());

        // Wrong length
        assert!(validate("1234567890").is_err());
        assert!(validate("123456789012").is_err());
    }

    #[test]
    fn test_normalize() {
        assert_eq!(normalize("123.456.789-09"), "12345678909");
        assert_eq!(normalize("12345678909"), "12345678909");
        assert_eq!(normalize("  123.456.789-09  "), "12345678909");
    }

    #[test]
    fn test_format() {
        assert_eq!(format("12345678909"), "123.456.789-09");
        assert_eq!(format("123.456.789-09"), "123.456.789-09");
        // Invalid length returns input
        assert_eq!(format("123"), "123");
    }

    #[test]
    fn test_is_cpf_format() {
        assert!(is_cpf_format("123.456.789-09"));
        assert!(is_cpf_format("12345678909"));
        assert!(!is_cpf_format("1234567890"));
        assert!(!is_cpf_format("abc.def.ghi-jk"));
    }

    #[test]
    fn test_mask() {
        assert_eq!(mask("12345678909"), "123.***.***-09");
        assert_eq!(mask("123.456.789-09"), "123.***.***-09");
    }
}
