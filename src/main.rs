#![feature(test)]
#![feature(is_sorted)]
extern crate test;

fn main() {}

#[cfg(test)]
mod tests {
    use crate::SearchSort;
    use test::Bencher;

    #[test]
    fn test_find_5() {
        let arr = vec![4, 82, 4, 32, 3, 20, 3, 2, 2, 9, 8, 7, 5, 0];

        let find = 5;

        assert_eq!(arr.find_me(find, 0, arr.len() - 1), Some(12))
    }

    #[test]
    fn test_find_empty() {
        let arr: Vec<i32> = vec![];
        let find = 5;
        assert_eq!(arr.find_me(find, 0, arr.len().saturating_sub(1)), None);
    }

    #[test]
    fn test_find_not_present() {
        let arr = vec![1, 2, 3, 4, 6, 7, 8];
        let find = 5;
        assert_eq!(arr.find_me(find, 0, arr.len() - 1), None);
    }

    #[test]
    fn test_find_present() {
        let arr = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let find = 5;
        assert_eq!(arr.find_me(find, 0, arr.len() - 1), Some(4));
    }

    #[test]
    fn test_find_negative_numbers() {
        let arr = vec![-8, -7, -5, -3, -1];
        let find = -5;
        assert_eq!(arr.find_me(find, 0, arr.len() - 1), Some(2));
    }

    #[test]
    fn test_find_multiple_occurrences() {
        let arr = vec![1, 2, 3, 3, 4, 5, 5, 5, 6, 7, 8];
        let find = 5;
        assert_eq!(arr.find_me(find, 0, arr.len() - 1), Some(5));
    }

    #[test]
    fn test_find_single_element_present() {
        let arr = vec![5];
        let find = 5;
        assert_eq!(arr.find_me(find, 0, arr.len() - 1), Some(0));
    }

    #[test]
    fn test_find_single_element_not_present() {
        let arr = vec![3];
        let find = 5;
        assert_eq!(arr.find_me(find, 0, arr.len() - 1), None);
    }

    #[test]
    fn test_find_invalid_range() {
        let arr = vec![1, 2, 3, 4, 5];
        let find = 5;
        assert_eq!(arr.find_me(find, 4, 2), None);
    }

    #[test]
    fn test_find_large_numbers() {
        let arr = vec![i32::MAX - 1, i32::MAX];
        let find = i32::MAX;
        assert_eq!(arr.find_me(find, 0, arr.len() - 1), Some(1));
    }

    #[test]
    fn test_find_small_numbers() {
        let arr = vec![i32::MIN + 1, i32::MIN];
        let find = i32::MIN;
        assert_eq!(arr.find_me(find, 0, arr.len() - 1), Some(1));
    }

    #[test]
    fn test_find_unsorted_array() {
        let arr = vec![9, 5, 2, 7, 3];
        let find = 5;
        assert_eq!(arr.find_me(find, 0, arr.len() - 1), Some(1)); // Should fail because the array is not sorted.
    }

    #[test]
    fn test_find_out_of_bound_index() {
        let arr = vec![1, 2, 3];
        let find = 3;
        assert_eq!(arr.find_me(find, 0, 5), None); // Should fail because end index is out of bounds.
    }

    #[test]
    fn test_find_zero_length() {
        let arr = vec![1, 2, 3];
        let find = 3;
        assert_eq!(arr.find_me(find, 2, 1), None); // Should fail because length is zero (end < start).
    }

    #[test]
    fn test_find_middle_element() {
        let arr = vec![1, 2, 3];
        let find = 2;
        assert_eq!(arr.find_me(find, 0, arr.len() - 1), Some(1)); // Should fail because your algorithm doesn't consider the middle element properly.
    }

    #[test]
    fn test_find_equal_elements() {
        let arr = vec![5, 5, 5, 5, 5];
        let find = 5;
        assert_eq!(arr.find_me(find, 0, arr.len() - 1), Some(0)); // Should fail because all elements are equal.
    }

