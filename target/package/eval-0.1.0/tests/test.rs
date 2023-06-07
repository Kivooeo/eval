#[cfg(test)]
mod tests {
    use eval::f;
    #[test]
    fn single_values() {
        assert_eq!(f!("0"), 0.0);
        assert_eq!(f!("1"), 1.0);
        assert_eq!(f!("42"), 42.0);
        assert_eq!(f!("350"), 350.0);
    }

    #[test]
    fn basic_operations() {
        assert_eq!(f!("1 + 1"), 2.0);
        assert_eq!(f!("1 - 1"), 0.0);
        assert_eq!(f!("1 * 1"), 1.0);
        assert_eq!(f!("1 / 1"), 1.0);
        assert_eq!(f!("12 * 123"), 1476.0);
    }

    #[test]
    fn whitespace_between_operators_and_operands() {
        assert_eq!(f!("1-1"), 0.0);
        assert_eq!(f!("1 -1"), 0.0);
        assert_eq!(f!("1- 1"), 0.0);
        assert_eq!(f!("1* 1"), 1.0);
    }

    #[test]
    fn unary_minuses() {
        assert_eq!(f!("1- -1"), 2.0);
        assert_eq!(f!("1--1"), 2.0);
        assert_eq!(f!("1 - -1"), 2.0);
        assert_eq!(f!("-42"), -42.0);
    }

    #[test]
    fn parentheses() {
        assert_eq!(f!("(1)"), 1.0);
        assert_eq!(f!("((1))"), 1.0);
        assert_eq!(f!("((80 - (19)))"), 61.0);
    }

    #[test]
    fn multiple_operators() {
        assert_eq!(f!("12* 123/(-5 + 2)"), -492.0);
        assert_eq!(f!("1 - -(-(-(-4)))"), -3.0);
        assert_eq!(f!("2 /2+3 * 4.75- -6"), 21.25);
        assert_eq!(f!("2 / (2 + 3) * 4.33 - -6"), 7.732);
        assert_eq!(f!("(1 - 2) + -(-(-(-4)))"), 3.0);
        assert_eq!(f!("((2.33 / (2.9+3.5)*4) - -6)"), 7.45625);
    }
}
