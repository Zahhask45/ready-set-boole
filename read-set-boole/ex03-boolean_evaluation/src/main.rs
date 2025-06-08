#[derive(Debug)]
pub enum Operator {
    Negation, // ! true now its false and vice versa
    Conjunction, // &
    Disjunction, // !
    ExclusiveDisjunction, // ^
    MaterialCondition, // >
    LogicalEquivalence, // =
}



#[derive(Debug)]
pub enum Node {
    // leaf
    Value(bool),

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
            '0' => tree.push(Value(false)),
            '1' => tree.push(Value(true)),
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
        Node::Value(val) => *val,
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
                MaterialCondition => res = left, // TODO
                LogicalEquivalence => res = left, // TODO
                Negation => panic!("Should not enter here"),
                
            }
            res
        }
    }
}

fn eval_formula(formula: &str) -> bool{
    let root = parse_formula(formula);
    evaluate(&root)
}

fn main() {
    println!("Damn: {}", eval_formula("101|&"));
    // println!("Damn: {}", eval_formula("10|"));
}
