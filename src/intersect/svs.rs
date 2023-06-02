use crate::{
    intersect::{
        galloping_inplace,
        Intersect2,
    },
    visitor::{Visitor, SliceWriter},
};


/// "Small vs. Small" adaptive set intersection algorithm.
/// Assumes input sets are ordered from smallest to largest.
pub fn svs<T>(sets: &[&[T]], out: &mut [T]) -> usize
where
    T: Ord + Copy,
{
    assert!(sets.len() >= 2);

    let mut count = 0;

    // Copies smallest set into (temporary) output set.
    // Is there a better way to do this?
    out[..sets[0].len()].clone_from_slice(sets[0]);

    for set in sets.iter().skip(1) {
        count = galloping_inplace(&mut out[..count], set);
    }
    count
}

pub fn svs_inplace<T>(sets: &mut [&mut [T]]) -> usize
where
    T: Ord + Copy,
{
    assert!(sets.len() >= 2);

    let mut count = 0;
    let mut iter = sets.iter_mut();

    let first = unsafe { iter.next().unwrap_unchecked() };

    for set in iter {
        count = galloping_inplace(&mut first[0..count], set);
    }
    count
}



/// Extends 2-set intersection algorithms to k-set.
/// Since SIMD algorithms cannot operate in place, to extend them to k sets, we
/// must use an additional output vector.
/// Returns (intersection length, final output index)
pub fn as_svs<T, V>(
    sets: &[&[T]],
    out0: &mut [T],
    out1: &mut [T],
    intersect: fn(&[T], &[T], &mut SliceWriter<T>) -> usize
) -> (usize, usize)
where
    T: Ord + Copy,
{
    assert!(sets.len() >= 2);

    let mut count = 0;
    let mut out_index = 0;

    {
        let mut writer = SliceWriter::from(&mut *out1);
        count = intersect(sets[0], sets[1], &mut writer);
    }

    for set_b in sets.iter().skip(2) {
        // Alternate output sets.
        let (mut writer, set_a) = if out_index == 0 {
            (SliceWriter::from(&mut *out1), &out0[..count])
        }
        else {
            (SliceWriter::from(&mut *out0), &out1[..count])
        };
        count = intersect(&set_a, set_b, &mut writer);
        out_index = !out_index;
    }

    (count, out_index)
}
