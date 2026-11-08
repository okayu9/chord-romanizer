use chord_romanizer::structure::formula;
use chord_romanizer::{BehaviorProfile, ParsedChord, ParsedSymbol, parse_chord};
use std::collections::BTreeSet;

pub fn chord(symbol: &str) -> ParsedChord {
    match parse_chord(symbol).unwrap() {
        ParsedSymbol::Chord(chord) => chord,
        _ => panic!("expected chord: {symbol}"),
    }
}

pub fn assert_formula(symbol: &str, expected: &[(u8, i8)]) {
    let parsed = chord(symbol);
    let actual: BTreeSet<_> = formula(&parsed, BehaviorProfile::StrictV1)
        .unwrap_or_else(|| panic!("unknown formula: {symbol}"))
        .tones
        .iter()
        .map(|tone| (tone.degree.number(), tone.alteration))
        .collect();
    assert_eq!(
        actual,
        expected.iter().copied().collect(),
        "formula for {symbol}"
    );
}
