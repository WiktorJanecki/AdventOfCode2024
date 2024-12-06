use std::ops::Deref;

#[derive(PartialEq, Clone, Debug)]
enum Direction{
    Left,
    Right,
    Up,
    Down
}

fn get_char_from_map(x: i32, y: i32, map: &Vec<Vec<char>>) -> Option<char>{
    if x < 0 || y < 0 || y as usize >= map.len() || x as usize >= map[0].len(){
        return None;
    }
    Some(map[y as usize][x as usize])
}

fn main() {
    let input = include_str!("../input.txt");
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();
    let mut player_pos: Option<(usize, usize)> = None;
    let mut path_matrix: Vec<Vec<u32>> = vec![];
    let mut dir_matrix: Vec<Vec<Option<Direction>>> = vec![];
    let mut map_matrix: Vec<Vec<char>> = vec![];

    for (y, line) in input.lines().enumerate() {
        path_matrix.push(vec![]);
        map_matrix.push(vec![]);
        dir_matrix.push(vec![]);
        for (x, ch) in line.chars().enumerate() {
            path_matrix[y].push(0);
            map_matrix[y].push(ch);
            dir_matrix[y].push(None);
            if ch == '^' {
                player_pos = Some((x,y));
            }
        }
    }
    let mut rotation = Direction::Up;
    let (a,b) = player_pos.expect("bad input");
    let x = a as i32;
    let y = b as i32;

    // simulation
    part_one(&mut path_matrix, &mut map_matrix, &mut rotation, x, y);

    let part_one = path_matrix.iter().flatten().sum::<u32>() + 1; // that one from last move
    println!("Part 1: {}", part_one);

    let mut total = 0;
    for yy in 0..height {
        for xx in 0..width {
            let mut new_map_matrix = map_matrix.clone();
            new_map_matrix[yy][xx] = '#';
            let mut new_path_matrix = dir_matrix.clone();
            let mut new_rotation = Direction::Up;
            if will_loop(&mut new_path_matrix,&mut new_map_matrix,&mut new_rotation,x,y ){
                total += 1;
            }
        }
    }
    println!("Part 2: {}", total);

}

fn part_one(path_matrix: &mut Vec<Vec<u32>>, map_matrix: &mut Vec<Vec<char>>, rotation: &mut Direction, mut x: i32, mut y: i32) {
    let mut char = get_char_from_map(x, y, &map_matrix);
    while char.is_some() {
        path_matrix[y as usize][x as usize] = 1;
        match char.unwrap() {
            '.' => {
                match rotation {
                    Direction::Up => {
                        y -= 1;
                    },
                    Direction::Right => { x += 1 },
                    Direction::Down => { y += 1 },
                    Direction::Left => { x -= 1 },
                }
            },
            '#' => {
                *rotation = match rotation {
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down,
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                };
            },
            '^' => {
                map_matrix[y as usize][x as usize] = '.';
            },
            _ => panic!("bad input")
        }
        char = match rotation {
            Direction::Left => get_char_from_map(x - 1, y, &map_matrix),
            Direction::Right => get_char_from_map(x + 1, y, &map_matrix),
            Direction::Up => get_char_from_map(x, y - 1, &map_matrix),
            Direction::Down => get_char_from_map(x, y + 1, &map_matrix),
        };
    }
}

fn will_loop(path_matrix: &mut Vec<Vec<Option<Direction>>>, map_matrix: &mut Vec<Vec<char>>, rotation: &mut Direction, mut x: i32, mut y: i32) -> bool {
    let mut char = get_char_from_map(x, y, &map_matrix);
    while char.is_some() {
        let original_direction = path_matrix[y as usize][x as usize].clone();
        if let Some(org) = original_direction.clone() {
            if org == rotation.clone(){
                return true;
            }
        }
        let orig_x = x;
        let orig_y = y;
        match char.unwrap() {
            '.' => {
                match rotation {
                    Direction::Up => {
                        y -= 1;
                    },
                    Direction::Right => { x += 1 },
                    Direction::Down => { y += 1 },
                    Direction::Left => { x -= 1 },
                }
                path_matrix[orig_y as usize][orig_x as usize] = Some(rotation.clone());
            },
            '#' => {
                *rotation = match rotation {
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down,
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                };
            },
            '^' => {
                map_matrix[y as usize][x as usize] = '.';
            },
            _ => panic!("bad input")
        }
        char = match rotation {
            Direction::Left => get_char_from_map(x - 1, y, &map_matrix),
            Direction::Right => get_char_from_map(x + 1, y, &map_matrix),
            Direction::Up => get_char_from_map(x, y - 1, &map_matrix),
            Direction::Down => get_char_from_map(x, y + 1, &map_matrix),
        };
    }
    false
}


