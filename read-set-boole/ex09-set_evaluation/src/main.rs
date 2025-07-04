use std::collections::HashSet;


#[derive(Debug, Clone)]
pub struct Sets {
    set: HashSet<i32>,
    name: char,
}

#[derive(Debug)]
pub enum Operator {
    Negation, // ! true now its false and vice versa
    Conjunction, // &
    Disjunction, // |
    ExclusiveDisjunction, // ^
    MaterialCondition, // >
    LogicalEquivalence, // =
}



#[derive(Debug)]
pub enum Node {
    // leaf
    Value(char),
    Bool(bool),
    Set(Sets),

    // Branches
    UnaryExpr {
        op: Operator,
        child: Box<Node>,
    },
    BinaryExpr {
        op: Operator,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
}

use Operator::*;
use Node::*;

fn parse_formula(formula: &str, all_sets: &Vec<Sets>) -> Node{
    let mut tree: Vec<Node> = Vec::new();

    for c in formula.chars() {
        match c {
            'A'..='Z' => {
                for iter in all_sets {
                    if c == iter.name {
                        tree.push(Set(iter.clone()));
                    }
                }
            }
            '!' =>{
                let child = Box::new(tree.pop().expect("Missing value for !"));
                tree.push(UnaryExpr {op: Negation, child });
            }
            '&' => {
                let rhs = Box::new(tree.pop().expect("Missing rhs value for &"));
                let lhs = Box::new(tree.pop().expect("Missing lhs value for &"));
                tree.push(BinaryExpr {op: Conjunction, lhs, rhs});
            }
            '|' => {
                let rhs = Box::new(tree.pop().expect("Missing rhs value for &"));
                let lhs = Box::new(tree.pop().expect("Missing lhs value for &"));
                tree.push(BinaryExpr {op: Disjunction, lhs, rhs});
            }
            '^' => {
                let rhs = Box::new(tree.pop().expect("Missing rhs value for ^"));
                let lhs = Box::new(tree.pop().expect("Missing lhs value for ^"));
                tree.push(BinaryExpr {op: ExclusiveDisjunction, lhs, rhs}); 
            }
            '>' => {
                let rhs = Box::new(tree.pop().expect("Missing rhs value for >"));
                let lhs = Box::new(tree.pop().expect("Missing lhs value for >"));
                tree.push(BinaryExpr {op: MaterialCondition, lhs, rhs});
            }
            '=' => {
                let rhs = Box::new(tree.pop().expect("Missing rhs value for ="));
                let lhs = Box::new(tree.pop().expect("Missing lhs value for ="));
                tree.push(BinaryExpr {op: LogicalEquivalence, lhs, rhs});
            }
            _ => panic!("Invalid char in the formula"),
        }
    }
    assert!(tree.len() == 1, "Invalid postfix expression");
    tree.pop().unwrap()
}


fn parse_formula_char(formula: &str, sets: &Vec<Vec<i32>>) ->  Vec<Sets> {
    let mut all_sets: Vec<Sets> = Vec::new();
    let mut used_char: Vec<char> = Vec::new();
    let mut position: usize = 0;
    

    for c in formula.chars() {
        match c {
            'A'..='Z' => {
                if !used_char.iter().any(|&val| val == c) {
                    used_char.push(c);
                    let hash = HashSet::from_iter(sets[position].clone());
                    all_sets.push(Sets {set: hash, name: c});
                    position += 1;
                }
            }
            '!' =>{
                continue;
            }
            '&' => {
                continue;
            }
            '|' => {
                continue;
            }
            '^' => {
                continue; 
            }
            '>' => {
                continue;
            }
            '=' => {
                continue;
            }
            _ => panic!("Invalid char in the formula"),
        }
    }
    all_sets
}
fn diff(left: HashSet<i32>, right: HashSet<i32>) -> HashSet<i32>{
    right.difference(&left).cloned().collect()
}

fn conjunction(left: HashSet<i32>, right: HashSet<i32>) -> HashSet<i32>{
    left.intersection(&right).cloned().collect()
}

fn exclusivedisjunction(left: HashSet<i32>, right: HashSet<i32>) -> HashSet<i32>{
    left.symmetric_difference(&right).cloned().collect()
}

fn disjunction(left: HashSet<i32>, right: HashSet<i32>) -> HashSet<i32>{
    left.union(&right).cloned().collect()
}

fn evaluate(node: &Node, universe: HashSet<i32>) -> HashSet<i32> {
    match node {
        Node::Set(val) => val.set.clone(),
        Node::UnaryExpr{ op: _, child } => {
            let val = evaluate(child, universe.clone());
            // return the one that are not in the set
            diff(val, universe.clone())
        }
        Node::BinaryExpr{ op, lhs, rhs} => {
            let left = evaluate(lhs, universe.clone());
            let right = evaluate(rhs, universe.clone());
            let res: HashSet<i32>;
            match op {
                Conjunction =>{
                    res = conjunction(left, right);
                }
                Disjunction => {
                    res = disjunction(left, right);
                }
                ExclusiveDisjunction => {
                    res = exclusivedisjunction(left, right);
                }
                MaterialCondition => {
                    res = disjunction(diff(left, universe.clone()), right);
                }
                LogicalEquivalence => {
                    res = diff(exclusivedisjunction(left, right), universe);
                }
                Negation => panic!("Should not enter here"),
                
            }
            res
        }
        Node::Value(_val) => panic!("There should not be any char at this momment"),
        Node::Bool(_val) => panic!("There should not be any Bool at this momment"),
    }
}

fn eval_set(formula: &str, sets: Vec<Vec<i32>>)-> Vec<i32>{
    let all_sets = parse_formula_char(formula, &sets);
    // println!("{all_sets:?}");
    let ast = parse_formula(formula, &all_sets);
    // println!("{ast:?}");
    let universe = create_universe(sets);
    let hash_value = evaluate(&ast, universe);
    // println!("Damn: {hash_value:?}");
    convert_hash_vec(hash_value) // returns a vec<i32>
    
}

fn main() {
    let sets: Vec<Vec<i32>> = vec![vec![0, 1, 2],vec![0, 3, 4]];
    println!("{:?}", eval_set("AB&", sets));
    let sets = vec![vec![0, 1, 2],vec![3, 4, 5]];
    println!("{:?}", eval_set("AB|", sets));
    let sets = vec![vec![0, 1, 2],vec![9]];
    println!("{:?}", eval_set("A!", sets));
}

fn convert_hash_vec(hash: HashSet<i32>) -> Vec<i32>{
    let mut final_vec = <HashSet<i32> as IntoIterator>::into_iter(HashSet::from_iter(hash.iter().copied())).collect::<Vec<i32>>();
    final_vec.sort();
    final_vec
}

fn create_universe(all_sets: Vec<Vec<i32>>) -> HashSet<i32> {
    let mut universe: HashSet<i32> = HashSet::new();
    for iter in all_sets{
        let hash = HashSet::from_iter(iter.clone());
        universe = disjunction(universe, hash);
    }
    universe
}
