//! Functions related to subsequences and substrings.

/// Compute the length of the longest common subsequence of two sequences.
///
/// Time complexity: `O(nm)` where `n = a.len()` and `m = b.len()`.
///
/// Example: `lcs(&[1,2,3,3], &[1,2,2,0,3]) = 3`
pub fn lcs<T: Eq>(a: &[T], b: &[T]) -> usize {
    let n = a.len();
    let m = b.len();
    let mut dp = vec![vec![0; m + 1]; n + 1];

    for i in (0..n).rev() {
        for j in (0..m).rev() {
            if a[i] == b[j] {
                dp[i][j] = 1 + dp[i + 1][j + 1];
            } else {
                dp[i][j] = std::cmp::max(dp[i + 1][j], dp[i][j + 1]);
            }
        }
    }

    dp[0][0]
}

#[cfg(test)]
mod tests {
    mod lcs {
        use super::super::lcs;

        #[test]
        fn both_sequences_empty() {
            assert_eq!(lcs(&[42; 0], &[42; 0]), 0);
        }

        #[test]
        fn one_sequence_empty() {
            assert_eq!(lcs(&[42; 0], &[1, 2, 3]), 0);
            assert_eq!(lcs(&[1, 2, 3], &[42; 0]), 0);
        }

        #[test]
        fn sequences_equal() {
            assert_eq!(lcs(&[1], &[1]), 1);
            assert_eq!(lcs(&[1, 2, 3], &[1, 2, 3]), 3);
        }

        #[test]
        fn one_subsequence_of_other() {
            let a = [1, 2, 3, 4, 5, 6, 7];
            let b = [2, 4, 7];
            let ans = 3;
            assert_eq!(lcs(&a, &b), ans);
            assert_eq!(lcs(&b, &a), ans);
        }

        #[test]
        fn general_case() {
            let a = [1, 2, 3, 0, 0, 0, 1, 2, 3, 4, 0, 0, 1, 2, 3];
            let b = [1, 2, 3, 9, 9, 9, 1, 2, 3, 9, 9, 4, 1, 2, 3];
            let ans = 10;
            assert_eq!(lcs(&a, &b), ans);
            assert_eq!(lcs(&b, &a), ans);
        }
    }
}
