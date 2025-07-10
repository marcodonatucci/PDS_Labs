use itertools::{Itertools, Permutations};
use std::thread;


pub fn mk_ops(symbols: &[char], n: usize) -> Vec<String> {
    if n == 0 {
        return vec![String::new()];
    }

    let mut result = vec![];

    for &symbol in symbols {
        for perm in mk_ops(symbols, n - 1) {
            result.push(format!("{}{}", symbol, perm));
        }
    }

    result
}

pub fn prepare(s: &str) -> Vec<String> {

    let mut result = vec![];
    let ops = mk_ops(&['+', '-', '*', '/'], 4);

    for digit in s.chars().permutations(s.len()) {
        for op_seq in &ops {
            let mut s = String::new();
            let mut it_op = op_seq.chars();
            for d in digit.iter() {
                s.push(*d);
                if let Some(op) = it_op.next() {
                    s.push(op);
                }
            }
            result.push(s);
        }
    }
    result
}

#[test]
fn test_mk_ops() {
    let symbols = vec!['+', '-', '*', '/'];
    let n = 4;
    let result = mk_ops(&symbols, n);
    assert_eq!(result.len(), symbols.len().pow(n as u32));

    let res = prepare("23423");
    println!("{} {:?}", res.len(), res.iter().take(n).collect::<Vec<_>>());
}

fn eval_expr(expr: &str) -> Option<i64> {
    let mut chars = expr.chars().peekable();

    // Parse first digit
    let mut acc = chars.next()?.to_digit(10)? as i64;

    while let Some(op) = chars.next() {
        let n = chars.next()?.to_digit(10)? as i64;
        match op {
            '+' => acc += n,
            '-' => acc -= n,
            '*' => acc *= n,
            '/' => {
                if n == 0 || acc % n != 0 {
                    return None; // divisione per zero o non intera
                }
                acc /= n;
            }
            _ => return None,
        }
    }

    Some(acc)
}

pub fn verify(v: &[String]) -> Vec<String> {
    v.iter()
        .filter_map(|expr| {
            eval_expr(expr).and_then(|val| if val == 10 { Some(expr.clone()) } else { None })
        })
        .collect()
}


fn main() {
    let expressions = prepare("74658"); // o altro input
    let n_threads = 4; // scegli quanti thread vuoi usare

    let chunk_size = (expressions.len() + n_threads - 1) / n_threads;
    let mut handles = vec![];
    let mut results = vec![];

    for i in 0..n_threads {
        let start = i * chunk_size;
        let end = std::cmp::min(start + chunk_size, expressions.len());
        if start >= end {
            continue;
        }

        let slice = &expressions[start..end];

        // Clono la slice perché `verify` prende solo slice di &String e non muove
        let slice = slice.to_vec();

        let handle = thread::spawn(move || {
            verify(&slice)
        });

        handles.push(handle);
    }

    for handle in handles {
        let mut res = handle.join().unwrap();
        results.append(&mut res);
    }

    println!("Trovate {} soluzioni", results.len());
    for sol in results.iter().take(10) {
        println!("{}", sol);
    }
}

