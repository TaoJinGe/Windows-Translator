use super::openai_types::ChatStreamResponse;

pub fn parse_stream_event(event: &str) -> Result<Vec<String>, String> {
    let mut deltas = Vec::new();

    for line in event.lines() {
        let Some(data) = line.strip_prefix("data:") else {
            continue;
        };
        let data = data.trim();

        if data.is_empty() || data == "[DONE]" {
            continue;
        }

        let chunk = serde_json::from_str::<ChatStreamResponse>(data)
            .map_err(|_| "Translation stream format is invalid".to_string())?;

        for choice in chunk.choices {
            if let Some(content) = choice.delta.content {
                if !content.is_empty() {
                    deltas.push(content);
                }
            }
        }
    }

    Ok(deltas)
}

pub fn find_event_end(buffer: &[u8]) -> Option<usize> {
    buffer
        .windows(2)
        .position(|window| window == b"\n\n")
        .or_else(|| buffer.windows(4).position(|window| window == b"\r\n\r\n"))
}

pub fn trim_event_separator(buffer: &mut Vec<u8>) {
    if buffer.starts_with(b"\r\n\r\n") {
        buffer.drain(..4);
    } else if buffer.starts_with(b"\n\n") {
        buffer.drain(..2);
    }
}

#[cfg(test)]
mod tests {
    use super::parse_stream_event;

    #[test]
    fn parses_stream_delta() {
        let event = r#"data: {"choices":[{"delta":{"content":"Hi"}}]}"#;

        assert_eq!(parse_stream_event(event).unwrap(), vec!["Hi"]);
    }

    #[test]
    fn ignores_stream_done() {
        assert!(parse_stream_event("data: [DONE]").unwrap().is_empty());
    }
}
