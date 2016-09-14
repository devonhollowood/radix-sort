extern crate radix_sort;

#[test]
fn simple_test() {
    let mut input = vec![0xDEADBEEF, 0xC001D00D, 0xCAFEC0DE];
    radix_sort::lsd_radix_sort(&mut input);
    assert_eq!(input, vec![0xC001D00D, 0xCAFEC0DE, 0xDEADBEEF]);
}
