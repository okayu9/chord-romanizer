mod common;
use common::{assert_formula, chord};
use chord_romanizer::{BehaviorProfile};
use chord_romanizer::structure::formula;

#[test]
fn omit_and_no_apply_to_a_comma_separated_parenthetical_list() {
    for symbol in ["C7(omit3,5)", "C7(no3,5)", "C7(omit3,omit5)"] {
        assert_formula(symbol, &[(1,0),(7,-1)]);
    }
    assert_formula("C7sus4(omit3,5)", &[(1,0),(4,0),(7,-1)]);
    assert_formula("C9(omit3,5,7)", &[(1,0),(9,0)]);
}

#[test]
fn omission_scope_stops_at_a_new_verb_or_parenthesis() {
    assert_formula("C7(omit3,add9,5)", &[(1,0),(5,0),(7,-1),(9,0)]);
    assert_formula("C7(omit3)(5)", &[(1,0),(5,0),(7,-1)]);
    assert_formula("C7(omit3,5)(add9)", &[(1,0),(7,-1),(9,0)]);
}

#[test]
fn unknown_text_still_prevents_a_formula() {
    for symbol in ["Cfoobar/E", "C7(omit3,banana)", "Caug9banana"] {
        assert!(formula(&chord(symbol), BehaviorProfile::StrictV1).is_none(), "{symbol}");
    }
}
