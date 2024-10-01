use std::process::Command;

#[cfg(target_os = "macos")]
pub fn sleep_is_prevented_by_apps() -> Result<Option<String>, anyhow::Error> {
    let output = Command::new("pmset")
        .arg("-g")
        .output()
        .map_err(|e| anyhow::anyhow!("Failed to execute pmset: {}", e))?;

    let res = extract_first_app(&String::from_utf8_lossy(&output.stdout));
    Ok(res) // No app is preventing sleep
}

pub fn extract_first_app(output_str: &str) -> Option<String> {
    if let Some(start_index) = output_str.find("display sleep prevented by") {
        // Extract the part after "display sleep prevented by"
        let rest_of_output = &output_str[start_index + "display sleep prevented by".len()..];

        // Find the closing parenthesis
        if let Some(end_index) = rest_of_output.find(')') {
            // Get the substring up to the closing parenthesis
            let apps_segment = &rest_of_output[..end_index];

            // Split the apps segment by commas and take the first element
            let first_app = apps_segment
                .split(',')
                .next()
                .map(|s| s.trim()) // Trim any whitespace
                .filter(|s| !s.is_empty()); // Ensure it's not empty

            return first_app.map(String::from);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_first_app_with_valid_input() {
        let input = "Some text before display sleep prevented by App1, App2, App3";
        let result = extract_first_app(input);
        assert_eq!(result, Some("App1".to_string()));
    }

    #[test]
    fn test_extract_first_with_single_input() {
        let input = "display sleep prevented by AppA";
        let result = extract_first_app(input);
        assert_eq!(result, Some("AppA".to_string()));
    }

    #[test]
    fn test_extract_first_app_with_no_apps() {
        let input = "display sleep prevented by ";
        let result = extract_first_app(input);
        assert_eq!(result, None);
    }

    #[test]
    fn test_extract_first_app_with_parenthesis() {
        let input = "(display sleep prevented by Steam Helper)\n\r tcpkeepalive         1\n\rlowpowermode         0";
        let result = extract_first_app(input);
        assert_eq!(result, Some("Steam Helper".to_string()));
    }
}
