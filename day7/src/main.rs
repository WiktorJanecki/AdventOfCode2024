use rayon::prelude::*;

#[derive(Debug, PartialEq, Clone)]
enum Token{
    Add,
    Multiply,
    Concat,
    Num(i128),
}

// turns out I'm dumb cuz you don't need to parse calculation
#[allow(dead_code)]
fn calc(input: Vec<Token>) -> i128{
    let mut input = input;
    input.push(Token::Add);

    let mut total = 0;
    let mut mult = 0;
    let mut operation = Token::Add;

    for token in input {
        match token {
            Token::Add => {
                total+=mult;
                mult = 0;
                operation = Token::Add;
            }
            Token::Multiply => {
                operation=Token::Multiply;
            }
            Token::Num(n) => {
                match operation {
                    Token::Add => {
                        mult = n;
                    }
                    Token::Multiply => {
                        mult*= n;
                    }
                    _ => {panic!("BAD INPUT")}
                }
            }
            _ => {panic!("BAD INPUT")}
        }
    }

    total
}
fn dumb_calc(input: Vec<Token>) -> i128{
    let mut total = 0_i128;
    let mut operation = Token::Add;

    for token in input {
        match token {
            Token::Add => {
                operation = Token::Add;
            }
            Token::Multiply => {
                operation=Token::Multiply;
            }
            Token::Concat => {
                operation=Token::Concat
            }
            Token::Num(n) => {
                match operation {
                    Token::Add => {
                        total += n as i128;
                    }
                    Token::Multiply => {
                        total *= n as i128;
                    }
                    Token::Concat => {
                        let mut str = "".to_owned();
                        str.push_str(&total.to_string());
                        str.push_str(&n.to_string());
                        total = str.parse::<i128>().unwrap();
                    }
                    _ => {panic!("BAD INPUT")}
                }
            }
        }
    }

    total
}
fn tokenize(input: &str) -> Vec<Token>{
    let mut creating_number = String::from("");
    let mut tokens = vec![];

    for char in input.chars() {
        if char.is_ascii_whitespace(){continue}
        if char.is_ascii_digit(){ creating_number.push(char); continue;}

        let number = creating_number.parse::<i128>().unwrap();
        tokens.push(Token::Num(number));
        creating_number.clear();

        if char == '*'{
            tokens.push(Token::Multiply);
        }
        if char == '+'{
            tokens.push(Token::Add);
        }
        if char == '|'{
            tokens.push(Token::Concat);
        }
    }
    let number = creating_number.parse::<i128>().unwrap();
    tokens.push(Token::Num(number));

    tokens
}
fn satisfies_part_one(input: &str) -> bool {
    let split: Vec<&str> = input.split(": ").collect();
    let should_result = split[0].parse::<i128>().unwrap();
    let numbers = split[1]
        .split(' ')
        .collect::<Vec<&str>>();
    // 2^(n-1) iterations at most
    // so our plus and minus array looks like binary number
    // where 0 is plus and 1 is multiply

    for binary_number in 0..2_i32.pow(numbers.len() as u32 - 1)  { // <0,2^(n-1)-1>
        let binary_form = format!("{binary_number:0>width$b}", width=numbers.len()-1);
        let replaced = binary_form.replace('0', "+").replace('1', "*");
        let mut equation = "".to_owned();
        for i in 0..numbers.len()-1 {
            equation.push_str(numbers[i]);
            equation.push(replaced.chars().nth(i).unwrap());
        }
        equation.push_str(numbers.last().unwrap());
        let tokenized = tokenize(&equation);
        if dumb_calc(tokenized) == should_result as i128{
            return true;
        }
    }
    false
}

