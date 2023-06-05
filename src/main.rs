struct Lexer {
    expression: String,
}

fn eval(x: &str) -> String {
    let operator: (usize, char) = {
        let mut tmp: (usize, char) = (0, '1');
        for i in x.chars().enumerate() {
            if matches!(i.1, '+' | '-' | '*' | '/') {
                tmp = i;
            }
        }
        if tmp.1 == '1' {
            panic!("He");
        }
        tmp
    };
    println!("{x}");
    println!("{:?}", operator);
    let mut left: ((usize, usize), f32) = ((0, operator.0), 0.0);
    let mut right: (usize, f32) = (operator.0, 0.0);
    left.1 = x[left.0 .0..left.0 .1].trim().parse().unwrap();
    println!("{:?}", left);
    right.1 = x[right.0 + 1..].trim().parse().unwrap();
    println!("{:?}", right);

    let result: f32 = match operator.1 {
        '+' => left.1 + right.1,
        '-' => left.1 - right.1,
        '*' => left.1 * right.1,
        '/' => left.1 / right.1,
        _ => panic!("Bad token"),
    };
    result.to_string()
}

impl Lexer {
    fn new(n: &str) -> Self {
        Self {
            expression: n.to_string(),
        }
    }
    fn par(&self) -> Vec<(usize, usize)> {
        let input = self.expression.as_bytes();
        let mut stack = Vec::new();
        let mut result = Vec::new();
        for (i, &c) in input.iter().enumerate() {
            match c {
                b'(' => stack.push(i),
                b')' => {
                    if let Some(start) = stack.pop() {
                        result.push((start, i));
                    }
                }
                _ => {}
            }
        }
        return result;
    }
    fn basic(&self) -> Vec<(usize, char)> {
        let mut result: Vec<(usize, char)> = Vec::new();
        for i in self.expression.chars().enumerate() {
            if matches!(i.1, '*' | '/') {
                result.push(i);
            }
        }
        for i in self.expression.chars().enumerate() {
            if matches!(i.1, '+' | '-') {
                result.push(i);
            }
        }
        println!("{result:?}");
        return result;
    }
    fn parse(&mut self) -> f32 {
        self.expression = self
            .expression
            .chars()
            .filter(|x| !x.is_ascii_whitespace())
            .collect();
        println!("{}", self.expression);
        //TODO! : ( 1 + ( 2 * 2 ) + 2 )
        let mut pars = self.par();
        loop {
            if pars.len() == 0 {
                break;
            }
            self.expression.replace_range(
                pars[0].0..pars[0].1,
                &eval(&self.expression[pars[0].0..pars[0].1]),
            );
            pars.remove(0);
            println!("{}", self.expression);
        }
        drop(pars);
        let mut basics: Vec<(usize, char)> = self.basic();
        loop {
            //TODO!: make is works
        }
        0.0
    }
}

fn main() {
    let mut e = Lexer::new("1 + 2 * 2 * 2 / 2");
    println!("{}", eval("2*2"));
    println!("{}", e.parse());
}
