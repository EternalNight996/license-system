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