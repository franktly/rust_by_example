// A Rust Program is mostly made up of a series of statements
// There are a few kinds of statement in Rust. The most common two are declaring a variable
// binding, and using a `;` with an expression
// and blocks
fn main() {
    // statement
    // statement
    // statement

    // variable binding
    let x = 5;

    // expression;
    x;
    x + 1;
    15;

    let y = {
        let x_sq = x * x;
        let x_cu = x * x * x;

        // This expression will be assigned to `y`
        x_sq + x_cu
    };

    let z = {
        // This semicolon suppresses this expression and () will return
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}
