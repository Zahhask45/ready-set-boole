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
            'A'..='Z' => {
                tree.push(Value(c));
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

// fn material_condition(formula: &str) -> String {
//     let 
// }

// last step removing the double negations
fn double_negation(formula: &str) -> String {
    let s = formula.replace("!!", "");
    s.to_owned()
}


fn negation_normal_form(formula: &str) -> String{
    let root = parse_formula(formula);
    // println!("{root:?}");
    // new_formula
}


fn main() {
    println!("AB&C|DE&^");
    // println!("AB&C|DE&^FG|^HI&^");
    println!("{}", negation_normal_form("AB&C|D!E!|&A!B!|C!&DE&&|"));
    // println!("{}", negation_normal_form("AB&C|DE&!&!AB&C|DE&&|FG|!&!AB&C|DE&!&!AB&C|DE&&|FG|&|HI&!&!AB&C|DE&!&!AB&C|DE&&|FG|!&!AB&C|DE&!&!AB&C|DE&&|FG|&|HI&&|"));
}
