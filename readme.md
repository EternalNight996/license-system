<div align="center">
  <img src="assets/icon.ico" alt="license-system" width="120"/>
  <h1>license-system</h1>
  <p><strong>Authorization System</strong></p>
</div>

## 🌐 Community Support

- 📮 Email Support: eternalnightyeah2@yeah.net
- 🐧 QQ Group: [984074316](https://qm.qq.com/q/984074316)
- 👥 WeChat Group: eternalnightwechat10

## 📈 Development Progress

| Module              | Status | Progress | Remarks                     |
|---------------------|--------|----------|-----------------------------|
| ChaCha20 Encryption | ✅     | 100%     | Integrated Libsodium        |
| AES-256-GCM Encryption | 🔄  | 100%     | Final testing in progress   |
| Offline Authorization | ✅    | 100%     | Hardware binding supported  |
| Online Verification | 🔄     | 30%      | Developing distributed auth |
| License Generation  | ✅     | 100%     | Supports RSA/ECC algorithms |

---

## Crates API
```toml
license-system = { version = "0.1", default-features = false, features = ["chacha20","aes256gcm"] }
```

## Example
```rust
use std::sync::Arc;
use license_system::{
    LicenseGenerator as _, LicenseManager, LicenseValidator as _, ChaCha20Protocol,
};

fn main() -> e_utils::Result<()> {
    // 创建授权管理器（使用固定密钥便于测试）
    let secret_key: [u8; 32] = [
        0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77,
        0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff, 0x00, 0x13, 0x24, 0x35, 0x46, 0x57, 0x68,
        0x79, 0x8a,
    ];
    let protocol = Arc::new(ChaCha20Protocol::new(secret_key));
    let license_manager = LicenseManager::new(protocol, 8)?;

    // Test users
    let user_ids = vec![
        "user1",
        "user2",
        "user3",
    ];

    println!("生成批量授权...");
    let licenses = license_manager.generate_batch_licenses(user_ids, 2)?;
    for license in &licenses {
        println!(
            "用户: {}, 授权码: {}, 过期时间: {}",
            license.user_id, license.license_key, license.expire_time
        );
    }

    // test
    println!("\n批量验证授权...");
    let verify_results = license_manager.verify_batch_licenses(&licenses)?;
    for result in &verify_results {
        println!("用户 {}: {}", result.user_id, result.message);
    }

    // test
    println!("\n验证单个授权...");
    if let Some(first_license) = licenses.first() {
        match license_manager.verify_license(&first_license.license_key) {
            Ok(res) => println!("验证结果: {} 剩余{}小时", res.message, res.days_remaining),
            Err(e) => println!("验证失败: {}", e),
        }
    }
    Ok(())
}
```

## 📜 License

[LICENSE](LICENSE)  
[COPYRIGHT](COPYRIGHT)

## 🤝 Contributing

We welcome all types of contributions!

- Submit Issues for bug reports or feature requests
- Create Pull Requests to improve code
- Enhance documentation
- Share usage experiences

Before submitting PRs, please ensure:
1. Code follows project standards
2. Necessary tests are added
3. Relevant documentation is updated

## 📜 Open Source Licenses

Dual-licensed under [MIT](LICENSE-MIT) and [Apache 2.0](LICENSE-APACHE).

---

<div align="center">
  <sub>Built with ❤️ by eternalnight996 and contributors.</sub>
</div>