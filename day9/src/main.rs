#[derive(Clone, Debug)]
enum Block{
    Empty,
    Full(u32), // with id
}

#[derive(Clone, Debug)]
struct SizedBlock{
    block: Block,
    size: usize,
}

fn input_to_memory_vec(str: &str) -> Vec<Block>{
    str
        .chars()
        .collect::<Vec<char>>()
        .iter()
        .enumerate()
        .map(|(i,c)|{
            let count = c.to_digit(10).unwrap();
            let out = if i % 2 == 0 {
                Block::Full(i as u32 / 2)
            }
            else{
                Block::Empty
            };
            (0..count).map(|_|out.clone()).collect::<Vec<Block>>()
        })
        .flatten().collect::<Vec<Block>>()
}

fn input_to_sized_blocks(str: &str) -> Vec<SizedBlock>{
    str
        .chars()
        .collect::<Vec<char>>()
        .iter()
        .enumerate()
        .map(|(i,c)|{
            let count = c.to_digit(10).unwrap();
            let out = if i % 2 == 0 {
                Block::Full(i as u32 / 2)
            }
            else{
                Block::Empty
            };
            SizedBlock{
                block: out,size: count as usize
            }
        })
        .collect::<Vec<SizedBlock>>()
}

fn sized_blocks_to_memory_vec(blocks: &Vec<SizedBlock>) -> Vec<Block>{
    blocks
        .iter()
        .filter(|b| b.size != 0)
        .map(| SizedBlock{ block, size }|{
            (0..*size).map(|_|block.clone()).collect::<Vec<Block>>()
        })
        .flatten()
        .collect::<Vec<Block>>()
}

fn checksum(memory: &Vec<Block>) -> u128 {
    memory
        .iter()
        .enumerate()
        .map(|(i,v)|{
        let id = match *v{
            Block::Empty => 0,
            Block::Full(id) => id
        };
        i as u128 * id as u128
    }).sum()
}

fn apply_part_one(mut memory: Vec<Block>) -> Vec<Block>{
    let mut i = 0_usize;
    let mut j = memory.len() - 1;
    while i<j{
        while matches!(memory[j],Block::Empty){
            j-=1;
        }
        if i>=j{break}
        if matches!(memory[i],Block::Empty) {
            memory[i] = memory[j].clone();
            memory[j] = Block::Empty;
        }
        i+=1;
    }
    memory
}

fn apply_part_two(mut memory: Vec<SizedBlock>) -> Vec<SizedBlock>{
    let mut j = memory.len()-1;
    while j>1 {
        while matches!(memory[j].block,Block::Empty) {
            j -= 1;
        }
        let mut d = 0;
        while d < j {
            if matches!(memory[d].block,Block::Empty) {
                let diff = memory[d].size as isize -memory[j].size as isize;
                if diff >= 0{
                    let helper = memory[j].clone();
                    memory[j] = SizedBlock{block: Block::Empty, size: helper.size};
                    let fragment = vec![helper,SizedBlock{block: Block::Empty,size:diff as usize}];
                    let new_mem = [&memory[0..d],&fragment,&memory[d+1..]].concat();
                    memory = new_mem;
                    j=memory.len()-1;
                    d=0;
                    break;
                }
            }
            d+=1;
        }
        j-=1;
    }
    memory
}

fn main() {
    let input = include_str!("../input.txt");
    let memory = input_to_memory_vec(input);
    let part_one = apply_part_one(memory);
    let result = checksum(&part_one);
    println!("PART ONE: {}", result);

    let blocked_memory = input_to_sized_blocks(input);
    let part_two = apply_part_two(blocked_memory);
    let result = checksum(&sized_blocks_to_memory_vec(&part_two));
    println!("PART TWO: {:?}", result);
}
fn memory_vec_to_str(memory: Vec<Block>) -> String {
    let mut str = String::new();
    for block in memory {
        match block {
            Block::Empty => {
                str.push('.');
            }
            Block::Full(id) => {
                str.push_str(id.to_string().as_str());
            }
        }
    }
    str
}
fn sized_vec_to_str(input: Vec<SizedBlock>) -> String{
    memory_vec_to_str(sized_blocks_to_memory_vec(&input))
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_to_memory_vec() {
        let input = "2333133121414131402";
        let memory = input_to_memory_vec(input);
        let memory_representation = memory_vec_to_str(memory);
        assert_eq!(memory_representation, "00...111...2...333.44.5555.6666.777.888899");
    }

    #[test]
    fn test_checksum() {
        let input = "0099811188827773336446555566..............".chars().map(|c| {
            if c.is_ascii_digit(){
                let id = c.to_digit(10).unwrap();
                Block::Full(id)
            }
            else{
                Block::Empty
            }
        }).collect::<Vec<Block>>();
        assert_eq!(checksum(&input),1928)
    }

    #[test]
    fn test_apply_part_one() {
        let input = "2333133121414131402";
        let memory = input_to_memory_vec(input);
        let part_one = apply_part_one(memory);
        assert_eq!(memory_vec_to_str(part_one),"0099811188827773336446555566..............");
    }
    #[test]
    fn test_apply_part_two() {
        let input = "2333133121414131402";
        let memory = input_to_sized_blocks(input);
        //00...111...2...333.44.5555.6666.777.888899
        let part_one = apply_part_two(memory);
        let normalized = sized_blocks_to_memory_vec(&part_one);
        //0099....2...333.44.5555.6666.777.8888
        assert_eq!(memory_vec_to_str(normalized),"00992111777.44.333....5555.6666.....8888..");
    }
}
