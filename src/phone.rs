//! Brazilian phone number validation and formatting
//!
//! Supports landline and mobile numbers with area codes (DDD).

use crate::error::{BrazilianValidationError, ValidationResult};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    /// Regex for Brazilian phone format (various formats accepted)
    /// Matches: +55 11 98765-4321, (11) 98765-4321, 11987654321, etc.
    static ref PHONE_REGEX: Regex = Regex::new(
        r"^(\+55\s?)?(\(?\d{2}\)?\s?)?(\d{4,5}[-\s]?\d{4})$"
    ).unwrap();

    /// Strict regex for normalized phone (digits only with optional +)
    static ref NORMALIZED_PHONE_REGEX: Regex = Regex::new(r"^\+?55?\d{10,11}$").unwrap();
}

/// Valid Brazilian area codes (DDD)
const VALID_DDDS: [&str; 67] = [
    // São Paulo
    "11", "12", "13", "14", "15", "16", "17", "18", "19",
    // Rio de Janeiro e Espírito Santo
    "21", "22", "24", "27", "28",
    // Minas Gerais
    "31", "32", "33", "34", "35", "37", "38",
    // Paraná
    "41", "42", "43", "44", "45", "46",
    // Santa Catarina
    "47", "48", "49",
    // Rio Grande do Sul
    "51", "53", "54", "55",
    // Distrito Federal, Goiás, Tocantins, Mato Grosso, Mato Grosso do Sul, Acre, Rondônia
    "61", "62", "63", "64", "65", "66", "67", "68", "69",
    // Bahia e Sergipe
    "71", "73", "74", "75", "77", "79",
    // Pernambuco, Alagoas, Paraíba, Rio Grande do Norte
    "81", "82", "83", "84", "85", "86", "87", "88", "89",
    // Pará, Amazonas, Roraima, Amapá, Maranhão, Piauí, Ceará
    "91", "92", "93", "94", "95", "96", "97", "98", "99",
];

/// Validate a Brazilian phone number
///
/// # Arguments
/// * `phone` - Phone string in various formats
///
/// # Returns
/// * `Ok(String)` - Normalized phone (digits only with country code)
/// * `Err(BrazilianValidationError)` - Validation error
///
/// # Examples
/// ```
/// use pleme_brazilian_validators::phone::validate;
///
/// assert!(validate("+55 11 98765-4321").is_ok());
/// assert!(validate("(11) 98765-4321").is_ok());
/// assert!(validate("11987654321").is_ok());
/// assert!(validate("12345").is_err()); // Too short
/// ```
pub fn validate(phone: &str) -> ValidationResult<String> {
    let cleaned = normalize(phone);

    // Remove country code for length check
    let without_country = if cleaned.starts_with("+55") {
        &cleaned[3..]
    } else if cleaned.starts_with("55") && cleaned.len() > 11 {
        &cleaned[2..]
    } else {
        &cleaned
    };

    // Check length (10 for landline, 11 for mobile)
    if without_country.len() < 10 || without_country.len() > 11 {
        return Err(BrazilianValidationError::InvalidLength {
            expected: 10, // or 11 for mobile
            actual: without_country.len(),
        });
    }

    // Validate DDD (area code)
    let ddd = &without_country[0..2];
    if !VALID_DDDS.contains(&ddd) {
        return Err(BrazilianValidationError::invalid_phone(format!(
            "DDD {} inválido",
            ddd
        )));
    }

    // Mobile numbers must start with 9
    if without_country.len() == 11 && !without_country[2..].starts_with('9') {
        return Err(BrazilianValidationError::invalid_phone(
            "celular deve começar com 9",
        ));
    }

    // Return with country code
    Ok(format!("+55{}", without_country))
}

/// Alias for validate() for consistent API
pub fn validate_phone(phone: &str) -> ValidationResult<String> {
    validate(phone)
}

/// Normalize a phone string by removing all non-digit characters (keeps +)
///
/// # Examples
/// ```
/// use pleme_brazilian_validators::phone::normalize;
///
/// assert_eq!(normalize("+55 11 98765-4321"), "+5511987654321");
/// assert_eq!(normalize("(11) 98765-4321"), "11987654321");
/// ```
pub fn normalize(phone: &str) -> String {
    phone
        .chars()
        .filter(|c| c.is_ascii_digit() || *c == '+')
        .collect()
}

/// Alias for normalize() for consistent API
pub fn normalize_phone(phone: &str) -> String {
    normalize(phone)
}

