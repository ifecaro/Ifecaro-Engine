// Utility helpers shared across the project

use crate::contexts::paragraph_context::Paragraph;

pub mod theme;

/// Find a paragraph in the provided list by its id.
///
/// This is a small helper that centralises the lookup logic so that multiple
/// call-sites (UI refresh code, tests, etc.) rely on the same behaviour.
pub fn find_paragraph_by_id<'a>(paragraphs: &'a [Paragraph], id: &str) -> Option<Paragraph> {
    paragraphs.iter().find(|p| p.id == id).cloned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contexts::paragraph_context::{ParagraphChoice, Text};

    fn paragraph(id: &str) -> Paragraph {
        Paragraph {
            id: id.to_string(),
            chapter_id: "chapter-1".to_string(),
            texts: vec![Text {
                lang: "en".to_string(),
                paragraphs: "Sample".to_string(),
                choices: vec![],
            }],
            choices: vec![ParagraphChoice::Simple(vec!["next".to_string()])],
        }
    }

    #[test]
    fn find_paragraph_by_id_returns_match() {
        let paragraphs = vec![paragraph("p-1"), paragraph("p-2")];

        let result = find_paragraph_by_id(&paragraphs, "p-2");

        assert!(result.is_some());
        assert_eq!(result.unwrap().id, "p-2");
    }

    #[test]
    fn find_paragraph_by_id_returns_none_for_missing_id() {
        let paragraphs = vec![paragraph("p-1"), paragraph("p-2")];

        let result = find_paragraph_by_id(&paragraphs, "p-3");

        assert!(result.is_none());
    }
}
