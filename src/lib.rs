pub mod math {
    #[macro_export]
    macro_rules! f {
        ($expr:expr) => {
            eval::math::eval($expr)
        };
        ($expr:expr;$round:expr) => {
            {
                let scale = 10_f64.powi($round as i32);
                (eval::math::eval($expr) * scale).round() / scale
            }
        };
        ($expr:expr => $($args:expr),*) => {
            eval::math::eval( eval::math::fill($expr, &[$($args),*]).as_str())
        };
        ($expr:expr => $($args:expr),*; $round:expr) => {
            {
                let scale = 10_f64.powi($round as i32);
                let result = eval::math::eval( eval::math::fill($expr, &[$($args),*]).as_str());
                (result * scale).round() / scale
            }
        };
    }
    fn replace_consts(x: &str) -> String {
        let mut x = x.to_lowercase();
        x = x.replace("pi", "3.1415926536");
        x = x.replace("p", "3.1415926536");
        x = x.replace("π", "3.1415926536");
        x = x.replace("e", "2.7182818284");
        x = x.replace("sq2", "1.41421");
        x = x.replace("sq3", "1.7320508075");
        x = x.replace("sq5", "2.23606");
        x = x.replace("gamma", "0.57721");
        x = x.replace("varphi", "1.61803");
        x = x.replace("beta", "0.28016");
        x = x.replace("lambda", "0.30366");
        x = x.replace("sigma", "0.35323");
        x = x.replace("psi", "3.35988");

        x
    }
    fn get(data: &Vec<(char, f64)>, a: char) -> Option<f64> {
        for i in data {
            if i.0 == a {
                return Some(i.1);
            }
        }
        None
    }
    pub fn fill(x: &str, values: &[f64]) -> String {
        let mut x = replace_consts(x);
        x = x.replace("cos", "|001");
        x = x.replace("sin", "|002");
        x = x.replace("tan", "|003");
        x = x.replace("acos", "|004");
        x = x.replace("asin", "|005");
        x = x.replace("atan", "|006");
        x = x.replace("cosh", "|007");
        x = x.replace("sinh", "|008");
        x = x.replace("tanh", "|009");
        x = x.replace("acosh", "|010");
        x = x.replace("asinh", "|011");
        x = x.replace("abs", "|012");
        x = x.replace("sqrt", "|013");
        let mut data: Vec<(char, f64)> = Vec::<(char, f64)>::new();
        let f: String = x.chars().filter(|x| x.is_alphabetic()).collect();
        let mut d: String = String::new();
        for i in f.chars() {
            if !d.contains(i) {
                d.push(i)
            }
        }
        drop(f);

        for i in 0..d.len() {
            data.push((d.chars().nth(i).unwrap(), values[i]));
        }

        let mut answer = x.to_string();
        let mut ln = 0usize;
        for i in x.chars() {
            if d.contains(i) {
                let content = get(&data, i);
                if content.is_some() {
                    answer.replace_range(ln..ln + 1, &content.unwrap().to_string());
                    ln += content.unwrap().to_string().len();
                }
            } else {
                ln += 1;
            }
        }
        answer = answer.replace("|001", "cos");
        answer = answer.replace("|002", "sin");
        answer = answer.replace("|003", "tan");
        answer = answer.replace("|004", "acos");
        answer = answer.replace("|005", "asin");
        answer = answer.replace("|006", "atan");
        answer = answer.replace("|007", "cosh");
        answer = answer.replace("|008", "sinh");
        answer = answer.replace("|009", "tanh");
        answer = answer.replace("|010", "acosh");
        answer = answer.replace("|011", "asinh");
        answer = answer.replace("|012", "abs");
        answer = answer.replace("|013", "sqrt");
        answer
    }
    pub fn eval(expr: &str) -> f64 {
        let mut l = Lexer::new(expr);
        l.parse()
    }
    struct Lexer {
        expression: String,
    }

    fn basic(x: &str) -> Vec<(usize, char)> {
        let mut result: Vec<(usize, char)> = Vec::new();
        let mut it = x.len();
        for i in x.chars().rev() {
            if matches!(i, '^') {
                result.push((it - 1, i));
            }
            it -= 1;
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
    fn replace_sequence(string: String) -> String {
        let mut result = String::new();
        let mut count_minus = 0;
        let mut count_plus = 0;

        for c in string.chars() {
            if c == '-' {
                count_minus += 1;
            } else if c == '+' {
                count_plus += 1;
            } else {
                if count_minus > 0 {
                    if count_minus % 2 == 0 {
                        result.push('+');
                    } else {
                        result.push('-');
                    }
                    count_minus = 0;
                }
                if count_plus > 0 {
                    result.push('+');
                    count_plus = 0;
                }
                result.push(c);
            }
        }

        if count_minus > 0 {
            if count_minus % 2 == 0 {
                result.push('+');
            } else {
                result.push('-');
            }
        }
        if count_plus > 0 {
            result.push('+');
        }

        result
    }
    fn evaluate(x: &str) -> String {
        let mut counter = 0;
        let mut x = x.to_string();
        for _ in 0..10 {
            x = replace_sequence(x);
        }
        x = x.replace("+-", "-");
        x = x.replace("-+", "-");
        for i in x.chars().rev() {
            if matches!(i, '+' | '-' | '*' | '/' | '^' | '%') {
                counter += 1;
            }
        }
        let result: String;
        match counter {
            1 => {
                if x.starts_with("-") || x.starts_with("+") {
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
                let mut left: ((usize, usize), f64) = ((0, operator.0), 0.0);
                let mut right: (usize, f64) = (operator.0, 0.0);
                left.1 = x[left.0 .0..left.0 .1].trim().parse().unwrap();
                right.1 = x[right.0 + 1..].trim().parse().unwrap();
                let result1: f64 = match operator.1 {
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
                    let left: f64;
                    let right: f64;
                    let mut left_idx = 0usize;
                    let mut right_idx = 0usize;
                    if basics.len() == 1 && basics[0].1 == '-' && basics[0].0 == 1 {
                        basics.remove(0);
                    }
                    basics = basic(&x);

                    let i = basics[0];
                    if basics.len() == 1
                        || (basics.len() == 2
                            && basics[1].1 == '-'
                            && basics[1].0 - 1 == basics[0].0)
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

                        right_idx = i.0 + 1;

                        let first_char = x.chars().nth(right_idx).unwrap();
                        if first_char == '-' || first_char == '+' {
                            right_idx += 1;
                        }
                        while right_idx < x.len()
                            && (x.chars().nth(right_idx).unwrap().is_digit(10)
                                || x.chars().nth(right_idx).unwrap() == '.')
                        {
                            right_idx += 1;
                        }
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
            let mut it = self.expression.len();
            for i in self.expression.chars() {
                if matches!(i, '^') {
                    result.push((it - 1, i));
                }
                it -= 1;
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
        fn parse(&mut self) -> f64 {
            self.expression = self
                .expression
                .chars()
                .filter(|x| !x.is_ascii_whitespace())
                .collect();
            self.expression = replace_consts(&self.expression);

            let mut trigs = find_trig_functions_indices(&self.expression);
            while trigs.len() != 0 {
                let i = trigs[0];
                let _tmp = {
                    let x = &self.expression.clone()[i.0..=i.1];
                    let idx = x.chars().position(|x| x == '(').unwrap();

                    let function = &x[0..idx];
                    let argument = &x[idx + 1..x.len() - 1];

                    let argument: f64 = eval(argument);
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
                        "sqrt" => argument.sqrt().to_string(),
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
                    &evaluate(&self.expression[pars[0].0 + 1..pars[0].1]),
                );
                pars = self.par();
            }
            drop(pars);
            let _basics: Vec<(usize, char)> = self.basic();
            self.expression = evaluate(&self.expression);
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
}
