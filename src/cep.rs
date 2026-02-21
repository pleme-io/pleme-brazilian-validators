//! CEP (Código de Endereçamento Postal) validation and formatting
//!
//! Brazilian postal code with 8 digits.

use crate::error::{BrazilianValidationError, ValidationResult};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    /// Regex for CEP format (with or without hyphen)
    static ref CEP_REGEX: Regex = Regex::new(r"^\d{5}-?\d{3}$").unwrap();
}

/// Validate a Brazilian CEP (postal code)
///
/// # Arguments
/// * `cep` - CEP string (with or without hyphen)
///
/// # Returns
/// * `Ok(String)` - Normalized CEP (8 digits only)
/// * `Err(BrazilianValidationError)` - Validation error
///
/// # Examples
/// ```
/// use pleme_brazilian_validators::cep::validate;
///
/// assert!(validate("12345-678").is_ok());
/// assert!(validate("12345678").is_ok());
/// assert!(validate("12345").is_err()); // Too short
/// ```
pub fn validate(cep: &str) -> ValidationResult<String> {
    let cleaned = normalize(cep);

    // Check length
    if cleaned.len() != 8 {
        return Err(BrazilianValidationError::InvalidLength {
            expected: 8,
            actual: cleaned.len(),
        });
    }

    // Ensure all characters are digits
    if !cleaned.chars().all(|c| c.is_ascii_digit()) {
        return Err(BrazilianValidationError::InvalidCharacters);
    }

    // Check for invalid CEPs (all zeros is invalid)
    if cleaned == "00000000" {
        return Err(BrazilianValidationError::invalid_cep("CEP inválido"));
    }

    Ok(cleaned)
}

/// Alias for validate() for consistent API
pub fn validate_cep(cep: &str) -> ValidationResult<String> {
    validate(cep)
}

/// Normalize a CEP string by removing all non-digit characters
///
/// # Examples
/// ```
/// use pleme_brazilian_validators::cep::normalize;
///
/// assert_eq!(normalize("12345-678"), "12345678");
/// assert_eq!(normalize("12345678"), "12345678");
/// ```
pub fn normalize(cep: &str) -> String {
    cep.chars().filter(|c| c.is_ascii_digit()).collect()
}

/// Alias for normalize() for consistent API
pub fn normalize_cep(cep: &str) -> String {
    normalize(cep)
}

/// Format a CEP string with standard punctuation (XXXXX-XXX)
///
/// # Arguments
/// * `cep` - CEP string (normalized or formatted)
///
/// # Returns
/// Formatted CEP string. Returns input unchanged if not 8 digits.
///
/// # Examples
/// ```
/// use pleme_brazilian_validators::cep::format;
///
/// assert_eq!(format("12345678"), "12345-678");
/// assert_eq!(format("12345-678"), "12345-678");
/// ```
pub fn format(cep: &str) -> String {
    let cleaned = normalize(cep);

    if cleaned.len() == 8 {
        format!("{}-{}", &cleaned[0..5], &cleaned[5..8])
    } else {
        cep.to_string()
    }
}

/// Alias for format() for consistent API
pub fn format_cep(cep: &str) -> String {
    format(cep)
}

/// Check if a string matches CEP format (does not validate if CEP exists)
///
/// # Examples
/// ```
/// use pleme_brazilian_validators::cep::is_cep_format;
///
/// assert!(is_cep_format("12345-678"));
/// assert!(is_cep_format("12345678"));
/// assert!(!is_cep_format("12345")); // 5 digits
/// ```
pub fn is_cep_format(cep: &str) -> bool {
    CEP_REGEX.is_match(cep)
}

/// Extract the region code (first digit) from CEP
///
/// Brazilian CEP regions:
/// - 0: Grande São Paulo
/// - 1: Interior de São Paulo
/// - 2: Rio de Janeiro e Espírito Santo
/// - 3: Minas Gerais
/// - 4: Bahia e Sergipe
/// - 5: Pernambuco, Alagoas, Paraíba e Rio Grande do Norte
/// - 6: Ceará, Piauí, Maranhão, Pará, Amazonas, Acre, Amapá e Roraima
/// - 7: Distrito Federal, Goiás, Tocantins, Mato Grosso, Mato Grosso do Sul e Rondônia
/// - 8: Paraná e Santa Catarina
/// - 9: Rio Grande do Sul
///
/// # Examples
/// ```
/// use pleme_brazilian_validators::cep::extract_region;
///
/// assert_eq!(extract_region("01310-100"), Some(0)); // São Paulo capital
/// assert_eq!(extract_region("20040-020"), Some(2)); // Rio de Janeiro
/// ```
pub fn extract_region(cep: &str) -> Option<u8> {
    let cleaned = normalize(cep);

    if cleaned.len() >= 1 {
        cleaned.chars().next()?.to_digit(10).map(|d| d as u8)
    } else {
        None
    }
}

