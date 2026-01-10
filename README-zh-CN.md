[English](README.md) | 中文文档

# btwo

一个用于与 Backblaze B2 云存储 API 交互的 Rust 库。

## 特性

- **账户授权**：授权您的 Backblaze B2 账户
- **文件上传**：直接上传小文件到 B2 存储桶
- **大文件上传**：分多个部分上传大文件
- **文件下载**：从存储桶下载文件
- **文件管理**：隐藏和取消文件
- **类型安全**：强类型的 API 响应和错误处理

## 安装

在您的 `Cargo.toml` 中添加：

```bash
cargo add btwo
```

## 文档

详细的文档请访问：

- [示例](examples/) - 查看常用操作的示例代码
- [API 文档](https://docs.rs/btwo) - 完整的 API 参考

## 示例

查看 `examples/` 目录以获取完整的工作示例：

- `basic_upload.rs` - 上传小文件
- `large_file_upload.rs` - 分块上传大文件
- `download_file.rs` - 下载文件
- `hide_file.rs` - 隐藏文件
- `cancel_large_file.rs` - 取消大文件上传

运行示例：

```bash
cargo run --example basic_upload
```

## 支持的 API 端点

- `b2_authorize_account` - 授权账户凭证
- `b2_get_upload_url` - 获取上传文件的 URL
- `b2_upload_file` - 上传小文件
- `b2_start_large_file` - 开始大文件上传
- `b2_get_upload_part_url` - 获取上传分块的 URL
- `b2_upload_part` - 上传大文件的一部分
- `b2_finish_large_file` - 完成大文件上传
- `b2_cancel_large_file` - 取消大文件上传
- `b2_download_file_by_name` - 按名称下载文件
- `b2_hide_file` - 隐藏文件版本

## 许可证

本项目采用 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件。

## 仓库

https://github.com/Its-its/backblaze-btwo

## 贡献

欢迎贡献！请随时提交 Pull Request。
