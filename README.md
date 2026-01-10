[中文文档](README-zh-CN.md) | English

# btwo

A Rust library for interacting with the Backblaze B2 Cloud Storage API.

## Features

- **Account Authorization**: Authorize your Backblaze B2 account
- **File Upload**: Upload small files directly to B2 buckets
- **Large File Upload**: Upload large files in multiple parts
- **File Download**: Download files by name from buckets
- **File Management**: Hide and cancel files
- **Type Safety**: Strongly typed API responses and error handling

## Installation

Add this to your `Cargo.toml`:

```bash
cargo add btwo
```

## Documentation

For detailed documentation, please visit:

- [Examples](examples/) - View example code for common operations
- [API Documentation](https://docs.rs/btwo) - Full API reference

## Examples

See the `examples/` directory for complete working examples:

- `basic_upload.rs` - Upload a small file
- `large_file_upload.rs` - Upload a large file in parts
- `download_file.rs` - Download a file
- `hide_file.rs` - Hide a file
- `cancel_large_file.rs` - Cancel a large file upload

To run an example:

```bash
cargo run --example basic_upload
```

## Supported API Endpoints

- `b2_authorize_account` - Authorize account credentials
- `b2_get_upload_url` - Get URL for uploading files
- `b2_upload_file` - Upload a small file
- `b2_start_large_file` - Start a large file upload
- `b2_get_upload_part_url` - Get URL for uploading parts
- `b2_upload_part` - Upload part of a large file
- `b2_finish_large_file` - Finish a large file upload
- `b2_cancel_large_file` - Cancel a large file upload
- `b2_download_file_by_name` - Download a file by name
- `b2_hide_file` - Hide a file version

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Repository

https://github.com/Its-its/backblaze-btwo

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