/// Get region name from CEP
///
/// # Examples
/// ```
/// use pleme_brazilian_validators::cep::get_region_name;
///
/// assert_eq!(get_region_name("01310-100"), Some("Grande São Paulo"));
/// assert_eq!(get_region_name("20040-020"), Some("Rio de Janeiro e Espírito Santo"));
/// ```
pub fn get_region_name(cep: &str) -> Option<&'static str> {
    extract_region(cep).map(|region| match region {
        0 => "Grande São Paulo",
        1 => "Interior de São Paulo",
        2 => "Rio de Janeiro e Espírito Santo",
        3 => "Minas Gerais",
        4 => "Bahia e Sergipe",
        5 => "Pernambuco, Alagoas, Paraíba e Rio Grande do Norte",
        6 => "Ceará, Piauí, Maranhão, Pará, Amazonas, Acre, Amapá e Roraima",
        7 => "Distrito Federal, Goiás, Tocantins, Mato Grosso, Mato Grosso do Sul e Rondônia",
        8 => "Paraná e Santa Catarina",
        9 => "Rio Grande do Sul",
        _ => "Região desconhecida",
    })
}

/// Extract the sub-region code (first 2 digits) from CEP
///
/// # Examples
/// ```
/// use pleme_brazilian_validators::cep::extract_subregion;
///
/// assert_eq!(extract_subregion("01310-100"), Some("01".to_string()));
/// ```
pub fn extract_subregion(cep: &str) -> Option<String> {
    let cleaned = normalize(cep);

    if cleaned.len() >= 2 {
        Some(cleaned[0..2].to_string())
    } else {
        None
    }
}

/// Extract the sector code (first 5 digits) from CEP
///
/// # Examples
/// ```
/// use pleme_brazilian_validators::cep::extract_sector;
///
/// assert_eq!(extract_sector("01310-100"), Some("01310".to_string()));
/// ```
pub fn extract_sector(cep: &str) -> Option<String> {
    let cleaned = normalize(cep);

    if cleaned.len() >= 5 {
        Some(cleaned[0..5].to_string())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_valid_cep() {
        assert!(validate("12345-678").is_ok());
        assert!(validate("12345678").is_ok());
        assert!(validate("01310-100").is_ok());
    }

    #[test]
    fn test_validate_invalid_cep() {
        // Wrong length
        assert!(validate("12345").is_err());
        assert!(validate("123456789").is_err());

        // All zeros
        assert!(validate("00000000").is_err());
    }

    #[test]
    fn test_normalize() {
        assert_eq!(normalize("12345-678"), "12345678");
        assert_eq!(normalize("12345678"), "12345678");
        assert_eq!(normalize("  12345-678  "), "12345678");
    }

    #[test]
    fn test_format() {
        assert_eq!(format("12345678"), "12345-678");
        assert_eq!(format("12345-678"), "12345-678");
        // Invalid length returns input
        assert_eq!(format("12345"), "12345");
    }

    #[test]
    fn test_is_cep_format() {
        assert!(is_cep_format("12345-678"));
        assert!(is_cep_format("12345678"));
        assert!(!is_cep_format("12345"));
        assert!(!is_cep_format("123456789"));
    }

    #[test]
    fn test_extract_region() {
        assert_eq!(extract_region("01310-100"), Some(0));
        assert_eq!(extract_region("20040-020"), Some(2));
        assert_eq!(extract_region("90000-000"), Some(9));
    }

    #[test]
    fn test_get_region_name() {
        assert_eq!(get_region_name("01310-100"), Some("Grande São Paulo"));
        assert_eq!(
            get_region_name("20040-020"),
            Some("Rio de Janeiro e Espírito Santo")
        );
    }

    #[test]
    fn test_extract_sector() {
        assert_eq!(extract_sector("01310-100"), Some("01310".to_string()));
    }
}
