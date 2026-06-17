pub fn strip_thinking(value: &str) -> String {
    let mut remaining = value.trim().to_string();

    loop {
        let lower = remaining.to_lowercase();
        let Some(start) = lower.find("<think>") else {
            break;
        };
        let Some(end_offset) = lower[start..].rfind("</think>") else {
            remaining.replace_range(start.., "");
            break;
        };
        let end = start + end_offset;

        remaining.replace_range(start..end + "</think>".len(), "");
    }

    let lower = remaining.to_lowercase();
    if let Some(end) = lower.rfind("</think>") {
        remaining.replace_range(..end + "</think>".len(), "");
    }

    remaining.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::strip_thinking;

    #[test]
    fn removes_thinking_that_quotes_closing_tag() {
        let value = "<think>
Do not output <think>, </think> or any chain-of-thought content.
Wait, I need to reconsider.
</think>

AI Translator";

        assert_eq!(strip_thinking(value), "AI Translator");
    }

    #[test]
    fn removes_unclosed_thinking() {
        let value = "<think>
I should not show this.";

        assert_eq!(strip_thinking(value), "");
    }

    #[test]
    fn removes_stray_closing_tag_prefix() {
        let value = "Hidden reasoning leaked.
</think>
Final translation";

        assert_eq!(strip_thinking(value), "Final translation");
    }
}
