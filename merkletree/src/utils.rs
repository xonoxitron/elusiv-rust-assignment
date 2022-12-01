pub fn is_pow2(number: u32) -> bool {
    number & (number - 1) == 0
}

pub fn split_vec<T: Clone>(arr: &Vec<T>) -> (Vec<T>, Vec<T>) {
    let m = arr.len() / 2;
    ((&arr[0..m]).to_vec(), (&arr[m..]).to_vec())
}
