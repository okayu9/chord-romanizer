mod common;
use common::assert_formula;

#[test]
fn suspension_preserves_the_fifth_from_the_base_quality() {
    assert_formula("Cm7b5sus2", &[(1, 0), (2, 0), (5, -1), (7, -1)]);
    assert_formula("Cdim(sus2)", &[(1, 0), (2, 0), (5, -1)]);
    assert_formula("C+sus2", &[(1, 0), (2, 0), (5, 1)]);
    assert_formula("Caug(sus4)", &[(1, 0), (4, 0), (5, 1)]);
}

#[test]
fn explicit_changes_and_omissions_survive_suspension() {
    assert_formula("C7sus2(b5)", &[(1, 0), (2, 0), (5, -1), (7, -1)]);
    assert_formula("Cm7b5sus2(omit5)", &[(1, 0), (2, 0), (7, -1)]);
    assert_formula("C7sus4", &[(1, 0), (4, 0), (5, 0), (7, -1)]);
}
