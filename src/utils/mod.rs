// Utility helpers shared across the project

use crate::contexts::paragraph_context::Paragraph;

/// Find a paragraph in the provided list by its id.
///
/// This is a small helper that centralises the lookup logic so that multiple
/// call-sites (UI refresh code, tests, etc.) rely on the same behaviour.
pub fn find_paragraph_by_id<'a>(paragraphs: &'a [Paragraph], id: &str) -> Option<Paragraph> {
    paragraphs.iter().find(|p| p.id == id).cloned()
} 