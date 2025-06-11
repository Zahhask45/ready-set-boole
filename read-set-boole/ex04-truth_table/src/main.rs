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

fn parse_formula(formula: &str) -> Node{
    let mut tree: Vec<Node> = Vec::new();

    for c in formula.chars() {
        match c {
            '0' => tree.push(Bool(false)),
            '1' => tree.push(Bool(true)),
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

fn parse_formula_char(formula: &str) -> (Node, Vec<char>) {
    let mut tree: Vec<Node> = Vec::new();
    let mut used_char: Vec<char> = Vec::new();

    for c in formula.chars() {
        match c {
            'A'..='Z' => {
                tree.push(Value(c));
                if !used_char.iter().any(|&val| val == c) {
                    used_char.push(c);
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
    (tree.pop().unwrap(), used_char)
}

fn evaluate(node: &Node) -> bool {
    match node {
        Node::Bool(val) => {
            println!("{val}");
            false
        }
        Node::UnaryExpr{ op, child } => {
            let val = evaluate(child);
            let res = !val;
            res
        }
        Node::BinaryExpr{ op, lhs, rhs} => {
            let left = evaluate(lhs);
            let right = evaluate(rhs);
            let res: bool;
            match op {
                Conjunction => res = left & right,
                Disjunction => res = left | right,
                ExclusiveDisjunction => res = left ^ right,
                MaterialCondition => res = !left | right,
                LogicalEquivalence => res = !(left ^ right),
                Negation => panic!("Should not enter here"),
                
            }
            res
        }
        Node::Value(_val) => panic!("There should not be any char at this momment"),
    }
}

fn give_value_to_char(current_line: i64, formula: &str, used_char: &Vec<char>) -> Node{
    let mut changed_formula: String = formula.to_string(); // fun fact .to_string() calls String::from and String::from calls .to_owned()
    let base: i64 = 2;
    let iterations = base.pow((used_char.len()) as u32) as f64;
    for i in 0..used_char.len(){
        
        if iterations * (1.00 / base.pow((i + 1) as u32) as f64) < current_line as f64{
            // println!("{}",i);
            changed_formula = changed_formula.replace(used_char[i], "0");
        } else {
            changed_formula = changed_formula.replace(used_char[i], "1");
        }
    }
    println!("{changed_formula}");
    parse_formula(changed_formula.as_str())
}

fn print_and_resolve(used_char: &Vec<char>, formula: &str) {
    for val in used_char {
        print!("| {val} ");
        
    }
    println!("| = |");
    let base = 2i64;
    // index in the Vec<char>
    // 1 -> 2 -> 3 -> 4 ->
    // 1/2 -> 1/4 -> 1/8 -> 1/16
    let iterations = base.pow((used_char.len()) as u32);
    println!("{iterations}");
    for i in 1..=iterations {
        let node = give_value_to_char(i, formula, used_char);
        // if i % 2 == 1 {
        //     println!("false");
        // } else {
        //     println!("true");
        // }
    }
    
}


fn print_truth_table(formula: &str){
    let (root, used_char) = parse_formula_char(formula);
    print_and_resolve(&used_char, formula);
}



// TRUTH TABLE GOES LIKE 2/4/8/16/32/..
// conditions to know if true or false:
// i starts at 1 and ends in 32 for example -> i % 2 -> if 1 false if 2 true
fn main() {
    print_truth_table("AC&B|");
}