    #[bench]
    fn bench_large_vector(b: &mut Bencher) {
        let large_vector: Vec<usize> = (0..1_000_000).collect();
        let find = 999_999;

        b.iter(|| {
            assert_eq!(
                large_vector.find_me(find, 0, large_vector.len() - 1),
                Some(999_999)
            );
        });
    }

    #[test]
    fn test_quicksort_empty() {
        let mut arr: Vec<i32> = vec![];
        arr.quicksort();
        assert_eq!(arr, []);
    }

    #[test]
    fn test_quicksort_single_element() {
        let mut arr = vec![1];
        arr.quicksort();
        assert_eq!(arr, [1]);
    }

    #[test]
    fn test_quicksort_multiple_elements() {
        let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6, 5];
        arr.quicksort();
        assert_eq!(arr, [1, 1, 2, 3, 4, 5, 5, 6, 9]);
    }

    #[test]
    fn test_quicksort_already_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5];
        arr.quicksort();
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_quicksort_reverse_sorted() {
        let mut arr = vec![5, 4, 3, 2, 1];
        arr.quicksort();
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_quicksort_example() {
        let mut arr = vec![
            1, 83, 873, 83, 5, 48, 2, 3, 94, 23, 48, 57, 20, 394, 85, 7, 20, 39, 48, 57, 2, 93,
            48, 57, 203, 984,
        ];
        arr.quicksort();
        assert_eq!(
            arr,
            [1, 2, 2, 3, 5, 7, 20, 20, 23, 39, 48, 48, 48, 48, 57, 57, 57, 83, 83, 85, 93, 94, 203, 394, 873, 984]
        );
    }
}

trait SearchSort<T> {
    fn find_me(&self, element: T, start: usize, end: usize) -> Option<usize>;

    fn quicksort(&mut self);
}

impl<T> SearchSort<T> for Vec<T>
where
    T: PartialEq + PartialOrd + Copy,
{
    // return the first occurence of element (left hand side)
    fn find_me(&self, element: T, start: usize, end: usize) -> Option<usize> {
        if start > end || start >= self.len() || end >= self.len() {
            return None;
        }

        let middle = (start + end) / 2;

        if self[start] == element {
            return Some(start);
        }

        if self[end] == element {
            return Some(end);
        }

        if self[middle] == element {
            return Some(middle);
        } else {
            let lhs = if middle == 0 {
                None
            } else {
                self.find_me(element, start, middle - 1)
            };
            let rhs = self.find_me(element, middle + 1, end);

            if let Some(idx) = lhs {
                return Some(idx);
            }

            if let Some(idx) = rhs {
                return Some(idx);
            }
        }

        None
    }

    fn quicksort(&mut self) {
        let slice = self.as_mut_slice();
        if !slice.is_empty() {
            let pivot_index = partition(slice).unwrap();
            let len = slice.len();

            quicksort_helper(&mut slice[0..pivot_index]);
            quicksort_helper(&mut slice[pivot_index + 1..len]);
        }
    }
}

fn quicksort_helper<T>(slice: &mut [T])
where
    T: PartialEq + PartialOrd + Copy,
{
    if !slice.is_empty() {
        let pivot_index = partition(slice).unwrap();
        let len = slice.len();

        quicksort_helper(&mut slice[0..pivot_index]);
        quicksort_helper(&mut slice[pivot_index + 1..len]);
    }
}

fn partition<T>(slice: &mut [T]) -> Result<usize, &str>
where
    T: PartialEq + PartialOrd + Copy,
{
    let len = slice.len();
    let pivot = slice[len - 1];
    let mut i = 0;

    for j in 0..len-1 {
        if slice[j] <= pivot {
            slice.swap(i, j);
            i += 1;
        }
    }

    slice.swap(i, len - 1);

    Ok(i)
}
