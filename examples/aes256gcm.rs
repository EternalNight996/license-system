use std::sync::Arc;

use license_system::{
    LicenseGenerator as _, LicenseManager, LicenseValidator as _, AesGcmProtocol,
};

fn main() -> e_utils::Result<()> {
    // 创建授权管理器（使用固定密钥便于测试）
    let secret_key: [u8; 32] = [
        0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77,
        0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff, 0x00, 0x13, 0x24, 0x35, 0x46, 0x57, 0x68,
        0x79, 0x8a,
    ];
    let protocol = Arc::new(AesGcmProtocol::new(secret_key));
    let license_manager = LicenseManager::new(protocol, 8)?;

    // 测试批量生成
    let user_ids = vec![
        "user1".to_string(),
        "user2".to_string(),
        "user3".to_string(),
    ];

    println!("生成批量授权...");
    let licenses = license_manager.generate_batch_licenses(&user_ids, 300)?;
    for license in &licenses {
        println!(
            "用户: {}, 授权码: {}, 过期时间: {}",
            license.user_id, license.license_key, license.expire_time
        );
    }

    // 测试批量验证
    println!("\n批量验证授权...");
    let verify_results = license_manager.verify_batch_licenses(&licenses)?;
    for result in &verify_results {
        println!("用户 {}: {}", result.user_id, result.message);
    }

    // 测试单个授权验证
    println!("\n验证单个授权...");
    if let Some(first_license) = licenses.first() {
        match license_manager.verify_license(&first_license.license_key) {
            Ok(res) => println!("验证结果: {} 剩余{}天", res.message, res.days_remaining),
            Err(e) => println!("验证失败: {}", e),
        }
    }
    Ok(())
}
