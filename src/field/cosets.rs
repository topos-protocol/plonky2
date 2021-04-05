use crate::field::field::Field;

/// Finds a set of shifts that result in unique cosets for the multiplicative subgroup of size
/// `2^subgroup_bits`.
pub(crate) fn get_unique_coset_shifts<F: Field>(
    subgroup_size: usize,
    num_shifts: usize,
) -> Vec<F> {
    // From Lagrange's theorem.
    let num_cosets = (F::ORDER - 1) / (subgroup_size as u64);
    assert!(num_shifts as u64 <= num_cosets,
            "The subgroup does not have enough distinct cosets");

    // Let g be a generator of the entire multiplicative group. Let n be the order of the subgroup.
    // The subgroup can be written as <g^(|F*| / n)>. We can use g^0, ..., g^(num_shifts - 1) as our
    // shifts, since g^i <g^(|F*| / n)> are distinct cosets provided i < |F*| / n, which we checked.
    F::MULTIPLICATIVE_GROUP_GENERATOR.powers()
        .take(num_shifts)
        .collect()
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::field::cosets::get_unique_coset_shifts;
    use crate::field::crandall_field::CrandallField;
    use crate::field::field::Field;

    #[test]
    fn distinct_cosets() {
        // TODO: Switch to a smaller test field so that collision rejection is likely to occur.

        type F = CrandallField;
        const SUBGROUP_BITS: usize = 5;
        const NUM_SHIFTS: usize = 50;

        let generator = F::primitive_root_of_unity(SUBGROUP_BITS);
        let subgroup_size = 1 << SUBGROUP_BITS;

        let shifts = get_unique_coset_shifts::<F>(SUBGROUP_BITS, NUM_SHIFTS);

        let mut union = HashSet::new();
        for shift in shifts {
            let coset = F::cyclic_subgroup_coset_known_order(generator, shift, subgroup_size);
            assert!(
                coset.into_iter().all(|x| union.insert(x)),
                "Duplicate element!");
        }
    }
}
