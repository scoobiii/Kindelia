//! src/wallet/voice_ui.rs
//! Text-to-Speech and Speech-to-Text mock for offline usage.
//! Responsibilities:
//! - provide simple TTS playback string (mocked)
//! - provide STT parser for a small subset of commands (Portuguese + English)
//!
//! NOTE: This is a mock for local/offline testing. Replace with real TTS/STT libs for production.

use regex::Regex;

/// Simulate TTS by returning the text that would be spoken.
/// In real implementation this would call an offline TTS engine.
pub fn tts_speak(text: &str) -> String {
    // normalize: short friendly phrase
    format!("(TTS) {}", text)
}

/// Simple speech-to-intent parser.
/// Supported utterances:
/// - "enviar 10 para joao"
/// - "send 10 to alice"
pub enum Intent {
    Send { to: String, amount: u64 },
    Unknown,
}

pub fn stt_parse(utterance: &str) -> Intent {
    let lower = utterance.to_lowercase();
    // Portuguese pattern: enviar 10 para joao
    let re_pt = Regex::new(r"enviar\s+(\d+)\s+(?:para|p/)\s+([^\s]+)").unwrap();
    if let Some(caps) = re_pt.captures(&lower) {
        let amount = caps.get(1).unwrap().as_str().parse::<u64>().unwrap_or(0);
        let to = caps.get(2).unwrap().as_str().to_string();
        return Intent::Send { to, amount };
    }
    // English: send 10 to alice
    let re_en = Regex::new(r"send\s+(\d+)\s+to\s+([^\s]+)").unwrap();
    if let Some(caps) = re_en.captures(&lower) {
        let amount = caps.get(1).unwrap().as_str().parse::<u64>().unwrap_or(0);
        let to = caps.get(2).unwrap().as_str().to_string();
        return Intent::Send { to, amount };
    }
    Intent::Unknown
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tts_returns() {
        let out = tts_speak("Olá, avó!");
        assert!(out.contains("Olá"));
    }

    #[test]
    fn stt_pt_send() {
        match stt_parse("Enviar 12 para joao") {
            Intent::Send { to, amount } => {
                assert_eq!(to, "joao");
                assert_eq!(amount, 12);
            }
            _ => panic!("unexpected intent"),
        }
    }

    #[test]
    fn stt_en_send() {
        match stt_parse("Send 5 to bob") {
            Intent::Send { to, amount } => {
                assert_eq!(to, "bob");
                assert_eq!(amount, 5);
            }
            _ => panic!("unexpected intent"),
        }
    }
}
