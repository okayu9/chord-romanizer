//! Chord-symbol parser.
//!
//! Parsing is intentionally lossless at the notation boundary: root, quality,
//! slash bass, and original lexemes are retained. Semantic validation of the
//! quality happens later, allowing unknown suffixes to round-trip without
//! pretending they describe a known major chord.

use crate::domain::{ChordQuality, ParsedChord, ParsedSymbol, SpelledNote};
use crate::error::ParseError;

/// Parse a chord, N.C. marker, or other supported progression symbol.
///
/// The standalone parser does not create boundaries; callers add those with
/// `ProgressionItem::boundary` because a long silence cannot be inferred from
/// the text `N.C.` alone.
pub fn parse_chord(symbol: &str) -> Result<ParsedSymbol, ParseError> {
    let text = symbol.trim();
    if text.is_empty() {
        return Err(ParseError::EmptyInput);
    }

    // Normalize only for recognition. Keep the trimmed original spelling in
    // the AST so an aligned editor can reproduce the user's input.
    let normalized_nc = text.replace(['.', ' '], "").to_ascii_uppercase();
    if matches!(normalized_nc.as_str(), "NC" | "NOCHORD") {
        return Ok(ParsedSymbol::NoChord {
            original_symbol: text.to_owned(),
        });
    }

    let (body, bass_token) = split_bass(text)?;

    let body = body.trim();
    if body.is_empty() {
        return Err(ParseError::InvalidRoot(body.to_owned()));
    }

    // A root token is one note letter followed by contiguous accidentals.
    // Quality parsing starts at the first non-accidental character.
    let mut root_end = body
        .char_indices()
        .next()
        .map(|(index, ch)| index + ch.len_utf8())
        .ok_or_else(|| ParseError::InvalidRoot(body.to_owned()))?;
    for (index, ch) in body[root_end..].char_indices() {
        if matches!(ch, '#' | 'b' | 'B' | 'x' | 'X') {
            root_end += ch.len_utf8();
        } else {
            let _ = index;
            break;
        }
    }

    let root_token = &body[..root_end];
    let root = SpelledNote::parse(root_token)
        .map_err(|_| ParseError::InvalidRoot(root_token.to_owned()))?;
    let root_lexeme = normalize_spelling(root_token);
    // Keep raw quality for exact rendering and parse a second structured view
    // for theory. Neither representation has to be reconstructed from the
    // other later.
    let quality = body[root_end..].to_owned();
    let quality_parsed = ChordQuality::parse(&quality);
    let (bass, bass_lexeme) = bass_token
        .map(|(raw, note)| (Some(note), Some(normalize_spelling(raw))))
        .unwrap_or((None, None));

    Ok(ParsedSymbol::Chord(ParsedChord {
        original_symbol: text.to_owned(),
        root,
        root_lexeme,
        quality,
        quality_parsed,
        bass,
        bass_lexeme,
    }))
}

fn split_bass(text: &str) -> Result<(&str, Option<(&str, SpelledNote)>), ParseError> {
    // A terminal (/E) or (onE) is a bass annotation, not a quality modifier.
    // Require a complete note and no other structural bass separator.
    if let Some(without_close) = text.strip_suffix(')') {
        if let Some((body, group)) = without_close.rsplit_once('(') {
            if let Some(bass) = group.strip_prefix('/').or_else(|| group.strip_prefix("on")) {
                if structural_slash(body).is_none() {
                    let bass = bass.trim();
                    let parsed = SpelledNote::parse(bass)
                        .map_err(|_| ParseError::InvalidBass(bass.to_owned()))?;
                    return Ok((body, Some((bass, parsed))));
                }
            }
        }
    }
    // Skip only a recognized basic 6/9. Any further slash remains in the
    // bass token and fails note parsing instead of being silently discarded.
    if let Some(position) = structural_slash(text) {
        let bass = text[position + 1..].trim();
        let parsed =
            SpelledNote::parse(bass).map_err(|_| ParseError::InvalidBass(bass.to_owned()))?;
        Ok((&text[..position], Some((bass, parsed))))
    } else {
        Ok((text, None))
    }
}

fn structural_slash(text: &str) -> Option<usize> {
    text.match_indices('/')
        .map(|(position, _)| position)
        .find(|position| !is_six_nine_separator(text, *position))
}

fn is_six_nine_separator(text: &str, position: usize) -> bool {
    let prefix = &text[..position];
    if !prefix
        .chars()
        .next()
        .is_some_and(|ch| ch.is_ascii_alphabetic())
    {
        return false;
    }
    let Some(after_nine) = text[position + 1..].strip_prefix('9') else {
        return false;
    };
    if !(after_nine.is_empty() || after_nine.starts_with(['(', ',', '/'])) {
        return false;
    }
    let root_end = prefix
        .char_indices()
        .skip(1)
        .take_while(|(_, ch)| matches!(ch, '#' | 'b' | 'B' | 'x' | 'X'))
        .last()
        .map_or(1, |(index, ch)| index + ch.len_utf8());
    if root_end > prefix.len() {
        return false;
    }
    let quality = prefix[root_end..].replace(' ', "");
    matches!(
        quality.as_str(),
        "6" | "m6" | "min6" | "M6" | "maj6" | "Maj6" | "MAJ6"
    )
}

fn normalize_spelling(token: &str) -> String {
    let mut chars = token.chars();
    match chars.next() {
        Some(first) => format!("{}{}", first.to_ascii_uppercase(), chars.as_str()),
        None => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn chord(symbol: &str) -> ParsedChord {
        match parse_chord(symbol).unwrap() {
            ParsedSymbol::Chord(chord) => chord,
            ParsedSymbol::NoChord { .. } | ParsedSymbol::Boundary { .. } => {
                panic!("expected a chord")
            }
        }
    }

    #[test]
    fn parses_root_quality_and_bass() {
        let parsed = chord("C#m7/G#");
        assert_eq!(parsed.root.to_string(), "C#");
        assert_eq!(parsed.quality, "m7");
        assert_eq!(parsed.bass.unwrap().to_string(), "G#");
    }

    #[test]
    fn preserves_double_accidental_lexemes() {
        assert_eq!(chord("Bbb7").root_lexeme, "Bbb");
        assert_eq!(chord("F##m7").root_lexeme, "F##");
        assert_eq!(chord("C7/Bbb").bass_lexeme.as_deref(), Some("Bbb"));
    }

    #[test]
    fn recognizes_no_chord() {
        assert!(matches!(
            parse_chord("N.C.").unwrap(),
            ParsedSymbol::NoChord { .. }
        ));
    }

    #[test]
    fn rejects_invalid_bass() {
        assert!(matches!(
            parse_chord("C/G/Db"),
            Err(ParseError::InvalidBass(_))
        ));
    }
}
