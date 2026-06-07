use regex::Regex;
use std::sync::LazyLock;

static IBAN_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b[A-Z]{2}[0-9]{2}(?:\s?[A-Z0-9]{4}){2,7}(?:\s?[A-Z0-9]{1,3})?\b").unwrap()
});

static EMAIL_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}").unwrap());

static CARD_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\b(?:\d{4}[-\s]?){3}\d{4}\b").unwrap());

static DATE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\b(?:\d{4}-\d{2}-\d{2}|\d{2}[/-]\d{2}[/-]\d{2,4})\b").unwrap());

static IP_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(?:(?:25[0-5]|2[0-4]\d|1?\d?\d)\.){3}(?:25[0-5]|2[0-4]\d|1?\d?\d)\b").unwrap()
});

static AMOUNT_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)(?:[$€£]\s?\d[\d.,]*|\d[\d.,]*\s?[$€£])").unwrap());

pub fn redact(text: &str) -> String {
    let mut redacted = text.to_string();

    redacted = IBAN_REGEX.replace_all(&redacted, "[REDACTED]").to_string();
    redacted = EMAIL_REGEX.replace_all(&redacted, "[REDACTED]").to_string();
    redacted = CARD_REGEX.replace_all(&redacted, "[REDACTED]").to_string();
    redacted = DATE_REGEX.replace_all(&redacted, "[REDACTED]").to_string();
    redacted = IP_REGEX.replace_all(&redacted, "[REDACTED]").to_string();
    redacted = AMOUNT_REGEX
        .replace_all(&redacted, "[REDACTED]")
        .to_string();

    redacted
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Copy, Clone)]
    enum PayloadShape {
        Plain,
        Multiline,
        Long,
        Malformed,
    }

    fn shaped_payload(shape: PayloadShape, token: &str) -> String {
        match shape {
            PayloadShape::Plain => format!("Synthetic payload with {token} and no other leak"),
            PayloadShape::Multiline => {
                format!("line 1: synthetic payload\nline 2: {token}\nline 3: still synthetic")
            }
            PayloadShape::Long => format!(
                "{} {token} {}",
                "prefix ".repeat(256),
                "suffix ".repeat(256)
            ),
            PayloadShape::Malformed => {
                format!("{{\"status\":\"broken\",\"raw\":{token},\"trail\":\"unterminated")
            }
        }
    }

    fn assert_redaction_properties(input: &str, needles: &[&str]) {
        let redacted = redact(input);

        for needle in needles {
            assert!(
                !redacted.contains(needle),
                "raw token leaked: {needle} in {redacted}"
            );
        }

        assert_eq!(redact(&redacted), redacted);
    }

    #[test]
    fn test_redact_iban() {
        let text = "Failed to process IBAN FR76 3000 6000 0112 3456 7890 123 for user";
        let redacted = redact(text);
        assert_eq!(redacted, "Failed to process IBAN [REDACTED] for user");
    }

    #[test]
    fn test_redact_email() {
        let text = "User john.doe@example.com not found";
        let redacted = redact(text);
        assert_eq!(redacted, "User [REDACTED] not found");
    }

    #[test]
    fn test_redact_amount() {
        let text = "Insufficient funds: tried to withdraw 150.00€ but balance is 10.50€";
        let redacted = redact(text);
        assert_eq!(
            redacted,
            "Insufficient funds: tried to withdraw [REDACTED] but balance is [REDACTED]"
        );
    }

    #[test]
    fn test_redact_card() {
        let text = "Card 4532 1234 5678 9012 declined";
        let redacted = redact(text);
        assert_eq!(redacted, "Card [REDACTED] declined");
    }

    #[test]
    fn test_redact_date_and_ip() {
        let text = "Rejected transfer on 2026-06-07 from 127.0.0.1";
        let redacted = redact(text);
        assert_eq!(redacted, "Rejected transfer on [REDACTED] from [REDACTED]");
    }

    #[test]
    fn test_redact_synthetic_corpus_properties() {
        let shapes = [
            PayloadShape::Plain,
            PayloadShape::Multiline,
            PayloadShape::Long,
            PayloadShape::Malformed,
        ];

        let cases: &[(&str, &[&str])] = &[
            (
                "IBAN FR76 3000 6000 0112 3456 7890 123",
                &["FR76 3000 6000 0112 3456 7890 123"],
            ),
            ("john.doe@example.com", &["john.doe@example.com"]),
            ("4532 1234 5678 9012", &["4532 1234 5678 9012"]),
            ("2026-06-07", &["2026-06-07"]),
            ("127.0.0.1", &["127.0.0.1"]),
            ("150.00€", &["150.00€"]),
            (
                "FR76 3000 6000 0112 3456 7890 123, john.doe@example.com, 4532 1234 5678 9012, 2026-06-07, 127.0.0.1, 150.00€",
                &[
                    "FR76 3000 6000 0112 3456 7890 123",
                    "john.doe@example.com",
                    "4532 1234 5678 9012",
                    "2026-06-07",
                    "127.0.0.1",
                    "150.00€",
                ],
            ),
        ];

        for shape in shapes {
            for (token, needles) in cases {
                let payload = shaped_payload(shape, token);
                assert_redaction_properties(&payload, needles);
            }
        }
    }
}
