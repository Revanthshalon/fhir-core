use std::cmp::Ordering;

use super::*;

fn dec(s: &str) -> Decimal {
    Decimal::try_from(s).unwrap()
}

fn cd(s: &str) -> Code {
    Code::new(s).unwrap()
}

fn uri(s: &str) -> Uri {
    Uri::new(s).unwrap()
}

struct M {
    value: Option<Decimal>,
    comparator: Option<Code>,
    system: Option<Uri>,
    code: Option<Code>,
}

impl M {
    fn exact(v: &str) -> Self {
        Self {
            value: Some(dec(v)),
            comparator: None,
            system: None,
            code: None,
        }
    }

    fn with_comparator(v: &str, cmp: &str) -> Self {
        Self {
            value: Some(dec(v)),
            comparator: Some(cd(cmp)),
            system: None,
            code: None,
        }
    }

    fn with_unit(mut self, system: &str, code: &str) -> Self {
        self.system = Some(uri(system));
        self.code = Some(cd(code));
        self
    }

    fn magnitude(&self) -> QuantityMagnitude<'_> {
        QuantityMagnitude {
            value: self.value.as_ref(),
            comparator: self.comparator.as_ref(),
            system: self.system.as_ref(),
            code: self.code.as_ref(),
        }
    }
}

fn cmp(a: &M, b: &M) -> Option<Ordering> {
    quantity_partial_cmp(a.magnitude(), b.magnitude())
}

// --- Exact vs exact ---

#[test]
fn test_exact_less() {
    assert_eq!(cmp(&M::exact("3"), &M::exact("5")), Some(Ordering::Less));
}

#[test]
fn test_exact_greater() {
    assert_eq!(cmp(&M::exact("5"), &M::exact("3")), Some(Ordering::Greater));
}

#[test]
fn test_exact_equal() {
    assert_eq!(cmp(&M::exact("5"), &M::exact("5")), Some(Ordering::Equal));
}

#[test]
fn test_exact_numerically_equal_different_string_still_equal_here() {
    // The interval-algebra layer only sees f64; the caller (PartialOrd wrapper) is
    // responsible for squashing this to None when the full PartialEq disagrees (see
    // module docs) — this function alone is unaware of that distinction.
    assert_eq!(cmp(&M::exact("5.0"), &M::exact("5")), Some(Ordering::Equal));
}

// --- Missing value / unit mismatch ---

#[test]
fn test_missing_value_is_incomparable() {
    let no_value = M {
        value: None,
        comparator: None,
        system: None,
        code: None,
    };
    assert_eq!(cmp(&no_value, &M::exact("5")), None);
}

#[test]
fn test_different_units_incomparable() {
    let a = M::exact("5").with_unit("http://unitsofmeasure.org", "mg");
    let b = M::exact("5").with_unit("http://unitsofmeasure.org", "kg");
    assert_eq!(cmp(&a, &b), None);
}

#[test]
fn test_same_units_comparable() {
    let a = M::exact("3").with_unit("http://unitsofmeasure.org", "mg");
    let b = M::exact("5").with_unit("http://unitsofmeasure.org", "mg");
    assert_eq!(cmp(&a, &b), Some(Ordering::Less));
}

#[test]
fn test_one_side_unitless_incomparable() {
    let a = M::exact("5");
    let b = M::exact("5").with_unit("http://unitsofmeasure.org", "mg");
    assert_eq!(cmp(&a, &b), None);
}

// --- ad (approximately) / unrecognized comparator ---

#[test]
fn test_ad_comparator_incomparable() {
    assert_eq!(cmp(&M::with_comparator("5", "ad"), &M::exact("100")), None);
}

#[test]
fn test_unrecognized_comparator_incomparable() {
    assert_eq!(cmp(&M::with_comparator("5", "~"), &M::exact("100")), None);
}

// --- Exact vs one-sided range ---

