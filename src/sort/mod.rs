use std::clone::Clone;
use std::collections::VecDeque;

pub fn insertion_sort <T>(arr: &[T]) -> Vec<T> 
where T: PartialOrd + Clone {
    let mut sorted = vec![arr[0].clone()];
    for i in 1..arr.len() {
        sorted.push(arr[i].clone());
        let mut j = i;
        while j > 0 && sorted[j] < sorted[j - 1 ] {
            sorted.swap(j, j -1);
            j = j - 1;
        }
    }
    sorted
}


pub fn selection_sort<T>(arr: &[T]) -> VecDeque<T>
where T: PartialOrd + Clone {
    if arr.len() == 1 {
        return VecDeque::from(arr.to_vec())
    }
    let mut sorted = selection_sort(&arr[1..]);
    let el = arr[0].clone();
    if el >= *sorted.back().unwrap(){
        sorted.push_back(el);
    } else if el <= *sorted.front().unwrap() {
        sorted.push_front(el);
    } else {
        let mut max = sorted.len() - 1;
        for i in 1..max {
            if sorted[i]>el && sorted[i] <= sorted[max] {
                max = i;
            }
        }
        sorted.insert(max, el)
    }
    sorted
}


#[cfg(test)]
mod tests{
    use super::*;

   #[test]
    fn insertion_sort_test_isize() {
        assert_eq!(vec![-1, 0, 1,2,3], insertion_sort(&vec![1,2,3, 0, -1]));
    }

   #[test]
    fn insertion_sort_test_str() {
        assert_eq!(vec!["abc", "cat", "dog", "zep", "zio"], insertion_sort(&vec!["cat", "zio", "zep", "abc", "dog"]));
    }
  
    #[test]
    fn selection_sort_test_isize() {
        assert_eq!(selection_sort(&vec![1,2,3, 0, -1]), [-1, 0, 1,2,3]);
    }

   #[test]
    fn selection_sort_test_str() {
        assert_eq!(selection_sort(&vec!["cat", "zio", "zep", "abc", "dog"]), ["abc", "cat", "dog", "zep", "zio"]);
    }


}
