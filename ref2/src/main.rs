fn main() {
    let var1 = 5;
    let var2 = Box::new(var1);
    let var2 = Box::new(var2);
    assert_eq!(var2, 5);
}
