fn get_mul_from_str(s: &str) -> u32 {
    // str mul(xxx,xxx) , len = 12
    if !s.starts_with("mul("){
        return 0;
    }
    let mut owned = s.chars().skip(4).collect::<String>();
    let first;
    let second;
    // len 8
    let mut j = 0;
    if let Ok(n) = owned[0..3].parse::<u32>(){
        j+=3;
        first = n;
    }
    else if let Ok(n) = owned[0..2].parse::<u32>(){
        j+= 2;
        first = n;
    }
    else if let Ok(n) = owned[0..1].parse::<u32>(){
        j+=1;
        first = n;
    }
    else {
        return 0;
    }
    owned = owned[j..].to_string();
    if !owned.starts_with(","){
        return 0;
    }
    owned = owned[1..].to_string();

    j = 0;
    if let Ok(n) = owned[0..3].parse::<u32>(){
        j+=3;
        second = n;
    }
    else if let Ok(n) = owned[0..2].parse::<u32>(){
        j+= 2;
        second = n;
    }
    else if let Ok(n) = owned[0..1].parse::<u32>(){
        j+=1;
        second = n;
    }
    else {
        return 0;
    }
    owned = owned[j..].to_string();
    if owned.starts_with(")"){
        return first*second;
    }
    0
}


fn main() {
    let input = include_str!("../input.txt");
    let mut total:u128 = 0;
    for i in 0..input.len()-11{
        total+=get_mul_from_str(&input[i..i+12]) as u128;
    }
    println!("{}", total);

    // part 2
    let mut total:u128 = 0;
    let mut toggle = 1;
    for i in 0..input.len()-11{
        if input[i..i+12].starts_with("do()"){
            toggle = 1;
        }
        if input[i..i+12].starts_with("don't()"){
            toggle = 0;
        }
        total+=get_mul_from_str(&input[i..i+12]) as u128 * toggle;
    }
    println!("{}", total);
}
