#[allow(dead_code)]
fn legacy_partition<T>(slice: &mut [T]) -> usize
where
    T: PartialOrd + Copy,
{
    let mut front = 0;
    let mut back = slice.len() - 1;
    let mut pivot = front;
    let pivot_val = slice[pivot].clone();
    while front < back {
        while front < back && slice[back] >= pivot_val {
            back -= 1;
        }
        slice[front] = slice[back];
        while front < back && slice[front] <= pivot_val {
            front += 1;
        }
        slice[back] = slice[front];
    }
    // assert_eq!(front, back);
    slice[front] = pivot_val;
    pivot = front;
    pivot
}

#[allow(dead_code)]
fn partition<T>(slice: &mut [T]) -> usize
where
    T: PartialOrd + Copy,
{
    let front = 0;
    let back = slice.len() - 1;
    let mid = (front + back) / 2;
    let mut pivot = back;
    slice.swap(pivot, back);
    let mut eof_less = front; // [begin, eof_less) contain 0 element at first
    let mut eof_greater = front; // [eof_less, eof_great) contain 0 element at first
    while eof_greater < back {
        if slice[eof_greater] < slice[back] {
            slice.swap(eof_greater, eof_less);
            eof_less += 1;
        }
        eof_greater += 1;
    }
    slice.swap(eof_less, back);
    pivot = eof_less;
    pivot
}

/// quick_sort
pub fn quick_sort<T: PartialOrd + Copy>(slice: &mut [T]) {
    if slice.len() <= 1 {
        return;
    }
    let pivot = partition(slice);
    quick_sort(&mut slice[..pivot]);
    quick_sort(&mut slice[pivot + 1..]);
}
