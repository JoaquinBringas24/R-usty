use macros::r;
fn main() {
    r!(
        library("ggplot2")

        calculate <- 2+2;
        data <- calculate + calculate;
        function add(a, b) {
            return a + b
        }


    )
}
