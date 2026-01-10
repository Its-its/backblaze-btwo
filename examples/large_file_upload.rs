use btwo::{endpoint, Credentials};
use bytes::Bytes;
use reqwest::Client;
use std::num::NonZeroUsize;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize credentials
    let credentials = Credentials::new(
        "your_application_key_id",
        "your_application_key",
    );

    // Create HTTP client
    let client = Client::new();

    // Authorize account
    let auth = credentials.authorize(&client).await?;
    println!("Account authorized: {}", auth.account_id);

    // Start a large file
    let bucket_id = "your_bucket_id".into();
    let file_name = "large_file.bin";
    let content_type = "application/octet-stream";

    let file_info = endpoint::start_large_file(
        &bucket_id,
        file_name,
        content_type,
        &auth,
        &client,
    )
    .await?;

    println!("Large file started with ID: {}", file_info.file_id);

    // Simulate file parts (in real scenario, read from actual file)
    let parts: Vec<Bytes> = vec![
        Bytes::from(vec![0u8; 5 * 1024 * 1024]), // 5 MB part 1
        Bytes::from(vec![1u8; 5 * 1024 * 1024]), // 5 MB part 2
    ];

    let mut sha1_parts = Vec::new();

    for (i, part) in parts.iter().enumerate() {
        // Get upload part URL
        let upload_part_url = endpoint::get_upload_part_url(
            &file_info.file_id,
            &auth,
            &client,
        )
        .await?;

        // Upload part
        let part_num = NonZeroUsize::new(i + 1).unwrap();
        let part_response = endpoint::upload_part(
            part_num,
            part.clone(),
            &upload_part_url,
            &client,
        )
        .await?;

        sha1_parts.push(part_response.content_md5);
        println!("Uploaded part {}: {} bytes", part_num, part_response.content_length);
    }

    // Finish the large file
    let complete_file = endpoint::finish_large_file(
        &file_info.file_id,
        &sha1_parts,
        &auth,
        &client,
    )
    .await?;

    println!("Large file completed!");
    println!("File ID: {}", complete_file.file_id);
    println!("Total size: {} bytes", complete_file.content_length);

    Ok(())
}
