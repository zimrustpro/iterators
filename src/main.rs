fn main() {
    let my_vec = vec!['b', 'l', 'e', 's', 's', 'e', 'd'];
    let mut my_vec_iter = my_vec.iter();

    assert_eq!(my_vec_iter.next(), Some(&'b'));
    assert_eq!(my_vec_iter.next(), Some(&'l'));
    assert_eq!(my_vec_iter.next(), Some(&'e'));
    assert_eq!(my_vec_iter.next(), Some(&'s'));
    assert_eq!(my_vec_iter.next(), Some(&'s'));
    assert_eq!(my_vec_iter.next(), Some(&'e'));
    assert_eq!(my_vec_iter.next(), Some(&'d'));
    assert_eq!(my_vec_iter.next(), None);
    assert_eq!(my_vec_iter.next(), None);
}
