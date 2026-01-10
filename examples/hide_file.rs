use btwo::{Credentials, endpoint};
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize credentials
    let credentials = Credentials::new("your_application_key_id", "your_application_key");

    // Create HTTP client
    let client = Client::new();

    // Authorize account
    let auth = credentials.authorize(&client).await?;
    println!("Account authorized: {}", auth.account_id);

    // Hide a file (creates a new file version marking the file as hidden)
    let bucket_id = "your_bucket_id".into();
    let file_name = "example.txt";

    let response = endpoint::hide_file(&bucket_id, file_name, &auth, &client).await?;

    println!("File hidden successfully!");
    println!("{}", response);

    Ok(())
}
