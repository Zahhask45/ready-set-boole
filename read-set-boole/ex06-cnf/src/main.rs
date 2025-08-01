use std::collections::HashSet;

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
pub enum Groups {
    FourTwo, // 4 by 2
    FourOne, // 4 by 1
    TwoTwo, // 2 by 2
    TwoOne, // 2 by 1
    OneOne, // 1 by 1
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
    form: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct KmapInfo {
    rows: usize,
    cols: usize,
    group_type: Groups,
    amount: usize,
}

pub const FOUR_TWO: usize = 8; 
pub const FOUR_ONE: usize = 4; 
pub const TWO_TWO: usize = 4; 
pub const TWO_ONE: usize = 2; 


use Operator::*;
use Node::*;
use Groups::*;


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

#[allow(dead_code)]
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
    // let mut used_char = parse_formula_char(formula);
    // if used_char.len() >= 2 && used_char.len() <= 4 {
    //     if used_char.len() == 4 {
    //         println!("K-MAP: {}", karnaugh_map4(formula, &mut used_char));
    //     }
    //     if used_char.len() == 3 {
    //         println!("K-MAP: {}", karnaugh_map3(formula, &mut used_char));
    //     }
    //     if used_char.len() == 2 {
    //         println!("K-MAP: {}", karnaugh_map2(formula, &mut used_char));
    //     }
    //     
    // }
    let mut original = parse_formula(formula);
    let mut root = do_all(parse_formula(formula));
    while original != root {
        original = root.clone();
        root = do_all(root);
    }
    ast_to_rpn(&root)
}



