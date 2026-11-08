mod common;
use chord_romanizer::structure::{formula_intervals, is_dominant_for};
use chord_romanizer::{BehaviorProfile, ChordInterpreter, SeventhQuality, SlashClassification};
use common::{assert_formula, chord};
use std::collections::HashSet;

#[test]
fn augmented_sevenths_use_the_minor_seventh() {
    for symbol in ["Caug7", "C+7", "Caug(7)", "C7(#5)"] {
        assert_formula(symbol, &[(1, 0), (3, 0), (5, 1), (7, -1)]);
        let parsed = chord(symbol);
        assert_eq!(parsed.quality_parsed.seventh, Some(SeventhQuality::Minor));
        assert!(is_dominant_for(&parsed, BehaviorProfile::StrictV1));
    }
}

#[test]
fn other_postfix_sevenths_keep_the_basic_quality() {
    for symbol in ["Csus47", "Csus4(7)"] {
        assert_formula(symbol, &[(1, 0), (4, 0), (5, 0), (7, -1)]);
    }
    assert_formula("Cm(7)", &[(1, 0), (3, -1), (5, 0), (7, -1)]);
    assert_formula("C(7)", &[(1, 0), (3, 0), (5, 0), (7, -1)]);
    assert_formula("Cadd9(7)", &[(1, 0), (3, 0), (5, 0), (7, -1), (9, 0)]);
    assert_formula("C(b9)7", &[(1, 0), (3, 0), (5, 0), (7, -1), (9, -1)]);
}

#[test]
fn explicit_major_and_diminished_sevenths_remain_distinct() {
    for symbol in ["Cmaj7(#5)", "CM7(#5)", "Cmaj7(#5,7)"] {
        assert_formula(symbol, &[(1, 0), (3, 0), (5, 1), (7, 0)]);
    }
    assert_formula("Cdim7", &[(1, 0), (3, -1), (5, -1), (7, -2)]);
    assert_formula("Cdim(7)", &[(1, 0), (3, -1), (5, -1), (7, -2)]);
    assert_formula("Cadd7", &[(1, 0), (3, 0), (5, 0), (7, 0)]);
}

#[test]
fn augmented_seventh_bass_is_a_real_inversion() {
    let parsed = chord("Caug7/Bb");
    let result =
        ChordInterpreter::new(BehaviorProfile::StrictV1).analyze_slash_chord(&parsed, None);
    assert_eq!(result.slash_classification, SlashClassification::Inversion);
}

#[test]
fn legacy_formula_and_original_lexeme_do_not_change() {
    let parsed = chord("Caug7/Bb");
    assert_eq!(parsed.quality, "aug7");
    assert_eq!(parsed.quality_parsed.raw, "aug7");
    assert_eq!(
        formula_intervals(&parsed, BehaviorProfile::Python019),
        Some(HashSet::from([0, 4, 7, 10]))
    );
}