/// Format a phone string with standard Brazilian formatting
///
/// # Arguments
/// * `phone` - Phone string (normalized or formatted)
///
/// # Returns
/// Formatted phone string. Returns input unchanged if invalid length.
///
/// # Examples
/// ```
/// use pleme_brazilian_validators::phone::format;
///
/// assert_eq!(format("11987654321"), "(11) 98765-4321");
/// assert_eq!(format("+5511987654321"), "+55 (11) 98765-4321");
/// assert_eq!(format("1134567890"), "(11) 3456-7890");
/// ```
pub fn format(phone: &str) -> String {
    let cleaned = normalize(phone);

    // Handle +55 prefix
    let (prefix, number) = if cleaned.starts_with("+55") {
        ("+55 ", &cleaned[3..])
    } else if cleaned.starts_with("55") && cleaned.len() > 11 {
        ("+55 ", &cleaned[2..])
    } else {
        ("", cleaned.as_str())
    };

    match number.len() {
        // Mobile: (XX) XXXXX-XXXX
        11 => format!(
            "{}({}) {}-{}",
            prefix,
            &number[0..2],
            &number[2..7],
            &number[7..11]
        ),
        // Landline: (XX) XXXX-XXXX
        10 => format!(
            "{}({}) {}-{}",
            prefix,
            &number[0..2],
            &number[2..6],
            &number[6..10]
        ),
        _ => phone.to_string(),
    }
}

/// Alias for format() for consistent API
pub fn format_phone(phone: &str) -> String {
    format(phone)
}

/// Check if a phone number is a mobile number
///
/// # Examples
/// ```
/// use pleme_brazilian_validators::phone::is_mobile;
///
/// assert!(is_mobile("11987654321")); // Mobile (9 after DDD)
/// assert!(!is_mobile("1134567890")); // Landline (no 9)
/// ```
pub fn is_mobile(phone: &str) -> bool {
    let cleaned = normalize(phone);

    // Remove country code
    let without_country = if cleaned.starts_with("+55") {
        &cleaned[3..]
    } else if cleaned.starts_with("55") && cleaned.len() > 11 {
        &cleaned[2..]
    } else {
        &cleaned
    };

    // Mobile has 11 digits and starts with 9 after DDD
    without_country.len() == 11 && without_country[2..].starts_with('9')
}

/// Check if a phone number is a landline
///
/// # Examples
/// ```
/// use pleme_brazilian_validators::phone::is_landline;
///
/// assert!(!is_landline("11987654321")); // Mobile
/// assert!(is_landline("1134567890")); // Landline
/// ```
pub fn is_landline(phone: &str) -> bool {
    let cleaned = normalize(phone);

    // Remove country code
    let without_country = if cleaned.starts_with("+55") {
        &cleaned[3..]
    } else if cleaned.starts_with("55") && cleaned.len() > 11 {
        &cleaned[2..]
    } else {
        &cleaned
    };

    // Landline has 10 digits
    without_country.len() == 10
}

/// Extract the DDD (area code) from a phone number
///
/// # Examples
/// ```
/// use pleme_brazilian_validators::phone::extract_ddd;
///
/// assert_eq!(extract_ddd("11987654321"), Some("11".to_string()));
/// assert_eq!(extract_ddd("+55 21 98765-4321"), Some("21".to_string()));
/// ```
pub fn extract_ddd(phone: &str) -> Option<String> {
    let cleaned = normalize(phone);

    // Remove country code
    let without_country = if cleaned.starts_with("+55") {
        &cleaned[3..]
    } else if cleaned.starts_with("55") && cleaned.len() > 11 {
        &cleaned[2..]
    } else {
        &cleaned
    };

    if without_country.len() >= 2 {
        Some(without_country[0..2].to_string())
    } else {
        None
    }
}

