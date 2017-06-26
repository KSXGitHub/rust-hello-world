pub fn swap<T: Copy>(a: &mut T, b: &mut T) {
    let c = *a;
    *a = *b;
    *b = c;
}
