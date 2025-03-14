<div align="center">
  <img src="assets/icon.ico" alt="license-system" width="120"/>
  <h1>license-system</h1>
  <p><strong>授权系统</strong></p>
</div>

## 🌐 社区支持

- 📮 邮箱支持：eternalnightyeah2@yeah.net
- 🐧 QQ群：[984074316](https://qm.qq.com/q/984074316)
- 👥 微信群：eternalnightwechat10


## 📈 功能开发进度

| 功能模块         | 状态  | 完成度 | 备注                 |
|------------------|-------|--------|----------------------|
| ChaCha20加密     | ✅    | 100%   | 集成Libsodium实现    |
| AES-256-GCM加密  | 🔄    | 100%    | 正在进行最终测试     |
| 离线授权机制     | ✅    | 100%   | 支持硬件绑定         |
| 线上授权验证     | 🔄    | 30%    | 分布式鉴权开发中     |
| 许可证生成       | ✅    | 100%   | 支持RSA/ECC算法      |

---

## 接口
```toml
license-system = { version = "0.1", default-features = false, features = ["chacha20","aes256gcm"] }
```

## 示例
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

    // 测试批量生成
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
            Ok(res) => println!("验证结果: {} 剩余{}小时", res.message, res.days_remaining),
            Err(e) => println!("验证失败: {}", e),
        }
    }
    Ok(())
}
```

## 📜 许可证

[LICENSE](LICENSE)
[COPYRIGHT](COPYRIGHT)

## 🤝 参与贡献

我们欢迎任何形式的贡献！

- 提交 Issue 报告 bug 或提出新功能建议
- 提交 Pull Request 改进代码
- 完善项目文档
- 分享使用经验

在提交 PR 之前，请确保：
1. 代码符合项目规范
2. 添加必要的测试
3. 更新相关文档

## 📜 开源协议

本项目采用 [MIT](LICENSE-MIT) 和 [Apache 2.0](LICENSE-APACHE) 双重协议。

---

<div align="center">
  <sub>Built with ❤️ by eternalnight996 and contributors.</sub>
</div>