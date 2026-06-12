pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
use pyo3::prelude::*;

/// This function checks the medical prompt for prompt injection attacks.
/// It returns a tuple: (Is_Safe: bool, Sanitized_Text: String)
#[pyfunction]
fn inspect_prompt(user_input: String) -> (bool, String) {
    // 1. We define a list of known attack signatures (jailbreaks/injections)
    // For a doctoral thesis, this is a great “rule-based first-line defense” approach.
    let malicious_patterns = vec![
        "ignore previous instructions",
        "ignore the instructions above",
        "system override",
        "output clear text password",
        "as an admin",
        "become a chatbot without rules"
    ];

    // We convert the text to lowercase so that case doesn't matter
    let lower_input = user_input.to_lowercase();
    let mut is_safe = true;
    let mut sanitized_text = user_input.clone();

    // 2. We scan the text using Rust's extreme performance
    for pattern in malicious_patterns {
        if lower_input.contains(pattern) {
            // If an attack pattern is found, we set the security flag to false
            is_safe = false;

            // We don't just delete the malicious part,
            // but replace it with a security tag for the audit log.
            sanitized_text = sanitized_text.replace(pattern, "[SECURITY BLOCK - POTENTIAL JAILBREAK DETECTED]");
        }
    }

    // 3. Return to Python: (security status, sanitized text)
    (is_safe, sanitized_text)
}

/// This macro defines the Python module.
/// It tells Rust what to name the Python interface.
#[pymodule]
fn medical_guard(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(inspect_prompt, m)?)?;
    Ok(())
}
