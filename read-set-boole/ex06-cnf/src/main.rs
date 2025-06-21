#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operator {
    Negation, // !q
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

#[derive(Debug, Clone, PartialEq, Eq)]
struct Kmapzero {
    row: usize,
    col: usize,
    grouped: bool,
}

use Operator::*;
use Node::*;


//====================================== PARSERS ========================================
//=======================================================================================

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

//=======================================================================================


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

fn distributivity(node: Node) -> Node{
    match node {
        Node::BinaryExpr {op: Operator::Disjunction, lhs, rhs} => {
            if let Node::BinaryExpr { op: Operator::Conjunction, lhs: left, rhs: right} = *lhs {
               // Recursively expand children first
               let lhs = Box::new(*left);
               let rhs = Box::new(*right);
   
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
            } else if let Node::BinaryExpr { op: Operator::Conjunction, lhs: left, rhs: right} = *rhs{
                // Recursively expand children first
                let lhs = Box::new(equivalence(*left));
                let rhs = Box::new(equivalence(*right));

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
            } else{
                Node::BinaryExpr {
                    op: Disjunction,
                    lhs: Box::new(distributivity(*lhs)),
                    rhs: Box::new(distributivity(*rhs)),
                }
            }
        }
        Node::UnaryExpr { op, child } => Node::UnaryExpr {
            op,
            child: Box::new(distributivity(*child)),
        },
        Node::BinaryExpr {op, lhs, rhs} => Node::BinaryExpr {
            op,
            lhs: Box::new(distributivity(*lhs)),
            rhs: Box::new(distributivity(*rhs)),
        },
        other => other,
    }
}

fn do_all(node: Node) -> Node {
    distributivity(double_negation(de_morgans_law(material_conditon(equivalence(remove_xor(node))))))
}


fn conjunctive_normal_form(formula: &str) -> String{
    karnaugh_map(formula);
    let mut original = parse_formula(formula);
    let mut root = do_all(parse_formula(formula));
    while original != root {
        original = root.clone();
        root = do_all(root);
    }
    ast_to_rpn(&root)
}



fn main() {
    // println!("{}", conjunctive_normal_form("AB&!"));
    // // A!B!|
    // println!("{}", conjunctive_normal_form("AB|!"));
    // // A!B!&
    // println!("{}", conjunctive_normal_form("AB|C&"));
    // // AB|C&
    // println!("{}", conjunctive_normal_form("AB|C|D|"));
    // ABCD|||
    // println!("{}", conjunctive_normal_form("AB&C&D&"));
    // println!("{}", conjunctive_normal_form("A!B&C&D&B&"));
    println!("{}", conjunctive_normal_form("BCD!A!&&&"));
    // ABCD&&&
    // println!("{}", conjunctive_normal_form("AB&!C!|"));
    // // A!B!C!||
    // println!("{}", conjunctive_normal_form("AB|!C!&"));
    // // A!B!C!&&

}







//========== PARSE THE ASYMETRIC SYNTAX TREE(AST) TO REVERSE POLISH NOTATION(RPN) =======

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

//=======================================================================================



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

fn give_value_to_char(current_line: i64, formula: &str, used_char: &[char]) -> Node {
    let mut changed_formula: String = formula.to_string();
    let base: i64 = 2;
    let n = used_char.len();

    for i in 0..n {
        let pow = base.pow((n - i - 1) as u32);
        let val = (current_line / pow) % 2;
        changed_formula = changed_formula.replace(used_char[i], &val.to_string());
    }

    parse_formula_binary(changed_formula.as_str())
}



fn gray_code(n: u8) -> usize{
    (n ^ (n >> 1)) as usize
}



// 
fn grouping(kmap: [[bool;4];4]) -> Vec<Kmapzero>{
    let mut zero_cells:Vec<Kmapzero> = Vec::new();
    for row in 0..4{
        for col in 0..4{
            if !kmap[row][col] {
                zero_cells.push(Kmapzero {row, col, grouped: false});
            }
        }
    }
    println!("Zero_cells: {:?}", zero_cells);
    // check first if group of 16, close if not check all 8, then if missing 4 then 2 and then 1
    zero_cells
}


// This will make the Karnaugh map and then I just need to sum the groups where there are 1(true) so we know that the others are 0(false)
fn karnaugh_map(formula: &str) {
    let mut kmap = [[false; 4]; 4];
    
    let mut used_char = parse_formula_char(formula);
    used_char.sort();
    // need to create conditin for when the used_char as more than 4 variables to exit (maybe put that in the main)
    let base = 2i64;
    let iterations = base.pow((used_char.len()) as u32);
    for i in 0..iterations {
        let mut changed_formula = formula.to_string();
        for j in 0..used_char.len() {
            let bit = (used_char.len() - 1) - j;
            let a = (i >> bit) & 1 == 1;
            changed_formula = changed_formula.replace(used_char[j], &(a as i8).to_string());
        }
        
        
        let row = gray_code((i / 4) as u8);
        let col = gray_code((i % 4) as u8);
        println!("{row}                 {col}");

        
        // let node = give_value_to_char(i, formula, &used_char);
        let val = evaluate(&parse_formula_binary(&changed_formula));
        kmap[row][col] = val;
    }
    print_kmap(kmap);
    grouping(kmap);
}



fn parse_formula_binary(formula: &str) -> Node{
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


#[cfg(debug_assertions)]
fn print_tree(formula: &str) {
    let node = parse_formula(formula);
    println!("{node:?}");
}


#[cfg(debug_assertions)]
fn print_kmap(kmap: [[bool;4];4]) {
    for i in 0..4{
        for j in 0..4{
            print!("[{}]", kmap[i][j] as i8);
        }
        println!();
    }
}


