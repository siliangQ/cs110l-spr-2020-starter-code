/* The following exercises were borrowed from Will Crichton's CS 242 Rust lab. */

use std::collections::HashSet;

fn main() {
    println!("Hi! Try running \"cargo test\" to run tests.");
}
// create a new vector from old vector
// we need to consume the old vector and build a new vector
// use into_iter
fn add_n(v: Vec<i32>, n: i32) -> Vec<i32> {
    //v.iter().map(|v| *v + n).collect()
    v.into_iter().map(|v| v + n).collect()
}

// modify the value in place
// we need to execute the iterator by using collect and Vec<_>
fn add_n_inplace(v: &mut Vec<i32>, n: i32) {
    let _: Vec<_> = v.iter_mut().map(|v| *v += n).collect();
}

// use Hashset and retain function to remove the elements from vector
fn dedup(v: &mut Vec<i32>) {
    let mut uniques = HashSet::new();
    v.retain(|e| uniques.insert(*e));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_n() {
        assert_eq!(add_n(vec![1], 2), vec![3]);
    }

    #[test]
    fn test_add_n_inplace() {
        let mut v = vec![1];
        add_n_inplace(&mut v, 2);
        assert_eq!(v, vec![3]);
    }

    #[test]
    fn test_dedup() {
        let mut v = vec![3, 1, 0, 1, 4, 4];
        dedup(&mut v);
        assert_eq!(v, vec![3, 1, 0, 4]);
    }
}
