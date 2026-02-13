#[cfg(test)]
mod tests {
    use crate::solution::Solution as sol;

    #[test]
    fn two_sum_exp_1() {
        assert_eq!(sol::two_sum(vec![2, 7, 11, 15], 9), [0, 1]);
    }

    #[test]
    fn two_sum_exp_2() {
        assert_eq!(sol::two_sum(vec![3, 2, 4], 6), [1, 2]);
    }

    #[test]
    fn two_sum_exp_3() {
        assert_eq!(sol::two_sum(vec![3, 3], 6), [0, 1]);
    }
}
