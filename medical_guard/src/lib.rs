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

/// Diese Funktion prüft den medizinischen Prompt auf Prompt-Injection-Angriffe.
/// Sie gibt ein Tupel zurück: (Ist_Sicher: bool, Sanitisierter_Text: String)
#[pyfunction]
fn inspect_prompt(user_input: String) -> (bool, String) {
    // 1. Wir definieren eine Liste von bekannten Angriffs-Signaturen (Jailbreaks/Injections)
    // Für eine Doktorarbeit ist das ein super "regelbasierter First-Line-Defense"-Ansatz.
    let malicious_patterns = vec![
        "ignore previous instructions",
        "ignore the instructions above",
        "system override",
        "output clear text password",
        "as an admin",
        "become a chatbot without rules"
    ];

    // Wir konvertieren den Text in Kleinbuchstaben, damit Groß-/Kleinschreibung keine Rolle spielt
    let lower_input = user_input.to_lowercase();
    let mut is_safe = true;
    let mut sanitized_text = user_input.clone();

    // 2. Wir scannen den Text mit der extremen Performance von Rust ab
    for pattern in malicious_patterns {
        if lower_input.contains(pattern) {
            // Wenn ein Angriffsmuster gefunden wird, setzen wir das Sicherheits-Flag auf false
            is_safe = false;

            // Akademischer Ansatz: Wir löschen den schädlichen Teil nicht nur,
            // sondern ersetzen ihn durch ein Sicherheits-Tag für das Audit-Log.
            sanitized_text = sanitized_text.replace(pattern, "[SECURITY BLOCK - POTENTIAL JAILBREAK DETECTED]");
        }
    }

    // 3. Rückgabe an Python: (Sicherheitsstatus, bereinigter Text)
    (is_safe, sanitized_text)
}

/// Dieses Makro definiert das Python-Modul.
/// Es sagt Rust, wie die Python-Schnittstelle heißen soll.
#[pymodule]
fn medical_guard(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(inspect_prompt, m)?)?;
    Ok(())
}
