use rand::{thread_rng, Rng};
use std::fs::File;
use std::io::Read;
use std::path::Path;

// compare hash with existing files to find if the file already exists
pub fn compare_if_has_exists(books_dir: &std::path::PathBuf, origin_hash: &String) -> Option<String> {
    // recursive scan books directory to find matching hash
    if let Ok(entries) = std::fs::read_dir(books_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();

                if path.is_dir() {
                    // Check if this directory contains a hash file
                    let hash_file_path = path.join("file_hash.txt");
                    if hash_file_path.exists() {
                        if let Ok(hash_content) = std::fs::read_to_string(&hash_file_path) {
                            // Compare the hash
                            if hash_content == *origin_hash {
                                // Found matching hash, now find the epub file in this directory
                                if let Ok(dir_entries) = std::fs::read_dir(&path) {
                                    for file_entry in dir_entries {
                                        if let Ok(file_entry) = file_entry {
                                            let file_path = file_entry.path();
                                            if file_path.extension().and_then(|e| e.to_str())
                                                == Some("epub")
                                            {
                                                // Found the epub file with matching hash
                                                return Some(
                                                    file_path.to_string_lossy().to_string(),
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    None
}

// Calculate MD5 hash of a file
pub fn calculate_md5_hash(file_path: &str) -> Result<String, String> {
    let path = Path::new(file_path);

    // Open the file
    let mut file =
        File::open(path).map_err(|e| format!("Failed to open file for hashing: {}", e))?;

    // Read file content
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .map_err(|e| format!("Failed to read file for hashing: {}", e))?;

    // Calculate MD5 hash
    let digest = md5::compute(&buffer);

    // Return the hash as a hexadecimal string
    Ok(format!("{:x}", digest))
}

// Generate a random string to use as file name
pub fn generate_random_string(length: usize) -> String {
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = thread_rng();

    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}
