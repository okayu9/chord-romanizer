mod common;
use common::assert_formula;

#[test]
fn unparenthesized_ninth_bases_include_the_seventh() {
    assert_formula("Caug9", &[(1, 0), (3, 0), (5, 1), (7, -1), (9, 0)]);
    assert_formula(
        "Caug9(#11)",
        &[(1, 0), (3, 0), (5, 1), (7, -1), (9, 0), (11, 1)],
    );
    assert_formula("Csus49", &[(1, 0), (4, 0), (5, 0), (7, -1), (9, 0)]);
}

#[test]
fn added_ninths_and_parenthetical_tensions_do_not_invent_lower_members() {
    assert_formula("Caug(9)", &[(1, 0), (3, 0), (5, 1), (9, 0)]);
    assert_formula("Csus4(9)", &[(1, 0), (4, 0), (5, 0), (9, 0)]);
    assert_formula("Cm7(11)", &[(1, 0), (3, -1), (5, 0), (7, -1), (11, 0)]);
    assert_formula("C7(13)", &[(1, 0), (3, 0), (5, 0), (7, -1), (13, 0)]);
    assert_formula("C6(9)", &[(1, 0), (3, 0), (5, 0), (6, 0), (9, 0)]);
}
