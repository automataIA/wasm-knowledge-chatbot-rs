/// Simple placeholder summarizer for communities and results.
pub struct Summarizer;

impl Summarizer {
    pub fn new() -> Self { Self }

    /// Summarize a block of text with a max length cap. Stub: truncate and append ellipsis.
    pub fn summarize(&self, text: &str, max_len: usize) -> String {
        if text.len() <= max_len { return text.to_string(); }
        let mut s = text[..max_len.min(text.len())].to_string();
        s.push('…');
        s
    }
}

impl Default for Summarizer {
    fn default() -> Self { Self::new() }
}
