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

/// Compute the edit distance between two sequences.
///
/// An edit corresponds to adding, removing, or substituting a character.
///
/// Time complexity: `O(nm)` where `n = a.len()` and `m = b.len()`.
///
/// Example: `edit_distance("hello", "halo") = 2`
pub fn edit_distance<T: Eq>(a: &[T], b: &[T]) -> usize {
    let n = a.len();
    let m = b.len();
    let mut dp = vec![vec![0; m + 1]; n + 1];

    for i in 1..=n {
        dp[i][0] = 1 + dp[i - 1][0];
    }
    for i in 1..=m {
        dp[0][i] = 1 + dp[0][i - 1];
    }
    for i in 1..=n {
        for j in 1..=m {
            if a[i - 1] == b[j - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] = 1 + [dp[i - 1][j - 1], dp[i - 1][j], dp[i][j - 1]]
                    .iter()
                    .min()
                    .unwrap();
            }
        }
    }

    dp[n][m]
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

    mod edit_distance {
        use super::super::edit_distance;

        #[test]
        fn both_sequences_empty() {
            let a: [i32; 0] = [];
            let b: [i32; 0] = [];
            assert_eq!(edit_distance(&a, &b), 0);
        }

        #[test]
        fn one_sequence_empty() {
            let a = [1, 2, 3];
            let b: [i32; 0] = [];
            let ans = 3;
            assert_eq!(edit_distance(&a, &b), ans);
            assert_eq!(edit_distance(&b, &a), ans);
        }

        #[test]
        fn one_subsequence_of_other() {
            let a = [1, 2, 3, 4, 5, 6, 7];
            let b = [2, 4, 7];
            let ans = 4;
            assert_eq!(edit_distance(&a, &b), ans);
            assert_eq!(edit_distance(&b, &a), ans);
        }

        #[test]
        fn single_insert() {
            let a = [1, 2, 3, 4];
            let b = [1, 2, 4];
            let ans = 1;
            assert_eq!(edit_distance(&a, &b), ans);
        }

        #[test]
        fn single_delete() {
            let a = [1, 2, 4];
            let b = [1, 2, 3, 4];
            let ans = 1;
            assert_eq!(edit_distance(&a, &b), ans);
        }

        #[test]
        fn single_change() {
            let a = [1, 2, 3, 4];
            let b = [1, 2, 5, 4];
            let ans = 1;
            assert_eq!(edit_distance(&a, &b), ans);
            assert_eq!(edit_distance(&b, &a), ans);
        }

        #[test]
        fn general_case() {
            let a = [1, 2, 3, 4, 5, 6];
            let b = [0, 1, 2, 4, 4, 6];
            let ans = 3;
            assert_eq!(edit_distance(&a, &b), ans);
            assert_eq!(edit_distance(&b, &a), ans);
        }

        #[test]
        fn example_case() {
            let a = "hello".as_bytes();
            let b = "halo".as_bytes();
            let ans = 2;
            assert_eq!(edit_distance(&a, &b), ans);
            assert_eq!(edit_distance(&b, &a), ans);
        }
    }
}
