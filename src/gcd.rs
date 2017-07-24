pub fn euclidean(p: isize, q: isize) -> isize {
    if q == 0 {
        return p;
    }
    return euclidean(q,p % q)
}


#[cfg(test)]
mod tests{
    use super::*;

   #[test]
    fn euclidean_gcd() {
        assert_eq!(2, euclidean(2,4));
    }

}
