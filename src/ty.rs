use e_utils::chrono::{DateTime, FixedOffset};

pub trait LicenseProtocol {
    fn generate(&self, days: u64) -> String;
    fn verify(&self, license: &str) -> e_utils::Result<DateTime<FixedOffset>>;
}

pub trait LicenseGenerator {
    fn generate_license(&self, user_id: &str, days: u64) -> e_utils::Result<LicenseInfo>;
    fn generate_batch_licenses(
        &self,
        user_ids: &[String],
        days: u64,
    ) -> e_utils::Result<Vec<LicenseInfo>>;
}

pub trait LicenseValidator {
    fn verify_license(&self, license_key: &str) -> e_utils::Result<LicenseValidationResult>;
    fn verify_batch_licenses(
        &self,
        licenses: &[LicenseInfo],
    ) -> e_utils::Result<Vec<LicenseValidationResult>>;
}

#[derive(Debug, Clone)]
pub struct LicenseInfo {
    pub user_id: String,
    pub license_key: String,
    pub expire_time: DateTime<FixedOffset>,
}

#[derive(Debug, Clone)]
pub struct LicenseValidationResult {
    pub user_id: String,
    pub is_valid: bool,
    pub message: String,
    pub days_remaining: i64,
}
