//! CNPJ (Cadastro Nacional da Pessoa Jurídica) validation and formatting
//!
//! Brazilian business taxpayer identification number with 14 digits
//! and two check digits calculated using weighted modulo 11.

use crate::error::{BrazilianValidationError, ValidationResult};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    /// Regex for CNPJ format (with or without punctuation)
    static ref CNPJ_REGEX: Regex = Regex::new(r"^\d{2}\.?\d{3}\.?\d{3}/?\d{4}-?\d{2}$").unwrap();
}

/// Known invalid CNPJs (all same digits)
const INVALID_CNPJS: [&str; 10] = [
    "00000000000000",
    "11111111111111",
    "22222222222222",
    "33333333333333",
    "44444444444444",
    "55555555555555",
    "66666666666666",
    "77777777777777",
    "88888888888888",
    "99999999999999",
];

/// Validate a Brazilian CNPJ number
///
/// Validates format, length, check digits, and rejects known invalid sequences.
///
/// # Arguments
/// * `cnpj` - CNPJ string (with or without punctuation)
///
/// # Returns
/// * `Ok(String)` - Normalized CNPJ (14 digits only)
/// * `Err(BrazilianValidationError)` - Validation error
///
/// # Examples
/// ```
/// use pleme_brazilian_validators::cnpj::validate;
///
/// assert!(validate("11.222.333/0001-81").is_ok());
/// assert!(validate("11222333000181").is_ok());
/// assert!(validate("11.111.111/1111-11").is_err()); // All same digits
/// ```
pub fn validate(cnpj: &str) -> ValidationResult<String> {
    let cleaned = normalize(cnpj);

    // Check length
    if cleaned.len() != 14 {
        return Err(BrazilianValidationError::InvalidLength {
            expected: 14,
            actual: cleaned.len(),
        });
    }

    // Ensure all characters are digits
    if !cleaned.chars().all(|c| c.is_ascii_digit()) {
        return Err(BrazilianValidationError::InvalidCharacters);
    }

    // Check for known invalid CNPJs
    if INVALID_CNPJS.contains(&cleaned.as_str()) {
        return Err(BrazilianValidationError::invalid_cnpj(
            "sequência de dígitos repetidos",
        ));
    }

    // Validate check digits
    if !validate_check_digits(&cleaned) {
        return Err(BrazilianValidationError::InvalidCheckDigits {
            document_type: "CNPJ".to_string(),
        });
    }

    Ok(cleaned)
}

/// Alias for validate() for consistent API
pub fn validate_cnpj(cnpj: &str) -> ValidationResult<String> {
    validate(cnpj)
}

/// Normalize a CNPJ string by removing all non-digit characters
///
/// # Examples
/// ```
/// use pleme_brazilian_validators::cnpj::normalize;
///
/// assert_eq!(normalize("11.222.333/0001-81"), "11222333000181");
/// assert_eq!(normalize("11222333000181"), "11222333000181");
/// ```
pub fn normalize(cnpj: &str) -> String {
    cnpj.chars().filter(|c| c.is_ascii_digit()).collect()
}

/// Alias for normalize() for consistent API
pub fn normalize_cnpj(cnpj: &str) -> String {
    normalize(cnpj)
}

/// Format a CNPJ string with standard punctuation (XX.XXX.XXX/XXXX-XX)
///
/// # Arguments
/// * `cnpj` - CNPJ string (normalized or formatted)
///
/// # Returns
/// Formatted CNPJ string. Returns input unchanged if not 14 digits.
///
/// # Examples
/// ```
/// use pleme_brazilian_validators::cnpj::format;
///
/// assert_eq!(format("11222333000181"), "11.222.333/0001-81");
/// assert_eq!(format("11.222.333/0001-81"), "11.222.333/0001-81");
/// ```
pub fn format(cnpj: &str) -> String {
    let cleaned = normalize(cnpj);

    if cleaned.len() == 14 {
        format!(
            "{}.{}.{}/{}-{}",
            &cleaned[0..2],
            &cleaned[2..5],
            &cleaned[5..8],
            &cleaned[8..12],
            &cleaned[12..14]
        )
    } else {
        cnpj.to_string()
    }
}

/// Alias for format() for consistent API
pub fn format_cnpj(cnpj: &str) -> String {
    format(cnpj)
}

/// Check if a string matches CNPJ format (does not validate check digits)
///
/// # Examples
/// ```
/// use pleme_brazilian_validators::cnpj::is_cnpj_format;
///
/// assert!(is_cnpj_format("11.222.333/0001-81"));
/// assert!(is_cnpj_format("11222333000181"));
/// assert!(!is_cnpj_format("1122233300018")); // 13 digits
/// ```
pub fn is_cnpj_format(cnpj: &str) -> bool {
    CNPJ_REGEX.is_match(cnpj)
}