fn satisfies_part_two(input: &str) -> bool {
    let split: Vec<&str> = input.split(": ").collect();
    let should_result = split[0].parse::<i128>().unwrap();
    let numbers = split[1]
        .split(' ')
        .collect::<Vec<&str>>();
    // 3rd operator can be space because tokenize concatenates by space already
    for numb in 0..3_i32.pow(numbers.len() as u32 - 1)  { // <0,3^(n-1)-1>
        let trinary_form = dec2tre(numb);
        let padding = format!("{trinary_form:0>width$}", width=numbers.len()-1);
        let replaced = padding.replace('0', "+").replace('1', "*").replace('2', "|");
        let mut equation = "".to_owned();
        for i in 0..numbers.len()-1 {
            equation.push_str(numbers[i]);
            equation.push(replaced.chars().nth(i).unwrap());
        }
        equation.push_str(numbers.last().unwrap());
        let tokenized = tokenize(&equation);
        if dumb_calc(tokenized) == should_result as i128{
            return true;
        }
    }
    false
}

fn dec2tre(arg: i32) -> String {
    if arg == 0 {return "0".to_string();}
    let mut n = arg;
    let mut str = String::new();
    while n != 0{
        str.push_str(&(n%3).to_string());
        n = n/3;
    }
    str.chars().rev().collect::<String>()
}

fn main() {
    let input = include_str!("../input.txt");
    let sum = input
        .lines()
        .filter(|l| satisfies_part_one(l))
        .map(|l| l.split(": ").collect::<Vec<&str>>()[0].parse::<i128>().unwrap())
        .sum::<i128>();
    println!("Part 1: {}", sum);

    let sum = input
        .par_lines()
        .filter(|l| satisfies_part_two(l))
        .map(|l| l.split(": ").collect::<Vec<&str>>()[0].parse::<i128>().unwrap())
        .sum::<i128>();
    println!("Part 2: {}", sum);
}

#[cfg(test)]
mod tests {
    use crate::{calc, satisfies_part_one, satisfies_part_two, tokenize, Token};

    const LONG_INPUT: &'static [Token] = &[
        Token::Num(1),
        Token::Multiply,
        Token::Num(2),
        Token::Add,
        Token::Num(3),
        Token::Multiply,
        Token::Num(4),
        Token::Add,
        Token::Num(5),
        Token::Add,
        Token::Num(6),
        Token::Multiply,
        Token::Num(3),
        Token::Multiply,
        Token::Num(4),
        Token::Add,
        Token::Num(1)
    ];

    #[test]
    fn part_two(){
        assert_eq!(satisfies_part_two("190: 10 19"), true);
        assert_eq!(satisfies_part_two("3267: 81 40 27"), true);
        assert_eq!(satisfies_part_two("83: 17 5"), false);
        assert_eq!(satisfies_part_two("292: 11 6 16 20"), true);
        assert_eq!(satisfies_part_two("156: 15 6"), true);
        assert_eq!(satisfies_part_two("7290: 6 8 6 15"), true);
        assert_eq!(satisfies_part_two("192: 17 8 14"), true);
        assert_eq!(satisfies_part_two("21037: 9 7 18 13"), false);
    }

    #[test]
    fn part_one(){
        assert_eq!(satisfies_part_one("190: 10 19"), true);
        assert_eq!(satisfies_part_one("3267: 81 40 27"), true);
        assert_eq!(satisfies_part_one("83: 17 5"), false);
        assert_eq!(satisfies_part_one("292: 11 6 16 20"), true);
        assert_eq!(satisfies_part_one("21037: 9 7 18 13"), false);
    }

    #[test]
    fn test_tokenize(){
        let left = tokenize("1*2+3*4+5+6*3*4+1");
        let right = LONG_INPUT.to_vec();
        assert_eq!(left, right);
    }

    #[test]
    fn test_calc_multiply(){
        let input = vec![
            Token::Num(1),
            Token::Multiply,
            Token::Num(2),
            Token::Multiply,
            Token::Num(3),
            Token::Multiply,
            Token::Num(4),
        ];
        assert_eq!(calc(input), 24);
    }

    #[test]
    fn test_calc_plus() {
        let input = vec![
            Token::Num(1),
            Token::Add,
            Token::Num(2),
            Token::Add,
            Token::Num(3),
            Token::Add,
            Token::Num(4),
        ];
        assert_eq!(calc(input), 10);
    }
    #[test]
    fn test_calc_plus_and_multiply() {
        // 1*2+3*4+5+6*3*4+1
        let input = LONG_INPUT.to_vec();
        assert_eq!(calc(input), 92);
    }
}
