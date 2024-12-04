use std::cmp::PartialEq;
use std::fs;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Token {
    Mul,
    Number,
    Comma,
    ClosingParen,
    Ignored,

    // part2
    Do,
    Dont
}

fn try_parse_mul(tokens: &[(Token, &str)]) -> Option<(u32, u32)> {
    if tokens.len() < 5 { return None; }
    let tokens = &tokens[tokens.len()-5..tokens.len()];
    if tokens[0].0 != Token::Mul || tokens[1].0 != Token::Number || tokens[2].0 != Token::Comma
        || tokens[3].0 != Token::Number || tokens[4].0 != Token::ClosingParen {
        return None;
    }
    let (_, num1_str) = tokens[1];
    let (_, num2_str) = tokens[3];
    if num1_str.len() > 3 || num2_str.len() > 3 { return None; }
    let num1: u32 = match num1_str.parse() {
        Ok(n) => n,
        Err(_) => return None
    };
    let num2: u32 = match num2_str.parse() {
        Ok(n) => n,
        Err(_) => return None
    };

    Some((num1, num2))
}

fn parse_input<F>(test_input: &str, try_parse_mul: F) -> Vec<(u32, u32)>
    where F: Fn(&[(Token, &str)]) -> Option<(u32, u32)>
{
    const MUL: &str = "mul(";
    const DO: &str = "do()";
    const DONT: &str = "don't()";
    let mut muls: Vec<(u32, u32)> = vec![];
    let mut tokens: Vec<(Token, &str)> = vec![];
    let mut offset: usize = 0;
    while offset < test_input.len() {
        let input = &test_input[offset..];
        let last_token = if tokens.len() > 0 { tokens.last().unwrap().0 } else { Token::Ignored };
        if input.starts_with(MUL) {
            tokens.push((Token::Mul, &input[0..MUL.len()]));
            offset += 4;
        } else if (last_token == Token::Mul || last_token == Token::Comma) && input.starts_with(|c| char::is_digit(c, 10)) {
            let mut len = 1;
            while input[len..].starts_with(|c| char::is_digit(c, 10)) { len += 1; }
            tokens.push((Token::Number, &input[0..len]));
            offset += len;
        } else if last_token == Token::Number && input.starts_with(',') {
            tokens.push((Token::Comma, &input[0..1]));
            offset += 1;
        } else if last_token == Token::Number && input.starts_with(')') {
            tokens.push((Token::ClosingParen, &input[0..1]));
            offset += 1;

            if let Some(numbers) = try_parse_mul(&tokens) {
                muls.push(numbers);
            }
        } else if input.starts_with(DO) {
            tokens.push((Token::Do, &input[0..DO.len()]));
            offset += DO.len();
        } else if input.starts_with(DONT) {
            tokens.push((Token::Dont, &input[0..DONT.len()]));
            offset += DONT.len();
        } else {
            tokens.push((Token::Ignored, &input[0..1]));
            offset += 1;
        }
    }
    muls
}

fn main() {
    //let test_input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    let file_input = fs::read_to_string("puzzle_input.txt")
        .expect("failed to read puzzle input");
    let test_input: &str = &file_input;

    // part 1
    {
        let muls = parse_input(test_input, try_parse_mul);
        //for (n1, n2) in &muls { println!("mul({},{})", n1, n2); }

        let mut sum = 0;
        for (n1, n2) in &muls {
            sum += n1 * n2;
        }
        println!("part 1 result: {sum}")
    }

    // part 2
    {
        let muls = parse_input(test_input, |tokens| {
            let last_do_or_dont_tok = tokens.iter()
                .map(|&(tok, _)| tok)
                .rfind(|&tok| tok == Token::Do || tok == Token::Dont)
                .unwrap_or(Token::Do);
            let do_ = last_do_or_dont_tok == Token::Do;
            if !do_ { return None; }
            try_parse_mul(tokens)
        });

        let mut sum = 0;
        for (n1, n2) in &muls {
            sum += n1 * n2;
        }
        println!("part 2 result: {sum}")
    }
}

// part1 in shell:
// grep -E -o -r 'mul\([0-9][0-9]*,[0-9][0-9]*\)' puzzle_input.txt | sed -r 's/mul\(([0-9]*),([0-9]*)\)/\1*\2/g' | paste -sd '+' | bc
