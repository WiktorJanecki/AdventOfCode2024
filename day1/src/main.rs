use std::iter::zip;

fn part_one(){
    let input = include_str!("../input.txt").to_string();
    let mut first_parsed = vec![];
    let mut second_parsed = vec![];
    for line in input.lines() {
        let (first,second) = line.split_once("   ").unwrap();;
        first_parsed.push(first.parse::<i32>().unwrap());
        second_parsed.push(second.parse::<i32>().unwrap());
    }
    first_parsed.sort();
    second_parsed.sort();
    let mut total = 0;
    for (x,y) in zip(first_parsed,second_parsed){
        total+=(x-y).abs();
    }
    println!("{}",total);
}

fn part_two(){
    let input = include_str!("../input.txt").to_string();
    let mut first_parsed = vec![];
    let mut second_parsed = vec![];
    for line in input.lines() {
        let (first,second) = line.split_once("   ").unwrap();;
        first_parsed.push(first.parse::<i32>().unwrap());
        second_parsed.push(second.parse::<i32>().unwrap());
    }
    first_parsed.sort();
    second_parsed.sort();
    let mut total = 0;
    for number in first_parsed{
        let mut count = 0;
        for second in &second_parsed{
            if *second < number{
                continue;
            }
            else if *second == number{
                count += 1;
            }
            else{
                break;
            }
        }
        total += count*number;
    }

    println!("{}",total);
}

fn main() {
    part_two()
}
