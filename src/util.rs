pub const API_URL_V2: &str = "https://api.backblazeb2.com/b2api/v2";


// Names can be pretty much any UTF-8 string up to 1024 bytes long. There are a few picky rules:
// No character codes below 32 are allowed.
// Backslashes are not allowed.
// DEL characters (127) are not allowed.
// File names cannot start with "/", end with "/", or contain "//".
pub fn encode_file_name(file_name: &str) -> String {
    let mut file_name = file_name
        .replace('\\', "/")
        .replace("//", "--")
        .replace(' ', "%20");

    if file_name.starts_with('/') {
        file_name.remove(0);
    }

    file_name
}