pub const API_URL_V2: &str = "https://api.backblazeb2.com/b2api/v2";

// Names can be pretty much any UTF-8 string up to 1024 bytes long. There are a few picky rules:
pub fn encode_file_name(file_name: &str) -> String {
    let mut file_name = file_name
        // Backslashes are not allowed.
        .replace('\\', "/")
        .replace("//", "--")
        .replace(' ', "%20")
        // DEL characters (127) are not allowed.
        .replace('\u{7F}', "-");

    // File names cannot start with "/", end with "/", or contain "//".
    if file_name.ends_with('/') {
        while file_name.ends_with('/') {
            file_name.pop();
        }
    }

    if file_name.starts_with('/') {
        while file_name.starts_with('/') {
            file_name.remove(0);
        }
    }

    // No character codes below 32 are allowed.
    for (index, cha) in file_name.clone().char_indices().rev() {
        if cha < '\u{20}' {
            file_name.remove(index);
        }
    }

    file_name
}
