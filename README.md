This crate is simple math evaluator, with trigonometrical functions and equations written on rust (that mean that blazingly fast)

---

So, here more about usage and functional.

```rust
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
```

so, here we can see example of using

> Todo: add support for trigonometrical functions and fill variables in one expressions. ✅
---
> Todo: add this to crates.io

---
## ver 1.1.
```rust
use eval::f;

fn main() {
    println!("{}", f!("cos(p)"));
    //output: -1
    // some fixes and update functionality of macro
    //
    // now we can use trigonometrical functions and fill variable
    // in one expression like, this
    assert_eq!(f!("cos(x)" => 2.0), -0.4161468365471424);
    //also now we can round our final result to N digits after point
    //like this
    assert_eq!(f!("cos(x)" => 2.0; 2), -0.42);
    //and we can use round in simple expressions
    assert_eq!(f!("3.143721736"; 2), 3.14);
    //it also support some contast values like
    /*
    pi {p, π} = 3.1415926536
    e = 2.7182818284
    sq2 = 1.41421
    sq3 = 1.7320508075
    sq5 = 2.23606
    gamma = 0.57721
    varphi = 1.61803
    beta = 0.28016
    lambda = 0.30366
    sigma = 0.35323
    psi = 3.35988
    */
    //and it also can work like this
    println!("{}", f!("sqrt(x) * sin(pi ^ e) / 1.500001" => 2.0; 2));
}
```
