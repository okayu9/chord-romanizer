mod common;

use chord_romanizer::structure::formula;
use chord_romanizer::{BehaviorProfile, ParsedSymbol, SeventhQuality, parse_chord};
use common::{assert_formula, chord};
use std::collections::BTreeSet;

#[test]
fn plus_tensions_change_the_written_degree_not_the_triad() {
    for symbol in ["C7+9", "C7(+9)", "C7add+9"] {
        assert_formula(symbol, &[(1, 0), (3, 0), (5, 0), (7, -1), (9, 1)]);
    }
    assert_formula(
        "CM9(+11)",
        &[(1, 0), (3, 0), (5, 0), (7, 0), (9, 0), (11, 1)],
    );
    assert_formula("C7(+13)", &[(1, 0), (3, 0), (5, 0), (7, -1), (13, 1)]);
    assert_formula("C+7", &[(1, 0), (3, 0), (5, 1), (7, -1)]);
    assert_formula("C7(b13)", &[(1, 0), (3, 0), (5, 0), (7, -1), (13, -1)]);
    assert_formula("CMb13", &[(1, 0), (3, 0), (5, 0), (13, -1)]);
    assert_formula("CM#11", &[(1, 0), (3, 0), (5, 0), (11, 1)]);
    assert_eq!(chord("CMb13").root.to_string(), "C");
}

#[test]
fn major_aliases_preserve_seventh_and_added_tension_distinctions() {
    for symbol in ["CMaj7", "CMAJ7", "C△7", "CΔ7", "Cmajor7"] {
        assert_formula(symbol, &[(1, 0), (3, 0), (5, 0), (7, 0)]);
    }
    assert_formula("CM", &[(1, 0), (3, 0), (5, 0)]);
    assert_formula("CM(11)", &[(1, 0), (3, 0), (5, 0), (11, 0)]);
    for symbol in ["CM6", "Cmaj6", "CMaj6"] {
        assert_formula(symbol, &[(1, 0), (3, 0), (5, 0), (6, 0)]);
    }
    for symbol in ["CmM9", "Cmmaj9", "CmMaj9"] {
        assert_formula(symbol, &[(1, 0), (3, -1), (5, 0), (7, 0), (9, 0)]);
    }
}

#[test]
fn explicit_major_seventh_is_independent_of_the_base() {
    assert_formula("CaugM7", &[(1, 0), (3, 0), (5, 1), (7, 0)]);
    assert_formula("CdimM7", &[(1, 0), (3, -1), (5, -1), (7, 0)]);
    assert_formula("C(M7)", &[(1, 0), (3, 0), (5, 0), (7, 0)]);
    assert_formula("Cm(M7)", &[(1, 0), (3, -1), (5, 0), (7, 0)]);
    assert_formula("C5(M7)", &[(1, 0), (5, 0), (7, 0)]);
    assert_formula("Caug(M9)", &[(1, 0), (3, 0), (5, 1), (7, 0), (9, 0)]);
    assert_eq!(
        chord("CaugM7").quality_parsed.seventh,
        Some(SeventhQuality::Major)
    );
}

#[test]
fn omission_prefixes_are_not_diminished_chords() {
    assert_formula("Comit3", &[(1, 0), (5, 0)]);
    assert_formula("Comit5", &[(1, 0), (3, 0)]);
    assert_formula("C7no3rd", &[(1, 0), (5, 0), (7, -1)]);
    assert_formula("C7(omitroot)", &[(3, 0), (5, 0), (7, -1)]);
    assert_formula("Cdim7", &[(1, 0), (3, -1), (5, -1), (7, -2)]);
    assert_formula("Co7", &[(1, 0), (3, -1), (5, -1), (7, -2)]);
}

#[test]
fn a_grouped_addition_remains_added_rather_than_altered() {
    assert_formula("Cmadd(b13)", &[(1, 0), (3, -1), (5, 0), (13, -1)]);
    assert_formula("Cadd(b5)", &[(1, 0), (3, 0), (5, 0), (5, -1)]);
    assert_formula("C(b5)", &[(1, 0), (3, 0), (5, -1)]);
}

