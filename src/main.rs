struct Lexer {
    expression: String,
}

fn basic(x: &str) -> Vec<(usize, char)> {
    let mut result: Vec<(usize, char)> = Vec::new();
    for i in x.chars().enumerate() {
        if matches!(i.1, '^') {
            result.push(i);
        }
    }
    for i in x.chars().enumerate() {
        if matches!(i.1, '*' | '/' | '%') {
            result.push(i);
        }
    }
    for i in x.chars().enumerate() {
        let mut y = '0';
        if i.0 > 0 {
            y = match x.chars().nth(i.0 - 1) {
                Some(val) => val,
                None => '0',
            };
        }

        if matches!(i.1, '+' | '-') {
            if matches!(y, '*' | '/' | '%' | '+' | '-' | '^') && i.1 == '-' {
                continue;
            }
            if !(i.0 == 0 && i.1 == '-') {
                result.push(i);
            }
        }
    }
    return result;
}

fn eval(x: &str) -> String {
    let mut counter = 0;
    let mut x = x.to_string();

    for i in x.chars() {
        if matches!(i, '+' | '-' | '*' | '/' | '^' | '%') {
            counter += 1;
        }
    }
    let mut result = String::new();
    match counter {
        1 => {
            if x.starts_with("-") {
                return x;
            }
            let operator: (usize, char) = {
                let mut tmp: (usize, char) = (0, '1');
                for i in x.chars().enumerate() {
                    if matches!(i.1, '+' | '-' | '*' | '/' | '^' | '%') {
                        tmp = i;
                    }
                }
                if tmp.1 == '1' {
                    panic!("He");
                }
                tmp
            };
            let mut left: ((usize, usize), f32) = ((0, operator.0), 0.0);
            let mut right: (usize, f32) = (operator.0, 0.0);
            left.1 = x[left.0 .0..left.0 .1].trim().parse().unwrap();
            right.1 = x[right.0 + 1..].trim().parse().unwrap();

            let result1: f32 = match operator.1 {
                '+' => left.1 + right.1,
                '-' => left.1 - right.1,
                '*' => left.1 * right.1,
                '/' => left.1 / right.1,
                '^' => left.1.powf(right.1),
                '%' => left.1 % right.1,
                _ => panic!("Bad token"),
            };

            result = result1.to_string();
        }
        y if y <= 0 => {
            result = x;
        }
        _ => {
            let mut basics: Vec<(usize, char)> = basic(&x);
            let mut res = 0f64;
            while basics.len() != 0 {
                let mut left = 0f64;
                let mut right = 0f64;
                let mut left_idx = 0usize;
                let mut right_idx = 0usize;

                if basics.len() == 1 && basics[0].1 == '-' && basics[0].0 == 1 {
                    basics.remove(0);
                }
                basics = basic(&x);
                println!("{basics:?}");
                println!("eq. is {x}");
                let i = basics[0];
                if basics.len() == 1
                    || (basics.len() == 2 && basics[1].1 == '-' && basics[1].0 - 1 == basics[0].0)
                {
                    left = x[0..i.0].parse::<f64>().expect("Error value");
                    right = x[i.0 + 1..].parse::<f64>().expect("Error value");
                } else {
                    left_idx = i.0 - 1;
                    while left_idx > 0
                        && (x.chars().nth(left_idx - 1).unwrap().is_digit(10)
                            || x.chars().nth(left_idx - 1).unwrap() == '.'
                            || x.chars().nth(left_idx - 1).unwrap() == '-')
                    {
                        if x.chars().nth(left_idx - 1).unwrap() == '-' {
                            left_idx -= 1;
                            break;
                        }
                        left_idx -= 1;
                    }
                    left = x[left_idx..i.0].parse::<f64>().expect("Error value");
                    println!("left is {}", left);
                    right_idx = i.0 + 1;
                    println!("right idx {}", right_idx);
                    println!("x len is {}", x.len());
                    let mut first_char = x.chars().nth(right_idx).unwrap();
                    println!("first char is {first_char}");
                    if first_char == '-' {
                        right_idx += 1;
                    }
                    while right_idx < x.len()
                        && (x.chars().nth(right_idx).unwrap().is_digit(10)
                            || x.chars().nth(right_idx).unwrap() == '.')
                    {
                        right_idx += 1;
                    }
                    println!("right is {}", &x[i.0 + 1..right_idx]);
                    right = x[i.0 + 1..right_idx].parse::<f64>().expect("Error value");
                }
                res = match i.1 {
                    '+' => left + right,
                    '-' => left - right,
                    '*' => left * right,
                    '/' => left / right,
                    '^' => left.powf(right),
                    '%' => left % right,
                    _ => panic!("hehehe"),
                };
                println!("basics is {:?}", basics);
                println!("{} - {} = {}", left_idx, right_idx, x);
                if basics.len() == 1 {
                    x = res.to_string();
                } else {
                    x.replace_range(left_idx..right_idx, &res.to_string());
                }
                basics = basic(&x);
            }
            result = res.to_string();
            return result;
        }
    }
    result
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
            if matches!(i.1, '^') {
                result.push(i);
            }
        }
        for i in self.expression.chars().enumerate() {
            if matches!(i.1, '*' | '/' | '%') {
                result.push(i);
            }
        }
        for i in self.expression.chars().enumerate() {
            let mut x = '0';

            if i.0 > 0 {
                x = match self.expression.chars().nth(i.0 - 1) {
                    Some(val) => val,
                    None => '0',
                };
            }

            if matches!(i.1, '+' | '-') {
                if matches!(x, '*' | '/' | '%' | '+' | '-' | '^') && i.1 == '-' {
                    continue;
                }
                if !(i.0 == 0 && i.1 == '-') {
                    result.push(i);
                }
            }
        }
        return result;
    }
    fn parse(&mut self) -> f32 {
        let mut result = 0f32;
        self.expression = self
            .expression
            .chars()
            .filter(|x| !x.is_ascii_whitespace())
            .collect();
        let mut trigs = find_trig_functions_indices(&self.expression);

        while trigs.len() != 0 {
            let i = trigs[0];
            let tmp = {
                let x = &self.expression.clone()[i.0..=i.1];
                let idx = x.chars().position(|x| x == '(').unwrap();

                let function = &x[0..idx];
                let argument = &x[idx + 1..x.len() - 1];
                let argument: f32 = argument.parse().unwrap();
                let res = match function {
                    "cos" => argument.cos().to_string(),
                    "sin" => argument.sin().to_string(),
                    "tan" => argument.tan().to_string(),
                    "acos" => argument.acos().to_string(),
                    "asin" => argument.asin().to_string(),
                    "atan" => argument.atan().to_string(),
                    "cosh" => argument.cosh().to_string(),
                    "sinh" => argument.sinh().to_string(),
                    "tanh" => argument.tanh().to_string(),
                    "acosh" => argument.acosh().to_string(),
                    "asinh" => argument.asinh().to_string(),
                    "atanh" => argument.atanh().to_string(),
                    "abs" => argument.abs().to_string(),
                    _ => "Unsupported function".to_string(),
                };
                self.expression.replace_range(i.0..=i.1, &res);

                trigs = find_trig_functions_indices(&self.expression);
            };
        }
        let mut pars = self.par();
        loop {
            if pars.len() == 0 {
                break;
            }
            self.expression.replace_range(
                pars[0].0..=pars[0].1,
                &eval(&self.expression[pars[0].0 + 1..pars[0].1]),
            );
            pars = self.par();
        }
        drop(pars);
        let mut basics: Vec<(usize, char)> = self.basic();
        self.expression = eval(&self.expression);
        self.expression.parse().unwrap()
    }
}

fn find_trig_functions_indices(expression: &str) -> Vec<(usize, usize)> {
    let mut indices = Vec::new();
    let mut start_index = 0;
    let mut is_inside_trig_func = false;

    for (index, c) in expression.chars().enumerate() {
        if c.is_alphabetic() {
            if !is_inside_trig_func {
                start_index = index;
                is_inside_trig_func = true;
            }
        } else if c == '(' && is_inside_trig_func {
            let end_index = find_matching_parenthesis(&expression[index..]);
            if let Some(end_index) = end_index {
                indices.push((start_index, index + end_index));
                is_inside_trig_func = false;
            }
        } else {
            is_inside_trig_func = false;
        }
    }

    indices
}

fn find_matching_parenthesis(expression: &str) -> Option<usize> {
    let mut level = 0;

    for (index, c) in expression.chars().enumerate() {
        match c {
            '(' => level += 1,
            ')' => {
                level -= 1;
                if level == 0 {
                    return Some(index);
                }
            }
            _ => {}
        }
    }

    None
}
fn main() {
    let mut l = Lexer::new("123.45*(678.90 / (-2.5+ 11.5)-(80 -19) *33.25) / 20 + 11");
    println!("{}", l.parse() as f64);
}
