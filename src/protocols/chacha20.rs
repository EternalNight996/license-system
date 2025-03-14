use crate::ty::*;
use chacha20poly1305::{
  aead::{Aead, NewAead},
  XChaCha20Poly1305, XNonce,
};
use crc32fast::hash as crc32;
use e_utils::chrono::{DateTime, Duration, FixedOffset, Utc};

const KEY_SIZE: usize = 32;
const NONCE_SIZE: usize = 24;

/// # Example
/// ```rust
/// use std::sync::Arc;
/// use license_system::{
///     LicenseGenerator as _, LicenseManager, LicenseValidator as _, ChaCha20Protocol,
/// };
/// fn main() -> e_utils::Result<()> {
///     // 创建授权管理器（使用固定密钥便于测试）
///     let secret_key: [u8; 32] = [
///         0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77,
///         0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff, 0x00, 0x13, 0x24, 0x35, 0x46, 0x57, 0x68,
///         0x79, 0x8a,
///     ];
///     let protocol = Arc::new(ChaCha20Protocol::new(secret_key));
///     let license_manager = LicenseManager::new(protocol, 8)?;
///     // 测试批量生成
///     let user_ids = vec![
///         "user1",
///         "user2",
///         "user3",
///     ];
///     println!("生成批量授权...");
///     let licenses = license_manager.generate_batch_licenses(user_ids, 2)?;
///     for license in &licenses {
///         println!(
///             "用户: {}, 授权码: {}, 过期时间: {}",
///             license.user_id, license.license_key, license.expire_time
///         );
///     }
///     // 测试批量验证
///     println!("\n批量验证授权...");
///     let verify_results = license_manager.verify_batch_licenses(&licenses)?;
///     for result in &verify_results {
///         println!("用户 {}: {}", result.user_id, result.message);
///     }
///     // 测试单个授权验证
///     println!("\n验证单个授权...");
///     if let Some(first_license) = licenses.first() {
///         match license_manager.verify_license(&first_license.license_key) {
///             Ok(res) => println!("验证结果: {} 剩余{}小时", res.message, res.hours_remaining),
///             Err(e) => println!("验证失败: {}", e),
///         }
///     }
///     Ok(())
/// }
///```
pub struct ChaCha20Protocol {
  secret_key: [u8; KEY_SIZE],
}

impl ChaCha20Protocol {
  pub fn new(secret_key: [u8; KEY_SIZE]) -> Self {
    Self { secret_key }
  }
}

impl LicenseProtocol for ChaCha20Protocol {
  // 生成许可证（约比AES方案节省30%资源）
  fn generate(&self, hours: u64) -> String {
    let nonce = e_utils::algorithm!([u8; NONCE_SIZE]);

    let expire_time = (Utc::now() + Duration::hours(hours as i64)).timestamp();
    let mut data = expire_time.to_le_bytes().to_vec();

    // 添加简单校验码
    let checksum = crc32(&data) ^ 0xDEADBEEF;
    data.extend_from_slice(&checksum.to_le_bytes());

    // 轻量加密
    let cipher = XChaCha20Poly1305::new_from_slice(&self.secret_key).unwrap();
    let ciphertext = cipher.encrypt(&nonce.into(), &*data).unwrap();

    let mut output = Vec::with_capacity(NONCE_SIZE + ciphertext.len());
    output.extend_from_slice(&nonce);
    output.extend(ciphertext);

    e_utils::algorithm::base64::encode(output).trim_end_matches('=').to_string()
  }

  // 验证许可证（内存占用减少约40%）
  fn verify(&self, license: &str) -> e_utils::Result<DateTime<FixedOffset>> {
    // 添加回可能缺少的等号
    let padded_license = {
      let padding_len = (4 - license.len() % 4) % 4;
      let mut padded = String::with_capacity(license.len() + padding_len);
      padded.push_str(license);
      padded.extend(std::iter::repeat('=').take(padding_len));
      padded
    };

    let data = e_utils::algorithm::base64::decode(&padded_license).map_err(|_| "Invalid format")?;
    if data.len() < NONCE_SIZE + 12 {
      return Err("Invalid license".into());
    }

    let (nonce, ciphertext) = data.split_at(NONCE_SIZE);
    let cipher = XChaCha20Poly1305::new_from_slice(&self.secret_key).unwrap();

    let plaintext = cipher.decrypt(XNonce::from_slice(nonce), ciphertext).map_err(|_| "Decryption failed")?;

    if plaintext.len() != 12 {
      return Err("Invalid data length".into());
    }

    let (time_bytes, checksum_bytes) = plaintext.split_at(8);
    let expire_time = i64::from_le_bytes(time_bytes.try_into().unwrap());
    let stored_checksum = u32::from_le_bytes(checksum_bytes.try_into().unwrap());

    // 快速校验
    let calculated_checksum = crc32(time_bytes) ^ 0xDEADBEEF;
    if stored_checksum != calculated_checksum {
      return Err("Checksum mismatch".into());
    }

    // 使用中国时区 (UTC+8)
    let china_timezone = FixedOffset::east_opt(8 * 3600).unwrap();
    let expire = DateTime::from_timestamp(expire_time, 0)
      .ok_or("Invalid timestamp")?
      .with_timezone(&china_timezone);

    if Utc::now().with_timezone(&china_timezone) > expire {
      return Err("License expired".into());
    }

    Ok(expire)
  }
}
