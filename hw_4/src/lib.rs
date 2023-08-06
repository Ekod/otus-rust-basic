pub type TupI32 = (i32, i32);
pub type SlcI32 = [i32];

pub fn get_tuple_elems(tup: &mut TupI32, get_last_elem: bool) -> &mut i32 {
    if get_last_elem {
        &mut tup.1
    } else {
        &mut tup.0
    }
}

pub fn get_slice_nth_elem(slc: &mut SlcI32, n: usize) -> Option<&mut i32> {
    if slc.is_empty() {
        return None;
    }

    let last_elem_idx = slc.len() - 1;

    if n <= last_elem_idx {
        Some(&mut slc[n])
    } else {
        None
    }
}

pub fn get_slice_nth_elem_from_end(slc: &SlcI32, n: usize) -> Option<&i32> {
    if slc.is_empty() {
        return None;
    }

    let last_elem_idx = slc.len() - 1;

    if n <= last_elem_idx {
        let mut result = None;

        for i in (0..=last_elem_idx).rev() {
            let elem_reversed_idx = last_elem_idx - i;

            if n == elem_reversed_idx {
                result = Some(&slc[i]);
                break;
            }
        }

        result
    } else {
        None
    }
}

pub fn get_two_slices_from_one(slc: &SlcI32, n: usize) -> Option<(&[i32], &[i32])> {
    if !slc.is_empty() {
        Some((&slc[..n], &slc[n..]))
    } else {
        None
    }
}

pub fn get_slices_array_from_slice(slc: &SlcI32) -> Vec<&[i32]> {
    let required_slc_len = 4;

    if slc.len() >= required_slc_len {
        let mut result = Vec::new();
        let slc_parts = slc.len() / 4;
        let mut next_part_holder = slc_parts;

        for i in 0..required_slc_len {
            if i == 0 {
                result.push(&slc[..slc_parts]);
            } else if next_part_holder + slc_parts <= slc.len()
                && next_part_holder + slc_parts < required_slc_len
            {
                let next_elem = next_part_holder + slc_parts;

                result.push(&slc[next_part_holder..next_elem]);
                next_part_holder += slc_parts;
            } else {
                result.push(&slc[next_part_holder..]);
            }
        }

        result
    } else {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_tuple_elems_success() {
        let mut tup: TupI32 = (1, 2);
        let mut get_last_elem = false;

        let tup_mut_ref = &mut tup;

        let mut expected = 1;
        let actual = get_tuple_elems(tup_mut_ref, get_last_elem);
        assert_eq!(actual, &mut expected);

        get_last_elem = true;
        let mut expected = 2;
        let actual = get_tuple_elems(tup_mut_ref, get_last_elem);
        assert_eq!(actual, &mut expected)
    }

    #[test]
    fn get_slc_nth_elem_success() {
        let mut vec = Vec::from([1, 2, 3]);
        let idx = 1;
        let vec_mut_ref = &mut vec;

        let mut expected = 2;
        let result = get_slice_nth_elem(vec_mut_ref, idx);
        match result {
            Some(actual) => {
                assert_eq!(actual, &mut expected);
            }
            None => panic!("Elem not found"),
        }
    }

    #[test]
    fn get_slc_nth_elem_from_end_success() {
        let vec = Vec::from([3, 2, 1]);
        let mut idx = 0;
        let vec_ref = &vec;

        let mut expected = 1;
        let mut result = get_slice_nth_elem_from_end(vec_ref, idx);
        match result {
            Some(actual) => {
                assert_eq!(actual, &expected);
            }
            None => panic!("Elem not found"),
        }

        idx = 2;
        expected = 3;
        result = get_slice_nth_elem_from_end(vec_ref, idx);
        match result {
            Some(actual) => {
                assert_eq!(actual, &expected);
            }
            None => panic!("Elem not found"),
        }
    }

    #[test]
    fn get_two_slices_from_one_success() {
        let vec = Vec::from([1, 2, 3]);
        let separator = 1;
        let vec_ref = &vec;

        let expected = (&vec[..1], &vec[1..]);
        let result = get_two_slices_from_one(vec_ref, separator);
        match result {
            Some(actual) => {
                assert_eq!(actual, expected);
            }
            None => panic!("Elem not found"),
        }
    }

    #[test]
    fn get_slices_array_from_slice_success() {
        let vec = Vec::from([1, 2, 3, 4]);
        let vec_ref = &vec;

        let expected = Vec::from([&[1], &[2], &[3], &[4]]);
        let result = get_slices_array_from_slice(vec_ref);

        assert_eq!(result, expected);

        let vec2 = Vec::from([1, 2, 3, 4, 5]);
        let vec2_ref = &vec2;

        let expected = Vec::from([&[1][..], &[2][..], &[3][..], &[4, 5][..]]);
        let result = get_slices_array_from_slice(vec2_ref);

        assert_eq!(expected, result);
    }
}
