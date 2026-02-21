//! Error types for Brazilian validators

use thiserror::Error;

/// Result type alias for Brazilian validation operations
pub type ValidationResult<T> = Result<T, BrazilianValidationError>;

/// Errors that can occur during Brazilian document validation
#[derive(Debug, Clone, Error, PartialEq, Eq)]
pub enum BrazilianValidationError {
    /// Invalid CPF (individual taxpayer ID)
    #[error("CPF inválido: {0}")]
    InvalidCpf(String),

    /// Invalid CNPJ (business taxpayer ID)
    #[error("CNPJ inválido: {0}")]
    InvalidCnpj(String),

    /// Invalid CEP (postal code)
    #[error("CEP inválido: {0}")]
    InvalidCep(String),

    /// Invalid phone number
    #[error("Telefone inválido: {0}")]
    InvalidPhone(String),

    /// Invalid PIX key
    #[error("Chave PIX inválida: {0}")]
    InvalidPixKey(String),

    /// Invalid document format (generic)
    #[error("Formato de documento inválido: {document_type}")]
    InvalidDocumentFormat { document_type: String },

    /// Document has invalid check digits
    #[error("Dígitos verificadores inválidos para {document_type}")]
    InvalidCheckDigits { document_type: String },

    /// Document contains invalid characters
    #[error("Caracteres inválidos no documento")]
    InvalidCharacters,

    /// Document has invalid length
    #[error("Tamanho inválido: esperado {expected}, recebido {actual}")]
    InvalidLength { expected: usize, actual: usize },
}

impl BrazilianValidationError {
    /// Create an invalid CPF error with a message
    pub fn invalid_cpf(msg: impl Into<String>) -> Self {
        Self::InvalidCpf(msg.into())
    }

    /// Create an invalid CNPJ error with a message
    pub fn invalid_cnpj(msg: impl Into<String>) -> Self {
        Self::InvalidCnpj(msg.into())
    }

    /// Create an invalid CEP error with a message
    pub fn invalid_cep(msg: impl Into<String>) -> Self {
        Self::InvalidCep(msg.into())
    }

    /// Create an invalid phone error with a message
    pub fn invalid_phone(msg: impl Into<String>) -> Self {
        Self::InvalidPhone(msg.into())
    }

    /// Create an invalid PIX key error with a message
    pub fn invalid_pix_key(msg: impl Into<String>) -> Self {
        Self::InvalidPixKey(msg.into())
    }

    /// Get error code for API responses
    pub fn error_code(&self) -> &'static str {
        match self {
            Self::InvalidCpf(_) => "INVALID_CPF",
            Self::InvalidCnpj(_) => "INVALID_CNPJ",
            Self::InvalidCep(_) => "INVALID_CEP",
            Self::InvalidPhone(_) => "INVALID_PHONE",
            Self::InvalidPixKey(_) => "INVALID_PIX_KEY",
            Self::InvalidDocumentFormat { .. } => "INVALID_DOCUMENT_FORMAT",
            Self::InvalidCheckDigits { .. } => "INVALID_CHECK_DIGITS",
            Self::InvalidCharacters => "INVALID_CHARACTERS",
            Self::InvalidLength { .. } => "INVALID_LENGTH",
        }
    }

    /// Get document type that failed validation
    pub fn document_type(&self) -> &str {
        match self {
            Self::InvalidCpf(_) => "CPF",
            Self::InvalidCnpj(_) => "CNPJ",
            Self::InvalidCep(_) => "CEP",
            Self::InvalidPhone(_) => "phone",
            Self::InvalidPixKey(_) => "PIX key",
            Self::InvalidDocumentFormat { document_type } => document_type,
            Self::InvalidCheckDigits { document_type } => document_type,
            Self::InvalidCharacters => "document",
            Self::InvalidLength { .. } => "document",
        }
    }
}

#[cfg(feature = "serialization")]
impl serde::Serialize for BrazilianValidationError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("BrazilianValidationError", 3)?;
        state.serialize_field("code", self.error_code())?;
        state.serialize_field("document_type", self.document_type())?;
        state.serialize_field("message", &self.to_string())?;
        state.end()
    }
}

#[cfg(feature = "graphql")]
impl From<BrazilianValidationError> for async_graphql::Error {
    fn from(err: BrazilianValidationError) -> Self {
        async_graphql::Error::new(err.to_string())
            .extend_with(|_, e| {
                e.set("code", err.error_code());
                e.set("document_type", err.document_type());
            })
    }
}
