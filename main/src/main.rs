use macros::r;
fn main() {
    r! {
            calculate <- 2 + 2.;
            substraction <- 5 - 4;
            multi <- 3. * 10;
            division <- 10 / 23;
    };

    println!("{}", calculate);
    println!("{}", substraction);
    println!("{}", multi);
    println!("{}", division);
}
