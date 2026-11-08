mod common;
use common::assert_formula;

#[test]
fn plus_five_is_an_augmented_fifth_not_an_extra_perfect_fifth() {
    for symbol in ["C+5", "Caug", "C(#5)"] {
        assert_formula(symbol, &[(1, 0), (3, 0), (5, 1)]);
    }
    assert_formula("C+5(9)", &[(1, 0), (3, 0), (5, 1), (9, 0)]);
    assert_formula("C+5(omit3)", &[(1, 0), (5, 1)]);
}

#[test]
fn fifth_changes_and_b13_stay_different_degrees() {
    assert_formula("C7+5", &[(1, 0), (3, 0), (5, 1), (7, -1)]);
    assert_formula("C7(b13)", &[(1, 0), (3, 0), (5, 0), (7, -1), (13, -1)]);
    assert_formula("C5add9", &[(1, 0), (5, 0), (9, 0)]);
}