#[test]
fn suspension_and_augmented_aliases_keep_the_other_degrees() {
    for symbol in ["Csus", "CSus4"] {
        assert_formula(symbol, &[(1, 0), (4, 0), (5, 0)]);
    }
    for symbol in ["Csus7", "C7sus"] {
        assert_formula(symbol, &[(1, 0), (4, 0), (5, 0), (7, -1)]);
    }
    assert_formula("C7aug", &[(1, 0), (3, 0), (5, 1), (7, -1)]);
    assert_formula("CM7aug", &[(1, 0), (3, 0), (5, 1), (7, 0)]);
    assert_formula("Cø", &[(1, 0), (3, -1), (5, -1), (7, -1)]);
    assert_formula("Cøsus2", &[(1, 0), (2, 0), (5, -1), (7, -1)]);
}

#[test]
fn six_nine_does_not_consume_a_slash_bass() {
    for symbol in ["C6/9", "CMaj6/9", "C6/9/G"] {
        assert_formula(symbol, &[(1, 0), (3, 0), (5, 0), (6, 0), (9, 0)]);
    }
    assert_formula("Cm6/9", &[(1, 0), (3, -1), (5, 0), (6, 0), (9, 0)]);
    let parsed = chord("C6/9/G");
    assert_eq!(parsed.quality, "6/9");
    assert_eq!(parsed.bass.unwrap().to_string(), "G");
    assert_eq!(parsed.original_symbol, "C6/9/G");
}

#[test]
fn parenthetical_bass_is_separate_from_the_quality() {
    for symbol in ["C7(/E)", "C7(onE)"] {
        let parsed = chord(symbol);
        assert_eq!(parsed.quality, "7");
        assert_eq!(parsed.bass.unwrap().to_string(), "E");
        assert_eq!(parsed.original_symbol, symbol);
        assert_formula(symbol, &[(1, 0), (3, 0), (5, 0), (7, -1)]);
    }
    assert_formula("C7(b9)(/E)", &[(1, 0), (3, 0), (5, 0), (7, -1), (9, -1)]);
}

#[test]
fn original_quality_lexemes_are_preserved() {
    for quality in ["Maj7", "△7", "7+9", "mM9", "omit3", "madd(b13)"] {
        let parsed = chord(&format!("C{quality}"));
        assert_eq!(parsed.quality, quality);
        assert_eq!(parsed.quality_parsed.raw, quality);
    }
}

#[test]
fn uncertain_or_unknown_text_is_not_removed() {
    for symbol in [
        "C>",
        "C----",
        "C-",
        "C7/9",
        "Csus9",
        "C7alt",
        "C△",
        "C7(M7)",
        "Cdim7(M7)",
        "C7(+garbage)",
        "C7(+#9)",
        "C7(/E)/G",
        "Cmaug",
        "Cm7sus4(aug)",
        "Bomit3,12",
        "C7(add12)",
        "G7(12)",
        "Ｃ6/9",
    ] {
        if let Ok(ParsedSymbol::Chord(parsed)) = parse_chord(symbol) {
            assert!(
                formula(&parsed, BehaviorProfile::StrictV1).is_none(),
                "{symbol}"
            );
        }
    }
}

#[test]
fn reviewed_aliases_match_the_frozen_degree_and_bass_expectations() {
    let cases = include_str!("fixtures/strict_notation_aliases.tsv");
    let mut count = 0;
    for line in cases
        .lines()
        .filter(|line| !line.starts_with('#') && !line.is_empty())
    {
        let fields: Vec<_> = line.split('\t').collect();
        assert_eq!(fields.len(), 4);
        let bytes: Vec<_> = (0..fields[0].len())
            .step_by(2)
            .map(|index| u8::from_str_radix(&fields[0][index..index + 2], 16).unwrap())
            .collect();
        let symbol = String::from_utf8(bytes).unwrap();
        let expected: BTreeSet<_> = fields[1]
            .split(',')
            .map(|pair| {
                let (degree, alteration) = pair.split_once(':').unwrap();
                (
                    degree.parse::<u8>().unwrap(),
                    alteration.parse::<i8>().unwrap(),
                )
            })
            .collect();
        let parsed = chord(&symbol);
        assert_formula(&symbol, &expected.iter().copied().collect::<Vec<_>>());
        assert_eq!(parsed.root.to_string(), fields[2], "root for {symbol}");
        assert_eq!(
            parsed.bass.map(|bass| bass.to_string()).as_deref(),
            (fields[3] != "-").then_some(fields[3]),
            "bass for {symbol}"
        );
        count += 1;
    }
    assert_eq!(count, 797);
}
