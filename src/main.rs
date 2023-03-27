use rayon;

fn bubble_sort(a: &mut Vec<i32>) {
    // item below limit are not jet unsorted
    for limit in (0..a.len()).rev() {
        for i in 0..limit {
            let j = i + 1;
            if a[j] < a[i] {
                a.swap(i, j)
            }
        }
    }
}

/// move item at i to correct position of a.sorted "pivi"
/// items left of pivi are all smaller then a[pivi]
/// items to the right are larger
fn pivot(a: &mut [i32], i: usize) -> usize {
    let n = a.len();
    let piv = a[i];
    a.swap(i, n - 1);
    let mut ileft = 0;
    let mut iright = n - 1;
    loop {
        for i in ileft..(iright + 1) {
            let v = a[i];
            ileft = i;
            if v > piv {
                break;
            }
        }
        for i in (ileft..iright).rev() {
            let v = a[i];
            iright = i;
            if v < piv {
                break;
            }
        }

        if ileft < iright {
            a.swap(ileft, iright);
            ileft += 1;
            iright -= 1;
        } else {
            break;
        }
    }
    a.swap(ileft, n - 1);
    ileft
}

fn quick_sort(a: &mut [i32]) {
    let l = a.len();
    if l < 2 {
        return;
    }
    let pivi = pivot(a, l / 2);
    let (left, right) = a.split_at_mut(pivi);

    rayon::join(|| quick_sort(left), || quick_sort(right));
    quick_sort(right)
}

fn main() {
    //let foo  = vec![0, 1, 2, 3, 4, 5, 6, 7];

    for i in 1..1 {
        println!("hello {}", i);
    }
    let mut v1 = [0, 1, 6, 5, 4, 8, 7, 2];
    quick_sort(&mut v1);
    dbg!(v1);
}
