pub fn binary_search <T>(value: T, arr: &[T])->isize 
where T: PartialOrd{ 
    if arr.len() == 0 {
        return -1;
    }
    else if arr[0] == value {
        return 0;
    }
    else if arr[arr.len() - 1] == value {
        return (arr.len() - 1) as isize;
    }
    else if arr.len() == 1 {
        return -1;
    }
    
    let mid = arr.len() / 2;
    if value > arr[mid] {
        let index = binary_search(value, &arr[mid..]);
        if index > 0 {
            mid as isize +index
        } else {
            -1
        }
    } else if value < arr[mid] {
        binary_search(value, &arr[0..mid])
    }
    else {
        mid as isize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_for_int_absent() {
        assert_eq!(-1, binary_search(12, &[1,2,3,4]));
    }

    #[test]
    fn search_for_str_absent() {
        assert_eq!(-1, binary_search("cat", &["abc", "cad", "zed"]));
    }
    
    #[test]
    fn search_for_int_present() {
        assert_eq!(0, binary_search(1, &[1,2,3,4]));
    }

    #[test]
    fn search_for_int_present_mid() {
        assert_eq!(1, binary_search(2, &[1,2,3,4]));
    }

    #[test]
    fn search_for_str_present_last() {
        assert_eq!(2, binary_search("cat", &["abc", "cad", "cat"]));
    }
}
