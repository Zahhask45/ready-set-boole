// use std::time::Instant;
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


fn evaluate(node: &Node) -> bool {
    match node {
        Node::Bool(val) => *val,
        Node::UnaryExpr{ op: _, child } => {
            let val = evaluate(child);
            !val
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

fn parse_formula_char(formula: &str) ->  Vec<char> {
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
    used_char
}

fn give_value_to_char(current_line: i64, formula: &str, used_char: &[char]) -> Node {
    let mut changed_formula: String = formula.to_string();
    let base: i64 = 2;
    let n = used_char.len();

    for i in 0..n {
        let pow = base.pow((n - i - 1) as u32);
        let val = (current_line / pow) % 2;
        changed_formula = changed_formula.replace(used_char[i], &val.to_string());
    }

    parse_formula(changed_formula.as_str())
}

fn sat(formula: &str) -> bool{
    let used_char = parse_formula_char(formula);
    let base = 2i64;
    let mut sati:bool = false;
    let iterations = base.pow((used_char.len()) as u32);
    for i in 0..iterations {
        let node = give_value_to_char(i, formula, &used_char);
        let val = evaluate(&node);
        if val {
            sati = true;
        }
    }
    sati
}

fn main() {
    println!("{}", sat("AB|"));
    // true
    println!("{}", sat("AB&"));
    // true
    println!("{}", sat("AA!&"));
    // false
    println!("{}", sat("AA^"));
    // false
}

#[cfg(debug_assertions)]
fn print_tree(formula: &str) {
    let node = parse_formula(formula);
    println!("{node:?}");
}
