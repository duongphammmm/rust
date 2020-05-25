fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    // iter() is immutable borrow
    // use into_iter() for ownership and iter_mut for mutable borrow

    // for loop takes ownership and implicitly makes v1_iter mutable
    for val in v1_iter {
        println!("Got: {}", val);
    }

    // iterator must be mutable because next() consumes (takes ownership) and changes it
    let mut v1_iter = v1.iter();
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    // consuming adaptors
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);
    // cannot use v1_iter after sum() because sum() comsumes (takes ownership) of it
    
    // itertator adaptors
    let v1: Vec<i32> = vec![1, 2, 3];
    // iterators are lazy, must use collect() to evaluate
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
}
