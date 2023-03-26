// use std::{thread, time};

fn bubble_sort(a: &mut Vec<i32>) {
    // item below limit are not jet unsorted
    for limit in (0..a.len()).rev() {
        for i in 0..limit {
            let j = dbg!(i + 1);
            if a[j] < a[i] {
                a.swap(i, j)
            }
        }
    }
}

fn main() {
    let mut v1 = vec![0, 1, 3, 5, 4, 8, 7, 2];
    bubble_sort(&mut v1);
    println!("{:#?}", v1);
}
