//! Brazilian Document and Format Validators
//!
//! This library provides validation and formatting utilities for Brazilian documents
//! and formats commonly used in e-commerce and financial applications.
//!
//! # Features
//!
//! - **CPF**: Brazilian individual taxpayer ID (Cadastro de Pessoas Físicas)
//! - **CNPJ**: Brazilian business taxpayer ID (Cadastro Nacional de Pessoa Jurídica)
//! - **CEP**: Brazilian postal code (Código de Endereçamento Postal)
//! - **Phone**: Brazilian phone numbers with regional codes
//! - **PIX**: Brazilian instant payment system keys
//!
//! # Example
//!
//! ```rust
//! use pleme_brazilian_validators::{cpf, cnpj, cep, phone, pix};
//!
//! // Validate CPF
//! assert!(cpf::validate("123.456.789-09").is_ok());
//!
//! // Format CPF
//! assert_eq!(cpf::format("12345678909"), "123.456.789-09");
//!
//! // Normalize (remove formatting)
//! assert_eq!(cpf::normalize("123.456.789-09"), "12345678909");
//! ```

pub mod cpf;
pub mod cnpj;
pub mod cep;
pub mod phone;
pub mod pix;
pub mod error;

pub use error::{BrazilianValidationError, ValidationResult};

// Re-export main functions for convenience
pub use cpf::{validate, validate as validate_cpf, format as format_cpf, normalize as normalize_cpf};
pub use cnpj::{validate as validate_cnpj, format as format_cnpj, normalize as normalize_cnpj};
pub use cep::{validate as validate_cep, format as format_cep, normalize as normalize_cep};
pub use phone::{validate as validate_phone, format as format_phone, normalize as normalize_phone};
pub use pix::validate_pix_key;
