struct Tree{
    // head
    number: u32,
    // numbers allowed to be only before number
    lesser: Vec<u32>,
}

impl Tree{
    fn new(n: u32) -> Tree{Tree{number: n, lesser: vec![]}}
}

fn are_pages_right(rev_page: &str, tree: &Vec<Tree>) -> bool {
    let mut vec: Vec<_> = rev_page.split(",").collect();
    vec.reverse();
    for i in 0..vec.len(){
        let number = vec[i].parse::<u32>().unwrap();
        for j in i+1..vec.len(){
            let number_j = vec[j].parse::<u32>().unwrap();
            if tree.iter().find(|&x| x.number == number_j).is_some_and(|e|e.lesser.contains(&number)) {
                return false;
            }
        }
    }

    true
}

fn order_page_right(rev_page: &str, tree: &Vec<Tree>) -> Vec<String> {
    let mut vec: Vec<_> = rev_page.split(",").collect();
    vec.reverse();
    for i in 0..vec.len(){
        let mut number = vec[i].parse::<u32>().unwrap();
        let mut j = i+1;
        while j < vec.len(){
            let number_j = vec[j].parse::<u32>().unwrap();
            if tree.iter().find(|&x| x.number == number_j).is_some_and(|e|e.lesser.contains(&number)) {
                // number (ity element) musi być po j-tym elemencie trzeba go insertowac w j+1 miejsce
                // potem usunac ity element
                // ustawic number na ity element
                // ustawic j na 0

                vec.insert(j+1,vec[i]);
                vec.remove(i);
                number = vec[i].parse::<u32>().unwrap();
                j = i;
            }
            j+=1;
        }
    }

    vec.reverse();
    vec.iter().map(|x| x.to_string()).collect()
}

fn main() {
    // parse input into vec of rules and vec of pages

    let input = include_str!("../input.txt");
    let lines: Vec<_> = input.lines().collect();
    let index_to_split = input.lines().position(|l| l.is_empty()).expect("bad input");
    let (rules, pages) = lines.split_at(index_to_split);

    // parse rules into trees
    let mut forest: Vec<Tree> = vec![];
    for rule in rules{
        let [a,b] = rule
            .split("|")
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let tree_b = if let Some(tree) = forest.iter_mut().find(|e| e.number==b) {
            tree
        } else {
            forest.push(Tree::new(b));
            forest.last_mut().unwrap()
        };
        tree_b.lesser.push(a);
    }
    let part_one = pages
        .iter()
        .skip(1)
        .filter(|&&el| are_pages_right(el, &forest))
        .map(|&line|{
            let splitted = line.split(",").collect::<Vec<_>>();
            let ind = splitted.len() / 2;
            splitted[ind].parse::<u32>().unwrap()
        })
        .sum::<u32>();
    println!("Part 1: {}", part_one); // 5329

    // PART 2

    let inordered_lines = pages
        .iter()
        .skip(1)
        .filter(|&&el| !are_pages_right(el, &forest))
        .collect::<Vec<_>>();
    let part_two = inordered_lines
        .iter()
        .map(|&line|{order_page_right(line,&forest)})
        .map(|splitted|{
            let ind = splitted.len() / 2;
            splitted[ind].parse::<u32>().unwrap()
        })
        .sum::<u32>();
    println!("Part 2: {}", part_two);
}