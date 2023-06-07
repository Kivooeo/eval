use eval::f;

fn main() {
    // so we can evaluate simple math expressions
    assert_eq!(f!("2 + 2"), 4.0);

    //and we can evaluata strange math expressions
    assert_eq!(f!("2 ---- +++++ 2"), 4.0);

    // we can consider brackets in our expressions
    assert_eq!(f!("(x + x) * x" => 2), 8.0);

    // and as we saw in the previous example we can fill variables with some values
    assert_eq!(f!("2 ---- +++++ x - x + y" => 2.0, 4.0), 6.0);
    //                         ^    ^  ^
    //                      x take 2.0|and y take 4.0

    // and we can evaluate trigonometrical functions like this
    assert_eq!(f!("cos(30)"), 0.15425144988758405);

    // ! We can't use trigonometrical functions and fill variable in one expression
}
