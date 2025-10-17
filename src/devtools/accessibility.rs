//! src/devtools/accessibility.rs
//! Accessibility SDK for DApps — declarative components and helpers.
//! Responsibilities:
//! - provide simple helpers for accessible labels, role hints and one-click templates.
//! - intended for DApp authors to import and use in their frontends.

pub struct AccessibleLabel {
    pub text: String,
    pub aria: String,
}

impl AccessibleLabel {
    pub fn new(text: &str) -> Self {
        AccessibleLabel {
            text: text.to_string(),
            aria: format!("label-{}", text.replace(" ", "-").to_lowercase()),
        }
    }
}

/// Simple template for one-click transaction button
pub struct OneClickTemplate {
    pub label: String,
    pub confirm_text: String,
}

impl OneClickTemplate {
    pub fn grandma() -> Self {
        OneClickTemplate {
            label: "Enviar (1-clique)".to_string(),
            confirm_text: "Confirma enviar? (um clique)".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn label_generation() {
        let l = AccessibleLabel::new("Enviar");
        assert!(l.aria.contains("enviar"));
    }

    #[test]
    fn grandma_template() {
        let t = OneClickTemplate::grandma();
        assert!(t.label.contains("Enviar"));
    }
}
