#[derive(Debug, Clone)]
struct PowerSet {
    value: i32,
    position: u32,
}

fn parse(set: &Vec<i32>) -> Vec<PowerSet>{
    let mut power: Vec<PowerSet> = Vec::new();
    let mut i = 0u32;
    let base = 2i32;

    for iter in set {
        power.push(PowerSet { value: *iter, position: base.pow(i) as u32});
        i += 1;
    }
    power
}

fn powerset(set: Vec<i32>) -> Vec<Vec<i32>>{
    let amount = set.len();
    let mut position = 0u32;
    let power = parse(&set);
    let base = 2u32;
    let iterations = base.pow(amount as u32);
    let mut final_set: Vec<Vec<i32>> = vec![];

    // need to parse the set to give them new values for their position
    // the integers in the set will be the iterator 1,2,4,8,16,32
    for i in 0..iterations {
        let mut new_set: Vec<i32> = vec![];
        if i == 0 { final_set.push(new_set.clone()); continue;}
        if i == base.pow(position) {
            new_set.push(set[position as usize]);
            final_set.push(new_set.clone());
            position += 1;
            continue;
        }
        // put together various values in a set
        for iter in power.clone() {
            
            if i & iter.position != 0{
                new_set.push(iter.value);
            }
        }
        final_set.push(new_set.clone());
    }
    final_set
}

fn main() {
    let set = vec![1, 2, 3];
    println!("{:?}", powerset(set));
    let set = vec![1, 2, 3, 4];
    println!("{:?}", powerset(set));
    let set = vec![1, 2, 3, 4, 5];
    println!("{:?}", powerset(set));
}
