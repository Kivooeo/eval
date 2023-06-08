use eval::f;

fn main() {
    println!("{}", f!("cos(x)" => 1.0; 2));
}
