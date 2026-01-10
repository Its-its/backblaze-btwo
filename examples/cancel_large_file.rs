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

    // Cancel a large file upload
    // Replace with your actual large file ID
    let file_id = "your_file_id".into();

    let response = endpoint::cancel_large_file(&file_id, &auth, &client).await?;

    println!("Large file cancelled successfully!");
    println!("File ID: {}", response.file_id);
    println!("File Name: {}", response.file_name);

    Ok(())
}
