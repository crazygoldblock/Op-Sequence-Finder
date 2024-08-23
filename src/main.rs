use std::collections::{HashMap, HashSet};
use std::env;
use std::time::Instant;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() < 5 {
        println!("Must suply 4 arguments");
        return;
    }

    if let Some((nodes, start, end, max)) = parse_arguments(args) {

        let start_time = Instant::now();
        let result = calculate(&nodes, start, end, max);
        let duration = start_time.elapsed();

        match result {
            Some(s) => {
                println!("Time: {:?}, Result steps: {} = ", duration, s.len());
    
                for n in s {
                    print!("{}, ", node_to_string(&n));
                }
            },
            None => println!("No solution"),
        }
    }
    else {
        println!("First argument must define operations, rest numbers");
    }
}
fn parse_arguments(args: Vec<String>) -> Option<(Vec<Operation>, i64, i64, usize)> {
    Some((
        parse_operations(&args[1])?,
        args[2].parse().ok()?,
        args[3].parse().ok()?,
        args[4].parse().ok()?
    ))
}
fn parse_operations(input: &str) -> Option<Vec<Operation>> {

    let mut result = Vec::new();
    let mut operation = ' ';
    let mut num = "".to_string();

    for c in input.chars() {
        if c.is_alphabetic() {
            if operation != ' ' {
                result.push(char_to_operation(operation, &num)?);
                num = "".to_string();
            }
            operation = c;
        }
        else {
            num += &c.to_string();
        }
    }
    if operation != ' ' {
        result.push(char_to_operation(operation, &num)?);
    }
    return Some(result);


    fn char_to_operation(c: char, n: &str) -> Option<Operation> {

        if c == 'r' {
            return Some(Operation::Rem);
        }

        let num: i64 = n.parse().ok()?;

        return Some( match c {
            'a' => Operation::Add(num),
            'm' => Operation::Mul(num),
            'd' => Operation::Div(num),
            'i' => {
                if num < 0 { return None; }
                Operation::Insert(num as u64)
            },
            _ => return None,
        });
    }
}
fn calculate(nodes: &[Operation], start_value: i64, end_value: i64, max_steps: usize) -> Option<Vec<Operation>> {

    if start_value == end_value {
        return Some(Vec::new());
    }

    let res = calculate_recursive(&nodes, &mut Vec::with_capacity(max_steps), start_value, end_value, max_steps);

    if res.len() == 0 {
        return None;
    }
    else {
        return Some(res);
    }
}
fn calculate_recursive(nodes: &[Operation], used: &mut Vec<Operation>, current: i64, end: i64, mut best: usize) -> Vec<Operation> {

    if used.len() >= best {
        return Vec::new();
    }

    if current == end {
        return used.clone(); 
    }

    let mut best_nodes = Vec::new();

    for node in nodes {

        used.push(node.clone());

        let result = calculate_recursive(nodes, used, transform(&node, current), end, best);

        used.pop();

        if result.len() > 0 {
            best = result.len();
            best_nodes = result;
        }
    }
    return best_nodes;
}
fn transform(node: &Operation, n: i64) -> i64 {
    match node {
        Operation::Add(a) => n + a,
        Operation::Mul(a) => n * a,
        Operation::Div(a) => n / a,
        Operation::Insert(a) => n * 10_i64.pow(a.to_string().len() as u32) as i64 + *a as i64,
        Operation::Rem => n / 10,
    }
}
fn node_to_string(node: &Operation) -> String {
    match node {
        Operation::Add(a) => format!("Add({})", a),
        Operation::Mul(a) => format!("Multiply({})", a),
        Operation::Div(a) => format!("Divide({})", a),
        Operation::Insert(a) => format!("Insert({})", a),
        Operation::Rem => format!("Remove"),
    }
}
#[derive(Clone)]
enum Operation {
    Add(i64),

    Mul(i64),
    Div(i64),

    Insert(u64),
    Rem,
}
