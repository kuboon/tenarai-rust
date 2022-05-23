fn main() {
    let mut a = 0;
    let b = 1;
    let c = mymacro::macro1!(a +++ b);
    dbg!(a, b, c);  
}
