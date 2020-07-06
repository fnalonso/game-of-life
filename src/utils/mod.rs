use std::fs;

pub fn load_layout_file(filename: String) -> Result<String, String> {
    let file_content = match fs::read_to_string(filename)  {
        Ok(data) => data,
        Err(_) => return Err("Could not open the layout file".to_string())
    };
    Ok(file_content)
}