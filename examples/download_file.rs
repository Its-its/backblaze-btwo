use btwo::{endpoint, Credentials};
use reqwest::Client;
use std::io::Write;

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

    // Download a file by name
    let bucket_name = "your_bucket_name";
    let file_path = "example.txt";

    let response = endpoint::download_file_by_name(
        bucket_name,
        file_path,
        auth,
        &client,
    )
    .await?;

    // Get file metadata from headers
    let content_length = response.content_length().unwrap_or(0);
    let content_type = response
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown");

    println!("Downloading file...");
    println!("Content-Type: {}", content_type);
    println!("Content-Length: {}", content_length);

    // Save to file
    let bytes = response.bytes().await?;
    let mut output_file = std::fs::File::create("downloaded.txt")?;
    output_file.write_all(&bytes)?;

    println!("File saved to 'downloaded.txt'");

    Ok(())
}
