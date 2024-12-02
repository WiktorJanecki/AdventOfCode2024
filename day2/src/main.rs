fn is_report_safe(report: &Vec<u32>) -> bool{
    let mut increasing = false;
    let mut decreasing = false;
    for i in 0..(report.len() - 1){ // 5 reports 4 differences
        let diff =  report[i+1] as i32 -  report[i] as i32;
        if let  1..=3 = diff.abs()  {} else{
            return false;
        }
        if diff > 0{
            increasing = true;
        }
        else if diff < 0{
            decreasing = true;
        }
    }
    if increasing && decreasing{
        return false
    }
    true
}
fn is_report_safe_part_2(report: &Vec<u32>) -> bool{
    for i in 0..report.len(){
        let reduced: Vec<u32> = report
            .iter()
            .enumerate()
            .filter(|(ind, el)| *ind != i)
            .map(|(_, el)| el)
            .cloned()
            .collect();
        if is_report_safe(&reduced){
            return true;
        }
    }
    false
}

fn main() {
    let input = include_str!("../input.txt");
    let reports: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.split(" ").map(|number| number.parse::<u32>().unwrap()).collect())
        .collect();
    // part 1
    let result = reports
        .iter()
        .cloned()
        .filter(|report| is_report_safe(report))
        .count();
    println!("{}", result);
    // part 2
    let result = reports
        .iter()
        .cloned()
        .filter(|report| is_report_safe_part_2(report))
        .count();
    println!("{}", result);
}
