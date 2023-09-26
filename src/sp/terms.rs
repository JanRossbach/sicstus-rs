use super::sys::*;

/// Compares two terms.
///
/// # Arguments
/// * x - The first term to compare.
/// * y - The second term to compare.
///
/// # Returns
///
/// -1 if x < y, zero if x = y, and 1 if x > y.
/// See also: <https://sicstus.sics.se/sicstus/docs/latest4/pdf/sicstus.pdf#ref-lte-cte>
pub fn sp_compare(x: SP_term_ref, y: SP_term_ref) -> std::cmp::Ordering {
    let res = unsafe { SP_compare(x, y) };
    match res {
        -1 => std::cmp::Ordering::Less,
        0 => std::cmp::Ordering::Equal,
        1 => std::cmp::Ordering::Greater,
        _ => panic!("Unexpected return value from SP_compare: {}", res),
    }
}
