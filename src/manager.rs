use crate::ty::*;
use e_utils::chrono::{FixedOffset, Utc};
use std::sync::Arc;

pub struct LicenseManager {
    protocol: Arc<dyn LicenseProtocol + Send + Sync>,
    timezone: FixedOffset,
}

impl LicenseManager {
    pub fn new(protocol: Arc<dyn LicenseProtocol + Send + Sync>, timezone_offset_hours: i32) -> e_utils::Result<Self> {
        let timezone = FixedOffset::east_opt(timezone_offset_hours * 3600)
            .ok_or("Invalid timezone offset")?;
        
        Ok(Self {
            protocol,
            timezone,
        })
    }
}

impl LicenseGenerator for LicenseManager {
    fn generate_license(&self, user_id: &str, days: u64) -> e_utils::Result<LicenseInfo> {
        let license_key = self.protocol.generate(days);
        let expire_time = self.protocol.verify(&license_key)?;
        
        Ok(LicenseInfo {
            user_id: user_id.to_string(),
            license_key,
            expire_time,
        })
    }

    fn generate_batch_licenses(&self, user_ids: &[String], days: u64) -> e_utils::Result<Vec<LicenseInfo>> {
        user_ids
            .iter()
            .map(|user_id| self.generate_license(user_id, days))
            .collect()
    }
}

impl LicenseValidator for LicenseManager {
    fn verify_license(&self, license_key: &str) -> e_utils::Result<LicenseValidationResult> {
        let now = Utc::now().with_timezone(&self.timezone);
        
        match self.protocol.verify(license_key) {
            Ok(expire) => {
                let days_remaining = expire.signed_duration_since(now).num_days();
                Ok(LicenseValidationResult {
                    user_id: String::new(), // 单个验证时可能无法获取用户ID
                    is_valid: days_remaining > 0,
                    message: if days_remaining > 0 {
                        format!("授权有效，过期时间: {}", expire)
                    } else {
                        "授权已过期".to_string()
                    },
                    days_remaining,
                })
            }
            Err(e) => Ok(LicenseValidationResult {
                user_id: String::new(),
                is_valid: false,
                message: format!("授权无效: {}", e),
                days_remaining: 0,
            })
        }
    }

    fn verify_batch_licenses(&self, licenses: &[LicenseInfo]) -> e_utils::Result<Vec<LicenseValidationResult>> {
        licenses
            .iter()
            .map(|license| {
                let mut result = self.verify_license(&license.license_key)?;
                result.user_id = license.user_id.clone();
                Ok(result)
            })
            .collect()
    }
}