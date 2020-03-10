struct BitonicArraySearcher {
    numbers: Vec<i64>,
}

impl BitonicArraySearcher {
    fn contains(&self, number: i64) -> bool {
        false
    }
}


#[cfg(test)]
mod tests {
    use crate::BitonicArraySearcher;

    #[test]
    fn searcher_does_not_contain_right() {
        assert!(!BitonicArraySearcher {
            numbers: vec![2, 4, 6, 8, 10, 9, 7, 3, 1]
        }.contains(5));
    }

    #[test]
    fn searcher_does_not_contain_left() {
        assert!(!BitonicArraySearcher {
            numbers: vec![2, 4, 8, 10, 9, 7, 5, 3, 1]
        }.contains(6));
    }

    #[test]
    fn searcher_does_not_contain_above_max() {
        assert!(!BitonicArraySearcher {
            numbers: vec![2, 4, 6, 8, 10, 9, 7, 5, 3, 1]
        }.contains(11));
    }

    #[test]
    fn searcher_does_not_contain_below_min() {
        assert!(!BitonicArraySearcher {
            numbers: vec![2, 4, 6, 8, 10, 9, 7, 5, 3, 1]
        }.contains(0));
    }

    #[test]
    fn searcher_contains_left_with_left_mid() {
        assert!(BitonicArraySearcher {
            numbers: vec![2, 4, 6, 8, 10, 9, 7, 5]
        }.contains(4));
    }

    #[test]
    fn searcher_contains_left_limit_with_left_mid() {
        assert!(BitonicArraySearcher {
            numbers: vec![2, 4, 6, 8, 10, 9, 7, 5]
        }.contains(2));
    }

    #[test]
    fn searcher_contains_left_with_right_mid() {
        assert!(BitonicArraySearcher {
            numbers: vec![2, 4, 6, 8, 10, 9, 7, 5, 3, 1]
        }.contains(4));
    }

    #[test]
    fn searcher_contains_left_limit_with_right_mid() {
        assert!(BitonicArraySearcher {
            numbers: vec![2, 4, 6, 8, 10, 9, 7, 5, 3, 1]
        }.contains(2));
    }

    #[test]
    fn searcher_contains_right_with_left_mid() {
        assert!(BitonicArraySearcher {
            numbers: vec![2, 4, 6, 8, 10, 9, 7, 5]
        }.contains(3));
    }

    #[test]
    fn searcher_contains_right_limit_with_left_mid() {
        assert!(BitonicArraySearcher {
            numbers: vec![2, 4, 6, 8, 10, 9, 7, 5]
        }.contains(1));
    }

    #[test]
    fn searcher_contains_right_with_right_mid() {
        assert!(BitonicArraySearcher {
            numbers: vec![2, 4, 6, 8, 10, 9, 7, 5, 3, 1]
        }.contains(3));
    }

    #[test]
    fn searcher_contains_right_limit_with_right_mid() {
        assert!(BitonicArraySearcher {
            numbers: vec![2, 4, 6, 8, 10, 9, 7, 5, 3, 1]
        }.contains(1));
    }

    #[test]
    fn searcher_contains_middle() {
        assert!(BitonicArraySearcher {
            numbers: vec![2, 4, 6, 8, 10, 9, 7, 5, 3]
        }.contains(10));
    }
}
