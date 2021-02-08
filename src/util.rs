use num_traits::ToPrimitive;

const MAX_PERM: usize = 12;
const FACTORIALS: [usize; MAX_PERM + 1] = [
    1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880, 3628800, 39916800, 479001600
];

#[inline(always)]
pub fn encode_perm<T: Copy + ToPrimitive>(arr: &[T]) -> usize {
    // TODO better array initialization?
    let mut mapped = [0u8; MAX_PERM];
    let mut index = [0u8; MAX_PERM];
    for i in 0u8..(arr.len() as u8) {
        mapped[i as usize] = i as u8;
        index[i as usize] = i as u8;
    }
    let mut res: usize = 0;
    for i in (0..arr.len()).rev() {
        let k = mapped[arr[i].to_usize().unwrap()];
        res += FACTORIALS[i] * k as usize;

        let idx = index[i];
        mapped[idx as usize] = k;
        index[k as usize] = idx;
    }
    res
}

#[inline(always)]
pub fn encode_comb<T: Copy + ToPrimitive>(arr: &[T], n: usize) -> usize {
    let mut occupied = [false; MAX_PERM];
    let mut res: usize = 0;
    for i in arr.iter() {
        occupied[i.to_usize().unwrap()] = true;
    }
    let mut k = arr.len();
    for i in (0..n).rev() {
        if occupied[i] {
            k -= 1;
        } else {
            res += comb(i, k - 1);
        }
        if k == 0 {
            break
        }
    }
    res
}

#[inline(always)]
pub fn encode_comb_opt(arr: &[bool], k: usize) -> usize {
    let mut k = k;
    let mut res = 0;
    for i in (0..arr.len()).rev() {
        if arr[i] {
            k -= 1;
        } else {
            res += comb(i, k - 1);
        }
        if k == 0 {
            break
        }
    }
    res
}

const COMB: [[usize; 4]; 13] =
    [[1, 0, 0, 0], [1, 1, 0, 0], [1, 2, 1, 0], [1, 3, 3, 1], [1, 4, 6, 4], [1, 5, 10, 10], [1, 6, 15, 20], [1, 7, 21, 35], [1, 8, 28, 56], [1, 9, 36, 84], [1, 10, 45, 120], [1, 11, 55, 165], [1, 12, 66, 220]];

#[inline(always)]
fn comb(n: usize, k: usize) -> usize {
    COMB[n][k]
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_encode_comb() {
        assert_eq!(encode_comb(&[8, 9, 10, 11], 12), 0);
        assert_eq!(encode_comb(&[3, 6, 9, 11], 12), 62);
        assert_eq!(encode_comb(&[1, 4, 8, 9], 12), 305);
        assert_eq!(encode_comb(&[0, 1, 2, 3], 12), 494);
    }

    fn get_opt_repr(repr: &[usize]) -> [bool; 12] {
        let mut buf = [false; 12];
        for i in repr {
            buf[*i] = true
        }
        buf
    }

    #[test]
    fn test_encode_comb_opt() {
        assert_eq!(encode_comb_opt(&get_opt_repr(&[8, 9, 10, 11]), 4), 0);
        assert_eq!(encode_comb_opt(&get_opt_repr(&[3, 6, 9, 11]), 4), 62);
        assert_eq!(encode_comb_opt(&get_opt_repr(&[1, 4, 8, 9]), 4), 305);
        assert_eq!(encode_comb_opt(&get_opt_repr(&[0, 1, 2, 3]), 4), 494);
    }

}