#[test]
fn test_exact_vs_lt_definite_greater_when_at_or_above_bound() {
    // Exact 10 vs "<5": true value of the second is below 5, so 10 is definitely
    // greater no matter what the exact unknown value is.
    assert_eq!(
        cmp(&M::exact("10"), &M::with_comparator("5", "<")),
        Some(Ordering::Greater)
    );
    // Boundary: exact 5 vs "<5" — still definite, since "<5" never attains 5.
    assert_eq!(
        cmp(&M::exact("5"), &M::with_comparator("5", "<")),
        Some(Ordering::Greater)
    );
}

#[test]
fn test_exact_vs_lt_indeterminate_when_below_bound() {
    assert_eq!(cmp(&M::exact("3"), &M::with_comparator("5", "<")), None);
}

#[test]
fn test_exact_vs_le_boundary_indeterminate() {
    // Exact 5 vs "<=5": the other side could itself be exactly 5, so no definite order.
    assert_eq!(cmp(&M::exact("5"), &M::with_comparator("5", "<=")), None);
}

#[test]
fn test_exact_vs_gt_definite_less_when_at_or_below_bound() {
    assert_eq!(
        cmp(&M::exact("5"), &M::with_comparator("5", ">")),
        Some(Ordering::Less)
    );
    assert_eq!(
        cmp(&M::exact("3"), &M::with_comparator("5", ">")),
        Some(Ordering::Less)
    );
}

#[test]
fn test_exact_vs_ge_boundary_indeterminate() {
    assert_eq!(cmp(&M::exact("5"), &M::with_comparator("5", ">=")), None);
}

// --- Range vs range: same-direction pairs are always indeterminate ---

#[test]
fn test_lt_vs_lt_always_indeterminate() {
    assert_eq!(
        cmp(
            &M::with_comparator("3", "<"),
            &M::with_comparator("100", "<")
        ),
        None
    );
}

#[test]
fn test_gt_vs_gt_always_indeterminate() {
    assert_eq!(
        cmp(
            &M::with_comparator("100", ">"),
            &M::with_comparator("3", ">")
        ),
        None
    );
}

// --- Range vs range: opposite-direction pairs ---

#[test]
fn test_lt_vs_gt_overlapping_is_indeterminate() {
    // <10 (upper bound 10) vs >5 (lower bound 5): 10 > 5, so the ranges overlap on
    // (5, 10) — no definite order.
    assert_eq!(
        cmp(
            &M::with_comparator("10", "<"),
            &M::with_comparator("5", ">")
        ),
        None
    );
}

#[test]
fn test_lt_vs_gt_truly_non_overlapping_is_definite_less() {
    // <5 vs >10: everything below 5 is definitely less than everything above 10.
    assert_eq!(
        cmp(
            &M::with_comparator("5", "<"),
            &M::with_comparator("10", ">")
        ),
        Some(Ordering::Less)
    );
}

#[test]
fn test_lt_vs_gt_touching_open_boundary_is_definite_less() {
    // <5 vs >5: both open at the same point, so no shared value is possible.
    assert_eq!(
        cmp(&M::with_comparator("5", "<"), &M::with_comparator("5", ">")),
        Some(Ordering::Less)
    );
}

#[test]
fn test_gt_vs_lt_symmetric_case() {
    assert_eq!(
        cmp(
            &M::with_comparator("10", ">"),
            &M::with_comparator("5", "<")
        ),
        Some(Ordering::Greater)
    );
}

#[test]
fn test_le_vs_ge_touching_closed_boundary_is_indeterminate() {
    // <=5 vs >=5: both attain 5, so they could both equal 5 — no definite order.
    assert_eq!(
        cmp(
            &M::with_comparator("5", "<="),
            &M::with_comparator("5", ">=")
        ),
        None
    );
}

#[test]
fn test_le_vs_gt_touching_boundary_is_definite_less() {
    // <=5 vs >5: the first can attain 5, the second never does, so no shared value.
    assert_eq!(
        cmp(
            &M::with_comparator("5", "<="),
            &M::with_comparator("5", ">")
        ),
        Some(Ordering::Less)
    );
}

#[test]
fn test_ge_vs_le_overlapping_is_indeterminate() {
    // >=10 vs <=20: overlaps on [10, 20].
    assert_eq!(
        cmp(
            &M::with_comparator("10", ">="),
            &M::with_comparator("20", "<=")
        ),
        None
    );
}
