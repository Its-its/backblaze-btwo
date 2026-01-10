use btwo::{endpoint, Credentials};
use reqwest::Client;

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

    // Get upload URL for a bucket
    // Replace with your actual bucket ID
    let bucket_id = "your_bucket_id".into();
    let upload_url = endpoint::get_upload_url(&bucket_id, &auth, &client).await?;
    println!("Upload URL obtained");

    // Upload a small file
    let file_name = "example.txt";
    let content_type = "text/plain";
    let contents = b"Hello, Backblaze B2!".to_vec();

    let response = endpoint::upload_file(
        file_name,
        content_type,
        contents,
        &upload_url,
        &client,
    )
    .await?;

    println!("File uploaded successfully!");
    println!("File ID: {}", response.file_id);
    println!("File Name: {}", response.file_name);

    Ok(())
}