fn main() {
    println!("{}", conjunctive_normal_form("AB&!"));
    // A!B!|
    println!("{}", conjunctive_normal_form("AB|!"));
    // A!B!&
    println!("{}", conjunctive_normal_form("AB|C&"));
    // AB|C&
    println!("{}", conjunctive_normal_form("AB|C|D|"));
    // ABCD|||
    println!("{}", conjunctive_normal_form("AB&!C!|"));
    // A!B!C!||
    println!("{}", conjunctive_normal_form("AB|!C!&"));
    // A!B!C!&&
    println!("{}", conjunctive_normal_form("AB&C&D&"));
    println!("{}", conjunctive_normal_form("A!B&C&D&B&"));
    println!("{}", conjunctive_normal_form("BCD!A!&&&"));
    println!("{}", conjunctive_normal_form("AB|C!D|&"));
    println!("{}", conjunctive_normal_form("CD^A!A|&B!B|&"));
    println!("{}", conjunctive_normal_form("AC|BC|AD|BD|&&&"));
    println!("{}", conjunctive_normal_form("AB|D|BC|D!|AC!|D|BC!|D|&&&"));
    println!("{}", conjunctive_normal_form("ABCD|||"));
    println!("{}", conjunctive_normal_form("B!C!|A!C|D!|ABCD|||&&"));
    
    // ABCD&&&

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

#[allow(dead_code)]
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


#[allow(dead_code)]
fn pending_false(zero_cells: &Vec<Kmapzero>) -> bool {
    let mut pending = false;
    for it in zero_cells {
        if !it.grouped {
            pending = true;
        }
    }
    pending
}

#[allow(dead_code)]
fn missing_trues(zero_cells: &Vec<Kmapzero>) -> usize {
    let mut pending = 0usize;
    for it in zero_cells {
        if !it.grouped {
            pending += 1;
        }
    }
    pending
}

#[allow(dead_code)]
fn create_16(zero_cells: &Vec<Kmapzero>) -> Vec<Kmapzero> {
    let mut new_zero_cells = zero_cells.clone();
    for it in &mut new_zero_cells{
        it.grouped = true;
    }
    new_zero_cells
}

#[allow(dead_code)]
fn check_left(zero_cells: &Vec<Kmapzero>, row: usize, col: usize, amount: &mut usize, lenght: usize) -> (Vec<Kmapzero>, String) {
    let mut new_zero_cells = zero_cells.clone();
    let mut group: String = Default::default();
    let next_col = if col == 0{lenght - 1}else{col - 1};
    // println!("DAMN: {col} and {next_col} and {lenght}");
    if *amount == 0 {
        return (zero_cells.clone(), group)
    }
    for it in &mut new_zero_cells{
        if it.row == row && it.col == col {
            it.grouped = true;
            group = it.form.clone() + " ";
            // println!("checked {it:?}");
            let mut new_group = Default::default();
            if *amount != 0 {
                *amount -= 1;
                (new_zero_cells, new_group) = check_left(&new_zero_cells, row, next_col, amount, lenght);
            }
            if new_group.is_empty() && *amount != 0{
                // println!("banana");
                break;
            }
            group = group + &new_group;
            // println!("{group}");
            return (new_zero_cells, group)
        }
    }
    (zero_cells.clone(), group)
}

#[allow(dead_code)]
fn check_right(zero_cells: &Vec<Kmapzero>, row: usize, col: usize, amount: &mut usize, lenght: usize) -> (Vec<Kmapzero>, String) {
    let mut new_zero_cells = zero_cells.clone();
    let mut group: String = Default::default();
    let next_col = if col + 1 == lenght{0}else{col + 1};
    // println!("DAMN: {col} and {next_col} and {lenght}");
    if *amount == 0 {
        return (zero_cells.clone(), group)
    }
    for it in &mut new_zero_cells{
        if it.row == row && it.col == col {
            it.grouped = true;
            group = it.form.clone() + " ";
            // println!("checked {it:?}");
            let mut new_group = Default::default();
            if *amount != 0 {
                *amount -= 1;
                (new_zero_cells, new_group) = check_right(&new_zero_cells, row, next_col, amount, lenght);
            }
            if new_group.is_empty() && *amount != 0{
                // println!("banana");
                break;
            }
            group = group + &new_group;
            // println!("{group}");
            return (new_zero_cells, group)
        }
    }
    (zero_cells.clone(), group)
}

#[allow(dead_code)]
fn check_up(zero_cells: &Vec<Kmapzero>, row: usize, col: usize, amount: &mut usize, lenght: usize) -> (Vec<Kmapzero>, String) {
    let mut new_zero_cells = zero_cells.clone();
    let mut group: String = Default::default();
    let next_row = if row == 0{lenght - 1}else{row - 1};
    if *amount == 0 {
        return (zero_cells.clone(), group)
    }
    for it in &mut new_zero_cells{
        if it.row == row && it.col == col {
            it.grouped = true;
            group = it.form.clone() + " ";
            // println!("checked {it:?}");
            let mut new_group = Default::default();
            if *amount != 0 {
                *amount -= 1;
                (new_zero_cells, new_group) = check_up(&new_zero_cells, next_row, col, amount, lenght);
            }
                // println!("banana");
            if new_group.is_empty() && *amount != 0{
                break;
            }
            group = group + &new_group;
            // println!("{group}");
            return (new_zero_cells, group)
        }
    }
    (zero_cells.clone(), group)
}

#[allow(dead_code)]
fn check_down(zero_cells: &Vec<Kmapzero>, row: usize, col: usize, amount: &mut usize, lenght: usize) -> (Vec<Kmapzero>, String) {
    let mut new_zero_cells = zero_cells.clone();
    let mut group: String = Default::default();
    let next_row = if row + 1 == lenght{0}else{row + 1};
    if *amount == 0 {
        return (zero_cells.clone(), group)
    }
    for it in &mut new_zero_cells{
        if it.row == row && it.col == col {
            it.grouped = true;
            group = it.form.clone() + " ";
            // println!("checked {it:?}");
            let mut new_group = Default::default();
            if *amount != 0 {
                *amount -= 1;
                (new_zero_cells, new_group) = check_down(&new_zero_cells, next_row, col, amount, lenght);
            }
            if new_group.is_empty() && *amount != 0{
                // println!("banana");
                break;
            }
            group = group + &new_group;
            // println!("{group}");
            return (new_zero_cells, group)
        }
    }
    (zero_cells.clone(), group)
}

#[allow(dead_code)]
fn check_vertically(zero_cells: &Vec<Kmapzero>, row: usize, col: usize, kmap_info: &mut KmapInfo) -> (Vec<Kmapzero>, String){
    if kmap_info.cols == 4 {
        return (zero_cells.clone(), "".to_string())
    }
    // 4 BY 2
    if kmap_info.group_type == FourTwo  || kmap_info.group_type == TwoTwo{
        let mut new_amount = kmap_info.amount;
        let mut group: String = Default::default();
        let (mut new_zero_cells, mut new_group) = check_up(zero_cells, row, col, &mut new_amount, kmap_info.rows);
        if new_group.is_empty() || new_amount != 0{
            // println!("I NEED TO GO DOWN");
            new_amount = kmap_info.amount;
            (new_zero_cells, new_group) = check_down(zero_cells, row, col, &mut new_amount, kmap_info.rows);
        }
        if !new_group.is_empty() || new_amount == 0{
            group = group + &new_group;
            let next_col = if col == 0{kmap_info.cols - 1}else{col - 1};
            if kmap_info.group_type == FourTwo{
                kmap_info.group_type = FourOne;
            } else if kmap_info.group_type == TwoTwo {
                kmap_info.group_type = TwoOne;
            }
            (new_zero_cells, new_group) = check_vertically(&new_zero_cells, row, next_col, kmap_info);
            if new_group.is_empty(){
                let next_col = if col + 1 == kmap_info.cols {0}else{col + 1};
                (new_zero_cells, new_group) = check_vertically(&new_zero_cells, row, next_col, kmap_info);
            }
            if new_group.is_empty() || new_amount != 0{
                return (zero_cells.clone(), String::new())
            }
            group = group + &new_group;
            
        }   
        
        if group.is_empty() {
            return (zero_cells.clone(), group)
        }
        (new_zero_cells, group)
    } else {
        let mut group: String = Default::default();
        let mut new_amount = kmap_info.amount;
        // println!("KMAP AMOUNT: {}", kmap_info.amount);
        let (mut new_zero_cells, mut new_group) = check_up(zero_cells, row, col, &mut new_amount, kmap_info.rows);
        if new_group.is_empty() || new_amount != 0{
            // println!("I NEED TO GO DOWN");
            new_amount = kmap_info.amount;
            (new_zero_cells, new_group) = check_down(zero_cells, row, col, &mut new_amount, kmap_info.rows);
        }
        if !new_group.is_empty() && new_amount == 0{
            group = group + &new_group;
        }


        
        // println!("what: {new_zero_cells:?}");
        if group.is_empty() || new_amount != 0{
            return (zero_cells.clone(), group)
        }
        (new_zero_cells, group)
    }

}

// 4 by 2 | 4 by 1 | 2 by 1 -> col by row
// need to return updated zero_cells and the group that it made
#[allow(dead_code)]
fn check_horizontally(zero_cells: &Vec<Kmapzero>, row: usize, col: usize, kmap_info: &mut KmapInfo) -> (Vec<Kmapzero>, String){
    // 4 BY 2
    if kmap_info.group_type == FourTwo  || kmap_info.group_type == TwoTwo{
        let mut new_amount = kmap_info.amount;
        let mut group: String = Default::default();
        let (mut new_zero_cells, mut new_group) = check_left(zero_cells, row, col, &mut new_amount, kmap_info.cols);
        if new_group.is_empty() || new_amount != 0{
            new_amount = kmap_info.amount;
            (new_zero_cells, new_group) = check_right(zero_cells, row, col, &mut new_amount, kmap_info.cols);
        }
        // println!("CHECK LEFT: {new_group:?}");
        if !new_group.is_empty() || new_amount == 0{
            group = group + &new_group;
            let next_row = if row  == 0{kmap_info.rows - 1}else{row - 1};
            if kmap_info.group_type == FourTwo{
                kmap_info.group_type = FourOne;
            } else if kmap_info.group_type == TwoTwo {
                kmap_info.group_type = TwoOne;
            }
            (new_zero_cells, new_group) = check_horizontally(&new_zero_cells, next_row, col, kmap_info);
            if new_group.is_empty(){
                let next_row = if row + 1 == kmap_info.rows {0}else{row + 1};
                (new_zero_cells, new_group) = check_horizontally(&new_zero_cells, next_row, col, kmap_info);
            }
            if new_group.is_empty() || new_amount != 0{
                return (zero_cells.clone(), String::new())
            }
            group = group + &new_group;
            
        }   
        
        
        if group.is_empty() {
            return (zero_cells.clone(), group)
        }
        (new_zero_cells, group)
    } else {
        let mut group: String = Default::default();
        let mut new_amount = kmap_info.amount;
        // println!("{new_amount}");
        let (mut new_zero_cells, mut new_group) = check_left(zero_cells, row, col, &mut new_amount, kmap_info.cols);
        if new_group.is_empty() || new_amount != 0{
            new_amount = kmap_info.amount;
            (new_zero_cells, new_group) = check_right(zero_cells, row, col, &mut new_amount, kmap_info.cols);
        }
        // println!("CHECK LEFT: {new_group:?}\nAMOUNT: {new_amount}");
        if !new_group.is_empty() && new_amount == 0{
            group = group + &new_group;
        }

        // println!("what: {new_zero_cells:?}");
        if group.is_empty() || new_amount != 0{
            return (zero_cells.clone(), group)
        }
        (new_zero_cells, group)
    }

}

#[allow(dead_code)]
fn check_square(zero_cells: &Vec<Kmapzero>, row: usize, col: usize, kmap_info: &mut KmapInfo) -> (Vec<Kmapzero>, String){
    // 2 BY 2
    if kmap_info.group_type == TwoTwo {
        let mut new_amount = kmap_info.amount;
        let mut group: String = Default::default();
        // need to check left first, TODO create check_left
        let (mut new_zero_cells, mut new_group) = check_left(zero_cells, row, col, &mut new_amount, kmap_info.cols);
        // println!("CHECK: {new_group:?}\nAmount: {new_amount}");
        if new_group.is_empty() || new_amount != 0{
            new_amount = kmap_info.amount;
            (new_zero_cells, new_group) = check_right(zero_cells, row, col, &mut new_amount, kmap_info.cols);
            // println!("CHECK: {new_group:?}\nAmount: {new_amount}");
        }
        if !new_group.is_empty() || new_amount == 0{
            group = group + &new_group;
            let next_row = if row  == 0{kmap_info.rows - 1}else{row - 1};
            kmap_info.group_type = TwoOne;
            (new_zero_cells, new_group) = check_square(&new_zero_cells, next_row, col, kmap_info);
            if new_group.is_empty(){
                let next_row = if row + 1 == kmap_info.rows {0}else{row + 1};
                (new_zero_cells, new_group) = check_square(&new_zero_cells, next_row, col, kmap_info);
            }
            if new_group.is_empty() || new_amount != 0{
                return (zero_cells.clone(), String::new())
            }
            group = group + &new_group;
            
        }   
        
        
        if group.is_empty() {
            return (zero_cells.clone(), group)
        }
        (new_zero_cells, group)
    } else {
        let mut group: String = Default::default();
        let mut new_amount = kmap_info.amount;
        let (mut new_zero_cells, mut new_group) = check_left(zero_cells, row, col, &mut new_amount, kmap_info.cols);
        if new_group.is_empty() || new_amount != 0 {
            new_amount = kmap_info.amount;
            (new_zero_cells, new_group) = check_right(zero_cells, row, col, &mut new_amount, kmap_info.cols)
        }

        if !new_group.is_empty() && new_amount == 0{
            group = group + &new_group;
        }


        
        // println!("what: {new_zero_cells:?}");
        if group.is_empty() || new_amount != 0{
            return (zero_cells.clone(), group)
        }
        (new_zero_cells, group)
    }
}

#[allow(dead_code)]
fn check_current_cell(zero_cells: &Vec<Kmapzero>, row: usize, col: usize) -> bool{
    for it in zero_cells{
        if it.row == row && it.col == col && !it.grouped{
            // println!("{it:?}");
            return true
        }
    }
    false
}

#[allow(dead_code)]
fn create_8<const C: usize, const R: usize>(zero_cells: &Vec<Kmapzero>, _kmap: [[bool;C];R]) -> (Vec<Kmapzero>, String){
    // check groups of 2 by 4 vertically and horizontally
    let mut new_zero_cells = zero_cells.clone();
    let mut group: String;
    let mut new_formula: String = Default::default();

    for row in (0..R).rev() {
        for col in (0..C).rev() {
            let mut kmap_info = KmapInfo{ rows: R, cols: C, group_type: FourTwo, amount: 4};
            if check_current_cell(&new_zero_cells, row, col) {
                (new_zero_cells, group) = check_vertically(&new_zero_cells, row, col, &mut kmap_info); // need to had a new arg to the checks, because I need the info about the current kmap(columns and rows)
                if group.is_empty(){
                    (new_zero_cells, group) = check_horizontally(&new_zero_cells, row, col, &mut kmap_info);
                }
                if group.is_empty() {continue;}
                let mut parts = group.trim().split(" ");
                let mut common: HashSet<String> = parts.next().unwrap().split(";").map(|s| s.to_string()).collect();
                for it in parts {
                    let current: HashSet<String> = it.split(";").map(|s| s.to_string()).collect();
                    common.retain(|c| current.contains(c));
                    
                }
                for x in common {
                    new_formula = new_formula + &x;
                    
                }
            }
            // check the new_formula and reduce to only 1 Letter
        }
    }
    (new_zero_cells, new_formula)
}

// check vertically, then horizontally, and then square
// need to check false group then all of them with the true maybe ill do it or not
#[allow(dead_code)]
fn create_4<const C: usize, const R: usize>(zero_cells: &Vec<Kmapzero>, _kmap: [[bool;C];R]) -> (Vec<Kmapzero>, String){
    let mut new_zero_cells = zero_cells.clone();
    let mut new_formula: String = Default::default();
    let mut group: String;

    for row in (0..R).rev() {
        for col in (0..C).rev() {
            let mut kmap_info = KmapInfo{ rows: R, cols: C, group_type: FourOne, amount: 4};
            if check_current_cell(&new_zero_cells, row, col) {
                // check group in a collumn
                (new_zero_cells, group) = check_vertically(&new_zero_cells, row, col, &mut kmap_info); // the last value will eventually be a CONST
                // println!("AFTER THE VERTICALLY: {group}");
                
                if group.is_empty(){
                    // check group in a row 
                    // println!("AMOUNT: {}", kmap_info.amount);
                    (new_zero_cells, group) = check_horizontally(&new_zero_cells, row, col, &mut kmap_info);
                    // println!("AFTER THE HORIZONTALLY: {group}");
                    if group.is_empty(){
                        kmap_info.group_type = TwoTwo;
                        kmap_info.amount = 2;
                        (new_zero_cells, group) = check_square(&new_zero_cells, row, col, &mut kmap_info);
                        // println!("THERE IS A SQUARE: {group}");
                    }
                }
                if group.is_empty() {continue;}
                let mut parts = group.trim().split(" ");
                let mut common: HashSet<String> = parts.next().unwrap().split(";").map(|s| s.to_string()).collect();
                for it in parts {
                    let current: HashSet<String> = it.split(";").map(|s| s.to_string()).collect();
                    common.retain(|c| current.contains(c));
                }
                let mut new_group: String = Default::default();
                for x in common {
                    // println!("COMMON: {x:?}");
                    if new_group.is_empty(){ new_group = new_group + "(" + &x; }
                    else{ new_group = new_group + "+" + &x; }
                }
                new_group += ")";
                new_formula += &new_group;
                // println!("CREATE_4: {group}");
            }
        }
    }
    (new_zero_cells, new_formula)
}

#[allow(dead_code)]
fn create_2<const C: usize, const R: usize>(zero_cells: &Vec<Kmapzero>, _kmap: [[bool;C];R]) -> (Vec<Kmapzero>, String){
    let mut new_zero_cells = zero_cells.clone();
    let mut new_formula: String = Default::default();
    let mut group: String = Default::default();

    for row in (0..R).rev() {
        for col in (0..C).rev() {
            let mut kmap_info = KmapInfo{ rows: R, cols: C, group_type: TwoOne, amount: 2};
            if check_current_cell(&new_zero_cells, row, col) {
                // check group in a collumn
                    (new_zero_cells, group) = check_vertically(&new_zero_cells, row, col, &mut kmap_info); // the last value will eventually be a CONST
                // println!("AFTER THE VERTICALLY: {group}");
                
                if group.is_empty(){
                    // check group in a row 
                    // println!("AMOUNT: {}", kmap_info.amount);
                    (new_zero_cells, group) = check_horizontally(&new_zero_cells, row, col, &mut kmap_info);
                    // println!("AFTER THE HORIZONTALLY: {group}");
                }
                if group.is_empty() {continue;}
                let mut parts = group.trim().split(" ");
                let mut common: HashSet<String> = parts.next().unwrap().split(";").map(|s| s.to_string()).collect();
                for it in parts {
                    let current: HashSet<String> = it.split(";").map(|s| s.to_string()).collect();
                    common.retain(|c| current.contains(c));
                }
                let mut new_group: String = Default::default();
                for x in common {
                    // println!("COMMON: {x:?}");
                    if new_group.is_empty(){ new_group = new_group + "(" + &x; }
                    else{ new_group = new_group + "+" + &x; }
                }
                new_group += ")";
                new_formula += &new_group;
                // println!("CREATE_2: {group}");
            }
        }
    }
    (new_zero_cells, new_formula)
}

#[allow(dead_code)]
fn create_1<const C: usize, const R: usize>(zero_cells: &Vec<Kmapzero>, _kmap: [[bool;C];R]) -> (Vec<Kmapzero>, String){
    let mut new_zero_cells = zero_cells.clone();
    let mut new_formula: String = Default::default();
    let mut group: String;

    for iter in &mut new_zero_cells {
        if !iter.grouped{
            iter.grouped = true;
            group = format!("({})", iter.form.replace(";", "+"));
            new_formula += &group;
        }

        // println!("{iter:?}");
    }
    // println!("{new_formula}");
    (new_zero_cells, new_formula)
}

#[allow(dead_code)]
fn grouping<const C: usize, const R: usize>(kmap: [[bool;C];R], zero_cells: &mut Vec<Kmapzero>) -> String{
    // println!("{}", kmap.len());
    // println!("Zero_cells: {:?}", zero_cells);
    // check first if group of 16, close if not check all 8, then if missing 4 then 2 and then 1
    let mut new_formula: String = Default::default();
    // while pending_false(zero_cells) {
    if missing_trues(zero_cells) == 16 {
        *zero_cells = create_16(zero_cells);
        new_formula.push('0');
    }
    if missing_trues(zero_cells) >= 1{
        let group: String;
        (*zero_cells, group) = create_8(zero_cells, kmap);
        // println!("CREATE_8: {}", group);
        new_formula.push_str(&group);
    }
    if missing_trues(zero_cells) >= 1{
        let group: String;
        (*zero_cells, group) = create_4(zero_cells, kmap);
        // println!("CREATE_4: {}", group);
        new_formula.push_str(&group);
    }
    if missing_trues(zero_cells) >= 1{
        let group: String;
        (*zero_cells, group) = create_2(zero_cells, kmap);
        // println!("CREATE_2: {}", group);
        new_formula.push_str(&group);
    }
    if missing_trues(zero_cells) >= 1{
        let group: String;
        (*zero_cells, group) = create_1(zero_cells, kmap);
        new_formula.push_str(&group);
    }
        // break;
    // }
    if new_formula.is_empty() {
        new_formula = "1".to_string();
    }
    new_formula
}


// This will make the Karnaugh map and then I just need to sum the groups where there are 1(true) so we know that the others are 0(false)
#[allow(dead_code)]
fn karnaugh_map4(formula: &str, used_char: &mut Vec<char>) -> String{
    let mut kmap = [[false; 4]; 4];
    let mut zero_cells:Vec<Kmapzero> = Vec::new();
    let mut str_char: String;


    
    used_char.sort();
    // need to create conditin for when the used_char as more than 4 variables to exit (maybe put that in the main)
    let base = 2i64;
    let iterations = base.pow((used_char.len()) as u32);
    for i in 0..iterations {
        let mut changed_formula = formula.to_string();
        str_char = used_char.iter().map(|c| c.to_string()).collect::<Vec<String>>().join(";");
        // println!("{str_char}");
        for j in 0..used_char.len() {
            let bit = (used_char.len() - 1) - j;
            let a = (i >> bit) & 1 == 1;
            changed_formula = changed_formula.replace(used_char[j], &(a as i8).to_string());
            if a {
                str_char = str_char.replace(used_char[j], &format!("{}{}", used_char[j], '!'));
            }
        }
        
        let row = gray_code((i / 4) as u8);
        let col = gray_code((i % 4) as u8);
        // println!("{row}                 {col}");

        
        // let node = give_value_to_char(i, formula, &used_char);
        let val = evaluate(&parse_formula_binary(&changed_formula));
        kmap[row][col] = val;
        if !kmap[row][col] {
            zero_cells.push(Kmapzero {row, col, grouped: false, form: str_char.clone()});
        }
    }
    // print_kmap(kmap);
    convert_group_rpn(grouping(kmap, &mut zero_cells))
    // grouping(kmap, &mut zero_cells)
}

// This will make the Karnaugh map and then I just need to sum the groups where there are 1(true) so we know that the others are 0(false)
#[allow(dead_code)]
fn karnaugh_map3(formula: &str, used_char: &mut Vec<char>) -> String{
    let mut kmap = [[false; 4]; 2];
    let mut zero_cells:Vec<Kmapzero> = Vec::new();
    let mut str_char: String;


    
    used_char.sort();
    // need to create conditin for when the used_char as more than 4 variables to exit (maybe put that in the main)
    let base = 2i64;
    let iterations = base.pow((used_char.len()) as u32);
    for i in 0..iterations {
        let mut changed_formula = formula.to_string();
        str_char = used_char.iter().map(|c| c.to_string()).collect::<Vec<String>>().join(";");
        // println!("{str_char}");
        for j in 0..used_char.len() {
            let bit = (used_char.len() - 1) - j;
            let a = (i >> bit) & 1 == 1;
            changed_formula = changed_formula.replace(used_char[j], &(a as i8).to_string());
            if a {
                str_char = str_char.replace(used_char[j], &format!("{}{}", used_char[j], '!'));
            }
        }
        
        let row = gray_code((i / 4) as u8);
        let col = gray_code((i % 4) as u8);
        // println!("{row}                 {col}");

        
        // let node = give_value_to_char(i, formula, &used_char);
        let val = evaluate(&parse_formula_binary(&changed_formula));
        kmap[row][col] = val;
        if !kmap[row][col] {
            zero_cells.push(Kmapzero {row, col, grouped: false, form: str_char.clone()});
        }
    }
    // print_kmap(kmap);
    convert_group_rpn(grouping(kmap, &mut zero_cells))
    // grouping(kmap, &mut zero_cells)
}

// This will make the Karnaugh map and then I just need to sum the groups where there are 1(true) so we know that the others are 0(false)
#[allow(dead_code)]
fn karnaugh_map2(formula: &str, used_char: &mut Vec<char>) -> String{
    let mut kmap = [[false; 2]; 2];
    let mut zero_cells:Vec<Kmapzero> = Vec::new();
    let mut str_char: String;


    
    used_char.sort();
    // need to create conditin for when the used_char as more than 4 variables to exit (maybe put that in the main)
    let base = 2i64;
    let iterations = base.pow((used_char.len()) as u32);
    for i in 0..iterations {
        let mut changed_formula = formula.to_string();
        str_char = used_char.iter().map(|c| c.to_string()).collect::<Vec<String>>().join(";");
        // println!("{str_char}");
        for j in 0..used_char.len() {
            let bit = (used_char.len() - 1) - j;
            let a = (i >> bit) & 1 == 1;
            changed_formula = changed_formula.replace(used_char[j], &(a as i8).to_string());
            if a {
                str_char = str_char.replace(used_char[j], &format!("{}{}", used_char[j], '!'));
            }
        }
        
        let row = gray_code((i / 2) as u8);
        let col = gray_code((i % 2) as u8);
        // println!("{row}                 {col}");

        
        // let node = give_value_to_char(i, formula, &used_char);
        let val = evaluate(&parse_formula_binary(&changed_formula));
        kmap[row][col] = val;
        if !kmap[row][col] {
            zero_cells.push(Kmapzero {row, col, grouped: false, form: str_char.clone()});
        }
    }
    // print_kmap(kmap);
    convert_group_rpn(grouping(kmap, &mut zero_cells))
    // grouping(kmap, &mut zero_cells)
}


#[allow(dead_code)]
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

#[allow(dead_code)]
fn convert_group_rpn(group: String) -> String{
    let mut rpn: String = Default::default();
    let mut groups: Vec<String> = Vec::new();
    let mut plus = 0usize;
    let mut mini_group: String = Default::default();
    for char in group.chars(){
        if char == '('{continue;}
        if char == ')'&& !mini_group.is_empty(){
            mini_group += &"|".repeat(plus);
            // println!("{mini_group}");
            groups.push(mini_group);
            mini_group = Default::default();
            plus = 0;
            continue;
        }
        if char == '+' {
            plus += 1;
            continue;
        }
        mini_group += &char.to_string();
    }
    // println!("{:?}", groups.len());
    let products = groups.len() - 1;
    rpn = groups.iter().map(|s| s.to_string()).collect();
    rpn += &"&".repeat(products);
    rpn
}


#[cfg(debug_assertions)]
fn print_tree(formula: &str) {
    let node = parse_formula(formula);
    println!("{node:?}");
}


#[cfg(debug_assertions)]
fn print_kmap<const C: usize, const R: usize>(kmap: [[bool;C];R]) {
    for i in 0..R{
        for j in 0..C{
            print!("[{}]", kmap[i][j] as i8);
        }
        println!();
    }
}


