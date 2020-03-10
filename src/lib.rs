use crate::MidSide::{Middle, Right, Left};

struct BitonicArraySearcher {
    numbers: Vec<i64>,
}

#[derive(PartialEq)]
enum MidSide {
    Left,
    Middle,
    Right,
}

impl BitonicArraySearcher {
    fn contains(&self, number: i64) -> i64 {
        let mut mid = self.numbers.len() / 2;
        let mut left_low_pointer = 0;
        let mut left_high_pointer = mid - 1;
        let mut right_low_pointer = mid;
        let mut right_high_pointer = self.numbers.len() - 1;
        while left_low_pointer <= left_high_pointer || right_low_pointer <= right_high_pointer {
            if number == self.numbers[mid] {
                return mid as i64;
            };
            if left_low_pointer == left_high_pointer && number != self.numbers[left_low_pointer]
                && right_low_pointer == right_high_pointer && number != self.numbers[right_low_pointer] {
                break;
            }
            let mid_side = self.get_mid_side(mid as i64);
            if mid_side == Left && number > self.numbers[mid] {
                right_high_pointer = mid;
                mid = (right_high_pointer - left_low_pointer) / 2;
                right_low_pointer = mid + 1;
                left_high_pointer = mid - 1;
            } else if mid_side == Right && number > self.numbers[mid] {
                left_low_pointer = mid;
                mid = (right_high_pointer - left_low_pointer) / 2;
                left_high_pointer = mid - 1;
                right_low_pointer = mid + 1;
            }
            if left_low_pointer <= left_high_pointer {
                let left_mid = ((left_high_pointer - left_low_pointer) / 2) + left_low_pointer;
                if number == self.numbers[left_mid] {
                    return left_mid as i64;
                }
                if number > self.numbers[left_mid] {
                    left_low_pointer = left_mid + 1;
                }
                if number < self.numbers[left_mid] {
                    left_high_pointer = if left_mid == 0 { 0 } else { left_mid - 1 };
                }
                if number == self.numbers[left_mid] {
                    return left_mid as i64;
                };
            }
            if right_low_pointer <= right_high_pointer {
                let right_mid = ((right_high_pointer - right_low_pointer) / 2) + right_low_pointer;
                if number == self.numbers[right_mid] {
                    return right_mid as i64;
                }
                if number > self.numbers[right_mid] {
                    right_high_pointer = if right_mid == 0 { 0 } else { right_mid - 1 };
                }
                if number < self.numbers[right_mid] {
                    right_low_pointer = right_mid + 1;
                }
                if number == self.numbers[right_mid] {
                    return right_mid as i64;
                };
            }
        }
        -1
    }

    fn get_mid_side(&self, mid: i64) -> MidSide {
        let mid = mid as usize;
        if self.numbers[mid] > self.numbers[mid - 1] && self.numbers[mid] < self.numbers[mid + 1] {
            Middle
        } else if self.numbers[mid] > self.numbers[mid - 1] {
            Right
        } else {
            Left
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::BitonicArraySearcher;

    #[test]
    fn searcher_does_not_contain_right() {
        assert_eq!(BitonicArraySearcher {
            numbers: vec![2, 4, 6, 8, 10, 9, 7, 3, 1]
        }.contains(5), -1);
    }

    #[test]
    fn searcher_does_not_contain_left() {
        assert_eq!(BitonicArraySearcher {
            numbers: vec![2, 4, 8, 10, 9, 7, 5, 3, 1]
        }.contains(6), -1);
    }

    #[test]
    fn searcher_does_not_contain_above_max() {
        assert_eq!(BitonicArraySearcher {
            numbers: vec![2, 4, 6, 8, 10, 9, 7, 5, 3, 1]
        }.contains(11), -1);
    }

    #[test]
    fn searcher_does_not_contain_below_min() {
        assert_eq!(BitonicArraySearcher {
            numbers: vec![2, 4, 6, 8, 10, 9, 7, 5, 3, 1]
        }.contains(0), -1);
    }

    #[test]
    fn searcher_contains_left_with_left_mid() {
        assert_eq!(BitonicArraySearcher {
            numbers: vec![2, 4, 6, 8, 10, 9, 7, 5]
        }.contains(4), 1);
    }

    #[test]
    fn searcher_contains_left_limit_with_left_mid() {
        assert_eq!(BitonicArraySearcher {
            numbers: vec![2, 4, 6, 8, 10, 9, 7, 5]
        }.contains(2), 0);
    }

    #[test]
    fn searcher_contains_left_with_right_mid() {
        assert_eq!(BitonicArraySearcher {
            numbers: vec![2, 4, 6, 8, 10, 9, 7, 5, 3, 1]
        }.contains(4), 1);
    }

    #[test]
    fn searcher_contains_left_limit_with_right_mid() {
        assert_eq!(BitonicArraySearcher {
            numbers: vec![2, 4, 6, 8, 10, 9, 7, 5, 3, 1]
        }.contains(2), 0);
    }

    #[test]
    fn searcher_contains_right_with_left_mid() {
        assert_eq!(BitonicArraySearcher {
            numbers: vec![2, 4, 6, 8, 10, 9, 7, 5]
        }.contains(7), 6);
    }

    #[test]
    fn searcher_contains_right_limit_with_left_mid() {
        assert_eq!(BitonicArraySearcher {
            numbers: vec![2, 4, 6, 8, 10, 9, 7, 5]
        }.contains(5), 7);
    }

    #[test]
    fn searcher_contains_right_with_right_mid() {
        assert_eq!(BitonicArraySearcher {
            numbers: vec![2, 4, 6, 8, 10, 9, 7, 5, 3, 1]
        }.contains(3), 8);
    }

    #[test]
    fn searcher_contains_right_limit_with_right_mid() {
        assert_eq!(BitonicArraySearcher {
            numbers: vec![2, 4, 6, 8, 10, 9, 7, 5, 3, 1]
        }.contains(1), 9);
    }

    #[test]
    fn searcher_contains_middle() {
        assert_eq!(BitonicArraySearcher {
            numbers: vec![2, 4, 6, 8, 10, 9, 7, 5, 3]
        }.contains(10), 4);
    }
}
