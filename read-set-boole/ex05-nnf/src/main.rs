#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operator {
    Negation, // ! true now its false and vice versa
    Conjunction, // &
    Disjunction, // |
    ExclusiveDisjunction, // ^
    MaterialCondition, // >
    LogicalEquivalence, // =
}



#[derive(Debug, Clone, PartialEq, Eq)]
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

fn equivalence(node: Node) -> Node {
    match node {
        Node::BinaryExpr { op: Operator::LogicalEquivalence, lhs, rhs } => {
            // Recursively expand children first
            let lhs = Box::new(equivalence(*lhs));
            let rhs = Box::new(equivalence(*rhs));

            // Contruct the Material condition of lhs with rhs
            let left_and = Node::BinaryExpr {
                op: Operator::MaterialCondition,
                lhs: lhs.clone(),
                rhs: rhs.clone(),
            };

            // Contruct the Material condition of rhs with lhs
            let right_and = Node::BinaryExpr {
                op: Operator::MaterialCondition,
                lhs: rhs.clone(),
                rhs: lhs.clone(),
            };

            // Return the Conjunction of the left and right conditions
            Node::BinaryExpr {
                op: Operator::Conjunction,
                lhs: Box::new(left_and),
                rhs: Box::new(right_and),
            }
        }
        Node::BinaryExpr { op, lhs, rhs } => Node::BinaryExpr {
            op,
            lhs: Box::new(equivalence(*lhs)),
            rhs: Box::new(equivalence(*rhs)),
        },
        Node::UnaryExpr { op, child } => Node::UnaryExpr {
            op,
            child: Box::new(equivalence(*child)),
        },
        other => other,
    }
}



fn material_conditon(node: Node) -> Node {
    match node {
        Node::BinaryExpr { op: Operator::MaterialCondition, lhs, rhs } => {
            // Recursively expand children first
            let lhs = Box::new(material_conditon(*lhs));
            let rhs = Box::new(material_conditon(*rhs));

            // Construct (¬lhs ∧ rhs)
            Node::BinaryExpr {
                op: Operator::Disjunction,
                lhs: Box::new(Node::UnaryExpr {
                    op: Operator::Negation,
                    child: lhs,
                }),
                rhs,
            }
        }
        Node::BinaryExpr { op, lhs, rhs } => Node::BinaryExpr {
            op,
            lhs: Box::new(material_conditon(*lhs)),
            rhs: Box::new(material_conditon(*rhs)),
        },
        Node::UnaryExpr { op, child } => Node::UnaryExpr {
            op,
            child: Box::new(material_conditon(*child)),
        },
        other => other,
    }
}


fn remove_xor(node: Node) -> Node {
    match node {
        Node::BinaryExpr { op: Operator::ExclusiveDisjunction, lhs, rhs } => {
            // Recursively expand children first
            let lhs = Box::new(remove_xor(*lhs));
            let rhs = Box::new(remove_xor(*rhs));

            // Construct (lhs ∧ ¬rhs)
            let left_and = Node::BinaryExpr {
                op: Operator::Conjunction,
                lhs: lhs.clone(),
                rhs: Box::new(Node::UnaryExpr {
                    op: Operator::Negation,
                    child: rhs.clone(),
                }),
            };

            // Construct (¬lhs ∧ rhs)
            let right_and = Node::BinaryExpr {
                op: Operator::Conjunction,
                lhs: Box::new(Node::UnaryExpr {
                    op: Operator::Negation,
                    child: lhs,
                }),
                rhs,
            };

            // Return (lhs ∧ ¬rhs) ∨ (¬lhs ∧ rhs)
            Node::BinaryExpr {
                op: Operator::Disjunction,
                lhs: Box::new(left_and),
                rhs: Box::new(right_and),
            }
        }
        Node::BinaryExpr { op, lhs, rhs } => Node::BinaryExpr {
            op,
            lhs: Box::new(remove_xor(*lhs)),
            rhs: Box::new(remove_xor(*rhs)),
        },
        Node::UnaryExpr { op, child } => Node::UnaryExpr {
            op,
            child: Box::new(remove_xor(*child)),
        },
        other => other,
    }
}


