
fn is_word_xmas(s: &str) -> bool {
    if s.len() != 4{
        return false;
    }
    s == "XMAS" || s == "SAMX"
}

fn are_coords_in_bounds(x: usize, y: usize, width: usize, height: usize) -> bool {
    x < width && y < height
}

fn is_vec_big_mas(arr: Vec<char>) -> bool{
    // len must be 9
    // 4 ways
    if  arr[0] == 'S' &&
        arr[2] == 'S' &&
        arr[4] == 'A' &&
        arr[6] == 'M' &&
        arr[8] == 'M'{
        return true;
    }
    if  arr[0] == 'M' &&
        arr[2] == 'S' &&
        arr[4] == 'A' &&
        arr[6] == 'M' &&
        arr[8] == 'S'{
        return true;
    }

    if  arr[0] == 'S' &&
        arr[2] == 'M' &&
        arr[4] == 'A' &&
        arr[6] == 'S' &&
        arr[8] == 'M'{
        return true;
    }

    if  arr[0] == 'M' &&
        arr[2] == 'M' &&
        arr[4] == 'A' &&
        arr[6] == 'S' &&
        arr[8] == 'S'{
        return true;
    }


    false
}
fn part_one(input: &str) -> u64{
    let mut sum = 0;
    // search in x-axis
    input.lines().for_each(|line|{
        for i in 0..=line.len()-4{
            let word = &line[i..i+4];
            if is_word_xmas(word){sum+=1;}
        }
    });
    // search in y-axis
    let matrix: Vec<char> = input
        .lines()
        .map(|line|{
            line.chars().collect::<Vec<_>>()
        }).flatten().collect();
    let width = input.lines().count();
    let height = input.lines().collect::<Vec<_>>()[0].chars().count();
    for i in 0..width{
        for j in 0..=height-4{
            let a = matrix[ j   *height + i];
            let b = matrix[(j+1)*height + i];
            let c = matrix[(j+2)*height + i];
            let d = matrix[(j+3)*height + i];
            let word = vec![a,b,c,d].into_iter().collect::<String>();
            if is_word_xmas(&word){sum+=1;}
        }
    }
    // search in diagonal (y=-x)
    for x in 0..width{
        for y in 0..height{
            let mut word = String::new(); // diagonal one
            let mut word_2 = String::new(); // diagonal one
            for i in 0..4{
                let x_i = x + i;
                let y_i = y + i;
                if are_coords_in_bounds(x_i, y_i, width, height) {
                    word.push(matrix[y_i*height+x_i]);
                }

                let x_i_2 = 3 + x - i;
                if are_coords_in_bounds(x_i_2, y_i, width, height) {
                    word_2.push(matrix[y_i*height+x_i_2]);
                }
            }
            if is_word_xmas(&word){
                sum += 1;
            }
            if is_word_xmas(&word_2){
                sum += 1;
            }
        }
    }
    sum
}

fn ij_char(i: usize, j: usize, lines: &Vec<&str>) -> char{
    lines[i].chars().nth(j).unwrap()
}

fn part_two(input: &str) -> u64{
    let mut sum = 0;
    let lines = input.lines().collect::<Vec<_>>();
    for i in 0..=lines.len() - 3{
        for j in 0..=lines[0].len() - 3{
            let arr = vec![
                ij_char(i,j,&lines), ij_char(i,j+1,&lines), ij_char(i,j+2,&lines),
                ij_char(i+1,j,&lines), ij_char(i+1,j+1,&lines), ij_char(i+1,j+2,&lines),
                ij_char(i+2,j,&lines), ij_char(i+2,j+1,&lines), ij_char(i+2,j+2,&lines),

            ];
            if is_vec_big_mas(arr){
                sum += 1;
            }
        }
    }
    sum
}


fn main() {
    let input = include_str!("../input.txt");
    let part_one = part_one(input);
    let part_two = part_two(input);
    println!("Part One: {}", part_one);
    println!("Part Two: {}", part_two);
}