/// Get the state(s) for a given DDD
///
/// # Examples
/// ```
/// use pleme_brazilian_validators::phone::get_state_for_ddd;
///
/// assert_eq!(get_state_for_ddd("11"), Some("São Paulo (Capital e Grande SP)"));
/// assert_eq!(get_state_for_ddd("21"), Some("Rio de Janeiro (Capital e Região)"));
/// ```
pub fn get_state_for_ddd(ddd: &str) -> Option<&'static str> {
    match ddd {
        // São Paulo
        "11" => Some("São Paulo (Capital e Grande SP)"),
        "12" => Some("São Paulo (Vale do Paraíba)"),
        "13" => Some("São Paulo (Baixada Santista)"),
        "14" => Some("São Paulo (Bauru)"),
        "15" => Some("São Paulo (Sorocaba)"),
        "16" => Some("São Paulo (Ribeirão Preto)"),
        "17" => Some("São Paulo (São José do Rio Preto)"),
        "18" => Some("São Paulo (Presidente Prudente)"),
        "19" => Some("São Paulo (Campinas)"),
        // Rio de Janeiro
        "21" => Some("Rio de Janeiro (Capital e Região)"),
        "22" => Some("Rio de Janeiro (Interior)"),
        "24" => Some("Rio de Janeiro (Petrópolis)"),
        // Espírito Santo
        "27" | "28" => Some("Espírito Santo"),
        // Minas Gerais
        "31" => Some("Minas Gerais (BH e Região)"),
        "32" | "33" | "34" | "35" | "37" | "38" => Some("Minas Gerais"),
        // Paraná
        "41" => Some("Paraná (Curitiba e Região)"),
        "42" | "43" | "44" | "45" | "46" => Some("Paraná"),
        // Santa Catarina
        "47" | "48" | "49" => Some("Santa Catarina"),
        // Rio Grande do Sul
        "51" => Some("Rio Grande do Sul (Porto Alegre)"),
        "53" | "54" | "55" => Some("Rio Grande do Sul"),
        // Centro-Oeste
        "61" => Some("Distrito Federal"),
        "62" => Some("Goiás (Goiânia)"),
        "63" => Some("Tocantins"),
        "64" => Some("Goiás"),
        "65" | "66" => Some("Mato Grosso"),
        "67" => Some("Mato Grosso do Sul"),
        // Norte
        "68" => Some("Acre"),
        "69" => Some("Rondônia"),
        "91" | "93" | "94" => Some("Pará"),
        "92" | "97" => Some("Amazonas"),
        "95" => Some("Roraima"),
        "96" => Some("Amapá"),
        // Nordeste
        "71" => Some("Bahia (Salvador)"),
        "73" | "74" | "75" | "77" => Some("Bahia"),
        "79" => Some("Sergipe"),
        "81" => Some("Pernambuco (Recife)"),
        "82" => Some("Alagoas"),
        "83" => Some("Paraíba"),
        "84" => Some("Rio Grande do Norte"),
        "85" | "88" => Some("Ceará"),
        "86" | "89" => Some("Piauí"),
        "87" => Some("Pernambuco"),
        "98" | "99" => Some("Maranhão"),
        _ => None,
    }
}

/// Mask a phone number for display
///
/// # Examples
/// ```
/// use pleme_brazilian_validators::phone::mask;
///
/// assert_eq!(mask("11987654321"), "(11) *****-4321");
/// ```
pub fn mask(phone: &str) -> String {
    let cleaned = normalize(phone);

    // Remove country code
    let without_country = if cleaned.starts_with("+55") {
        &cleaned[3..]
    } else if cleaned.starts_with("55") && cleaned.len() > 11 {
        &cleaned[2..]
    } else {
        &cleaned
    };

    match without_country.len() {
        11 => format!("({}) *****-{}", &without_country[0..2], &without_country[7..11]),
        10 => format!("({}) ****-{}", &without_country[0..2], &without_country[6..10]),
        _ => phone.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_valid_phone() {
        // Mobile with various formats
        assert!(validate("+55 11 98765-4321").is_ok());
        assert!(validate("(11) 98765-4321").is_ok());
        assert!(validate("11987654321").is_ok());

        // Landline
        assert!(validate("1134567890").is_ok());
        assert!(validate("(11) 3456-7890").is_ok());
    }

    #[test]
    fn test_validate_invalid_phone() {
        // Too short
        assert!(validate("12345").is_err());

        // Invalid DDD
        assert!(validate("00987654321").is_err());

        // Mobile without 9
        assert!(validate("11887654321").is_err());
    }

    #[test]
    fn test_normalize() {
        assert_eq!(normalize("+55 11 98765-4321"), "+5511987654321");
        assert_eq!(normalize("(11) 98765-4321"), "11987654321");
    }

    #[test]
    fn test_format() {
        assert_eq!(format("11987654321"), "(11) 98765-4321");
        assert_eq!(format("+5511987654321"), "+55 (11) 98765-4321");
        assert_eq!(format("1134567890"), "(11) 3456-7890");
    }

    #[test]
    fn test_is_mobile() {
        assert!(is_mobile("11987654321"));
        assert!(!is_mobile("1134567890"));
    }

    #[test]
    fn test_is_landline() {
        assert!(!is_landline("11987654321"));
        assert!(is_landline("1134567890"));
    }

    #[test]
    fn test_extract_ddd() {
        assert_eq!(extract_ddd("11987654321"), Some("11".to_string()));
        assert_eq!(extract_ddd("+55 21 98765-4321"), Some("21".to_string()));
    }

    #[test]
    fn test_get_state_for_ddd() {
        assert_eq!(
            get_state_for_ddd("11"),
            Some("São Paulo (Capital e Grande SP)")
        );
        assert_eq!(
            get_state_for_ddd("21"),
            Some("Rio de Janeiro (Capital e Região)")
        );
        assert_eq!(get_state_for_ddd("00"), None);
    }

    #[test]
    fn test_mask() {
        assert_eq!(mask("11987654321"), "(11) *****-4321");
        assert_eq!(mask("1134567890"), "(11) ****-7890");
    }
}