fn de_morgans_law(node: Node) -> Node{
    match node {
        Node::BinaryExpr { op, lhs, rhs } => Node::BinaryExpr {
            op,
            lhs: Box::new(de_morgans_law(*lhs)),
            rhs: Box::new(de_morgans_law(*rhs)),
        },
        Node::UnaryExpr { op: Negation, child } =>{
            match *child{
                Node::BinaryExpr {op: Operator::Conjunction, lhs, rhs} => {
                    Node::BinaryExpr {
                        op: Operator::Disjunction,
                        lhs: Box::new(Node::UnaryExpr {
                            op: Operator::Negation,
                            child: Box::new(*lhs),
                        }),
                        rhs: Box::new(Node::UnaryExpr {
                            op: Operator::Negation,
                            child: Box::new(*rhs),
                        }),
                    }
                }
                Node::BinaryExpr {op: Operator::Disjunction, lhs, rhs} => {
                    Node::BinaryExpr {
                        op: Operator::Conjunction,
                        lhs: Box::new(Node::UnaryExpr {
                            op: Operator::Negation,
                            child: Box::new(*lhs),
                        }),
                        rhs: Box::new(Node::UnaryExpr {
                            op: Operator::Negation,
                            child: Box::new(*rhs),
                        }),
                    }
                }
                other => Node::UnaryExpr {
                    op: Operator::Negation,
                    child: Box::new(other),
                }
            }
        }
        other => other,
    }
}



// last step removing the double negations
fn double_negation(node: Node) -> Node {
     match node {
        Node::UnaryExpr { op: Operator::Negation, child} => {
            match *child {
                Node::UnaryExpr {
                    op: Operator::Negation,
                    child: inner,
                } => {
                    *inner // Double negation: remove both
                }
                other => Node::UnaryExpr {
                    op: Operator::Negation,
                    child: Box::new(other),
                },
            }
        }
        Node::BinaryExpr { op, lhs, rhs } => Node::BinaryExpr {
            op,
            lhs: Box::new(double_negation(*lhs)),
            rhs: Box::new(double_negation(*rhs)),
        },
        other => other
    }
}

fn do_all(node: Node) -> Node {
    double_negation(de_morgans_law(material_conditon(equivalence(remove_xor(node)))))
}

fn ast_to_rpn(node: &Node) -> String {
    match node {
        Node::Value(val) => val.to_string(),
        Node::Bool(_c) => panic!("Should not contain '1' or '0'"),
        Node::UnaryExpr { op, child } => {
            let child_rpn = ast_to_rpn(child);
            format!("{}{}", child_rpn, operator_symbol(op))
        }
        Node::BinaryExpr { op, lhs, rhs } => {
            let lhs_rpn = ast_to_rpn(lhs);
            let rhs_rpn = ast_to_rpn(rhs);
            format!("{}{}{}", lhs_rpn, rhs_rpn, operator_symbol(op))
        }
    }
}

fn operator_symbol(op: &Operator) -> &str {
    match op {
        Operator::Negation => "!",
        Operator::Conjunction => "&",
        Operator::Disjunction => "|",
        Operator::ExclusiveDisjunction => "^",
        Operator::MaterialCondition => ">",
        Operator::LogicalEquivalence => "=",
    }
}


fn negation_normal_form(formula: &str) -> String{
    let mut original = parse_formula(formula);
    let mut root = do_all(parse_formula(formula));
    while original != root {
        original = root.clone();
        root = do_all(root);
    }
    ast_to_rpn(&root)
}



fn main() {
    println!("{}", negation_normal_form("AB&!"));
    // A!B!|
    println!("{}", negation_normal_form("AB|!"));
    // A!B!&
    println!("{}", negation_normal_form("AB>"));
    // A!B|
    println!("{}", negation_normal_form("AB="));
    // AB&A!B!&|
    println!("{}", negation_normal_form("AB|C&!"));
    // A!B!&C!|
    println!("{}", negation_normal_form("A!B!&!"));
    println!("{}", negation_normal_form("AB&C|DE&^FG|^HI&^"));
    println!("{}", negation_normal_form("AB&C|DE&!&!AB&C|DE&&|FG|!&!AB&C|DE&!&!AB&C|DE&&|FG|&|HI&!&!AB&C|DE&!&!AB&C|DE&&|FG|!&!AB&C|DE&!&!AB&C|DE&&|FG|&|HI&&|"));    
}


#[cfg(debug_assertions)]
fn print_tree(formula: &str) {
    let node = parse_formula(formula);
    println!("{node:?}");
}
