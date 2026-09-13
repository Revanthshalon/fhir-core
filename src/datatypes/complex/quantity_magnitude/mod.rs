//! Shared comparator-aware, unit-aware partial-order logic for the `Quantity` family
//! (`Quantity` itself plus its six profiles: `Age`, `Count`, `Distance`, `Duration`,
//! `MoneyQuantity`, `SimpleQuantity`).
//!
//! # Why this exists
//! FHIR's `comparator` (`<`, `<=`, `>=`, `>`, `ad`) turns a quantity's `value` from a
//! single point into an open-ended range: `<5mg` means "the true value is somewhere
//! below 5mg", not "the value is 5mg". Two such ranges are only sometimes orderable —
//! `<10` and `>20` are definitely ordered (everything below 10 is less than everything
//! above 20), but `<10` and `>5` overlap on `(5, 10)` and have no definite order. This
//! makes quantity comparison a genuine **partial** order (`PartialOrd`, returning
//! `Option<Ordering>`), never a total one (`Ord`) — there is no correct `Ord` impl for
//! this type, the same way there is no correct `Ord` for `f64` because of `NaN`.
//!
//! `None` is returned, not guessed, whenever:
//! - the two values carry different `system`/`code` (no UCUM unit conversion is
//!   performed by this crate — see `docs/REVIEW_ISSUES.md` ISSUE-009 — so quantities in
//!   different units are simply incomparable here),
//! - either side is missing a numeric `value`,
//! - either side's comparator is `ad` ("approximately") or any code outside `<`, `<=`,
//!   `>=`, `>` — `ad` has no defined bound to reason about (matching this crate's
//!   general policy of validating a bound `code`'s grammar only, not its `ValueSet`
//!   membership; an unrecognized comparator is treated the same way: unknown, so no
//!   order is claimed),
//! - the two ranges genuinely overlap (e.g. `<10` vs `>5`).
//!
//! # `PartialOrd`/`PartialEq` consistency
//! Every caller wraps this function as:
//! ```text
//! if self == other { return Some(Equal); }
//! quantity_partial_cmp(...)  // never itself returns Some(Equal); see below
//! ```
//! so `Some(Equal)` is returned if and only if the full derived `PartialEq` (which also
//! considers `id`/`extension`/`unit` display text) agrees — the same law `Decimal`
//! itself is built around (see `decimal/mod.rs`). This function's own exact/exact
//! branch below can observe `av == bv` for two structurally-unequal values (e.g.
//! differing `id`, or `Decimal` strings `"5.0"` vs `"5"` that are numerically but not
//! structurally equal); in that case it still returns `Some(Equal)`, and the caller's
//! `self == other` guard has already run first, so a genuine mismatch here would be a
//! caller wiring bug, not a contract violation — callers must always check `self ==
//! other` before delegating here.

use std::cmp::Ordering;

use crate::types::{Code, Decimal, Uri};

/// The fields that determine a `Quantity`-family value's magnitude for
/// comparator-aware ordering: the numeric value, its open/closed comparator, and its
/// unit identity (`system`+`code`). Deliberately crate-private and comparison-only —
/// it does not participate in any type's public shape (see `docs/REVIEW_ISSUES.md`
/// ISSUE-002 for why the 7 `Quantity`-family types don't share a public struct).
pub(crate) struct QuantityMagnitude<'a> {
    pub value: Option<&'a Decimal>,
    pub comparator: Option<&'a Code>,
    pub system: Option<&'a Uri>,
    pub code: Option<&'a Code>,
}

/// One side of a value range: `None` means unbounded in that direction; `Some((x,
/// true))` means the bound is attained (closed, from `<=`/`>=`); `Some((x, false))`
/// means it isn't (open, from `<`/`>`).
type Bound = Option<(f64, bool)>;

/// Comparator-aware, unit-aware partial order between two `Quantity`-family
/// magnitudes. See the module docs for exactly when this returns `None`.
pub(crate) fn quantity_partial_cmp(a: QuantityMagnitude, b: QuantityMagnitude) -> Option<Ordering> {
    fn unit_key<'a>(m: &QuantityMagnitude<'a>) -> (Option<&'a str>, Option<&'a str>) {
        (m.system.map(Uri::as_str), m.code.map(Code::as_str))
    }
    if unit_key(&a) != unit_key(&b) {
        return None;
    }

    let av = a.value?.as_f64();
    let bv = b.value?.as_f64();

    // Exact/exact is the common case and needs none of the range machinery below.
    if a.comparator.is_none() && b.comparator.is_none() {
        return av.partial_cmp(&bv);
    }

    let (a_lower, a_upper) = bounds(av, a.comparator)?;
    let (b_lower, b_upper) = bounds(bv, b.comparator)?;

    // "A is entirely left of B": A's upper bound at or below B's lower bound, and not
    // both attained at the same point (which would let them meet at a shared value).
    if let (Some((au, a_closed)), Some((bl, b_closed))) = (a_upper, b_lower) {
        if au < bl || (au == bl && !(a_closed && b_closed)) {
            return Some(Ordering::Less);
        }
    }
    // Symmetric check for "B is entirely left of A".
    if let (Some((bu, b_closed)), Some((al, a_closed))) = (b_upper, a_lower) {
        if bu < al || (bu == al && !(b_closed && a_closed)) {
            return Some(Ordering::Greater);
        }
    }
    None
}

/// Maps a value and comparator to its `(lower, upper)` bound pair, or `None` if the
/// comparator has no defined bound (`ad`, or any other code this crate doesn't
/// recognize as one of `<`, `<=`, `>=`, `>`).
fn bounds(value: f64, comparator: Option<&Code>) -> Option<(Bound, Bound)> {
    match comparator.map(Code::as_str) {
        None => Some((Some((value, true)), Some((value, true)))),
        Some("<") => Some((None, Some((value, false)))),
        Some("<=") => Some((None, Some((value, true)))),
        Some(">") => Some((Some((value, false)), None)),
        Some(">=") => Some((Some((value, true)), None)),
        _ => None,
    }
}

#[cfg(test)]
mod test;
