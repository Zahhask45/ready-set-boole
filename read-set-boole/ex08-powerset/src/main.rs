#[derive(Debug)]
struct PowerSet {
    value: i32,
    position: i32,
}

fn parse(set: Vec<i32>) -> Vec<PowerSet>{
    let mut power: Vec<PowerSet> = Vec::new();
    let mut i = 0u32;
    let base = 2i32;

    for iter in set {
        power.push(PowerSet { value: iter, position: base.pow(i)});
        i += 1;
    }
    power
}

fn powerset(set: Vec<i32>){
    let amount = set.len();
    let mut position = 0u32;
    // let power = parse(set);
    let base = 2u32;
    let iterations = base.pow(amount as u32);
    let mut final_set: Vec<Vec<i32>> = vec![];

    // need to parse the set to give them new values for their position
    // the integers in the set will be the iterator 1,2,4,8,16,32
    for i in 0..iterations {
        let mut new_set: Vec<i32> = vec![];
        if i == 0 { final_set.push(new_set.clone()); continue;}
        println!("{final_set:?}");
        if i == base.pow(position) {
            new_set.push(set[position as usize]);
            final_set.push(new_set.clone());
            position += 1;
            continue;
        }
        // put together various values in a set
        
        
        
    }
}

fn main() {
    let set = vec![1, 2, 3];
    powerset(set);
}