/// Validate CNPJ check digits using weighted modulo 11 algorithm
fn validate_check_digits(cnpj: &str) -> bool {
    let digits: Vec<u32> = cnpj
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect();

    if digits.len() != 14 {
        return false;
    }

    // Weights for first check digit
    let weights1 = [5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];

    // Calculate first check digit
    let mut sum = 0;
    for i in 0..12 {
        sum += digits[i] * weights1[i];
    }
    let check1 = if sum % 11 < 2 { 0 } else { 11 - (sum % 11) };

    if check1 != digits[12] {
        return false;
    }

    // Weights for second check digit
    let weights2 = [6, 5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];

    // Calculate second check digit
    sum = 0;
    for i in 0..13 {
        sum += digits[i] * weights2[i];
    }
    let check2 = if sum % 11 < 2 { 0 } else { 11 - (sum % 11) };

    check2 == digits[13]
}

/// Mask a CNPJ for display (shows first 2 and last 2 digits)
///
/// # Examples
/// ```
/// use pleme_brazilian_validators::cnpj::mask;
///
/// assert_eq!(mask("11222333000181"), "11.***.***/**01-81");
/// ```
pub fn mask(cnpj: &str) -> String {
    let cleaned = normalize(cnpj);

    if cleaned.len() == 14 {
        format!(
            "{}.***.***/**{}-{}",
            &cleaned[0..2],
            &cleaned[10..12],
            &cleaned[12..14]
        )
    } else {
        cnpj.to_string()
    }
}

/// Extract the base CNPJ (first 8 digits - company identifier)
///
/// # Examples
/// ```
/// use pleme_brazilian_validators::cnpj::extract_base;
///
/// assert_eq!(extract_base("11222333000181"), Some("11222333".to_string()));
/// ```
pub fn extract_base(cnpj: &str) -> Option<String> {
    let cleaned = normalize(cnpj);

    if cleaned.len() == 14 {
        Some(cleaned[0..8].to_string())
    } else {
        None
    }
}

/// Extract the branch number (digits 9-12)
///
/// # Examples
/// ```
/// use pleme_brazilian_validators::cnpj::extract_branch;
///
/// assert_eq!(extract_branch("11222333000181"), Some("0001".to_string()));
/// ```
pub fn extract_branch(cnpj: &str) -> Option<String> {
    let cleaned = normalize(cnpj);

    if cleaned.len() == 14 {
        Some(cleaned[8..12].to_string())
    } else {
        None
    }
}

/// Check if CNPJ is for the main branch (0001)
///
/// # Examples
/// ```
/// use pleme_brazilian_validators::cnpj::is_main_branch;
///
/// assert!(is_main_branch("11222333000181"));
/// assert!(!is_main_branch("11222333000281"));
/// ```
pub fn is_main_branch(cnpj: &str) -> bool {
    extract_branch(cnpj).map_or(false, |branch| branch == "0001")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_valid_cnpj() {
        // Valid CNPJ with formatting
        assert!(validate("11.222.333/0001-81").is_ok());
        // Valid CNPJ without formatting
        assert!(validate("11222333000181").is_ok());
    }

    #[test]
    fn test_validate_invalid_cnpj() {
        // All same digits
        assert!(validate("11.111.111/1111-11").is_err());
        assert!(validate("00000000000000").is_err());

        // Invalid check digits
        assert!(validate("11.222.333/0001-00").is_err());

        // Wrong length
        assert!(validate("1122233300018").is_err());
        assert!(validate("112223330001812").is_err());
    }

    #[test]
    fn test_normalize() {
        assert_eq!(normalize("11.222.333/0001-81"), "11222333000181");
        assert_eq!(normalize("11222333000181"), "11222333000181");
    }

    #[test]
    fn test_format() {
        assert_eq!(format("11222333000181"), "11.222.333/0001-81");
        assert_eq!(format("11.222.333/0001-81"), "11.222.333/0001-81");
        // Invalid length returns input
        assert_eq!(format("123"), "123");
    }

    #[test]
    fn test_is_cnpj_format() {
        assert!(is_cnpj_format("11.222.333/0001-81"));
        assert!(is_cnpj_format("11222333000181"));
        assert!(!is_cnpj_format("1122233300018"));
    }

    #[test]
    fn test_mask() {
        assert_eq!(mask("11222333000181"), "11.***.***/**01-81");
    }

    #[test]
    fn test_extract_base() {
        assert_eq!(extract_base("11222333000181"), Some("11222333".to_string()));
        assert_eq!(extract_base("invalid"), None);
    }

    #[test]
    fn test_extract_branch() {
        assert_eq!(extract_branch("11222333000181"), Some("0001".to_string()));
        assert_eq!(extract_branch("11222333000281"), Some("0002".to_string()));
    }

    #[test]
    fn test_is_main_branch() {
        assert!(is_main_branch("11222333000181"));
        assert!(!is_main_branch("11222333000281"));
    }
}
