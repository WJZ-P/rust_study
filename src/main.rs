fn main() {
    let mut vec = vec![1.0, 5.6, 10.3, 2.0, 15f32];
    vec.sort_unstable();
    assert_eq!(vec, vec![1.0, 2.0, 5.6, 10.3, 15f32]);
}