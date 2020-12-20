pub fn run_day18(puzzle_input: &str) {
    let input: String = String::from(
        std::fs::read_to_string(&puzzle_input)
            .expect(format!(
                "Could not read file \"{}\".",
                &puzzle_input
            ).as_str())
            .trim()
    );

    let results: Vec<i64> = input.split('\n')
        .map(|line| compute_left_to_right(&line))
        .collect();
    println!("Sum of left-to-right results: {}", results.iter().sum::<i64>());

    let results: Vec<i64> = input.split('\n')
        .map(|line| compute_addition_precedence(&line, true))
        .collect();
    println!(
        "Sum of addition-precedence results: {}",
        results.iter().sum::<i64>()
    );
}

fn compute_left_to_right(expr: &str) -> i64 {
    let mut brace_count: usize = 0;
    // Counting start and end from the end of the string:
    let mut brace_start: Option<usize> = None;
    let mut brace_end: usize = 0;
    let n = expr.len();
    for (i, c) in expr.chars().rev().enumerate() {
        match c {
            '*' => {
                if brace_count == 0 {
                    return compute_left_to_right(&expr[..n-i-1])
                        * compute_left_to_right(&expr[n-i..]);
                }
            },
            '+' => {
                if brace_count == 0 {
                    return compute_left_to_right(&expr[..n-i-1])
                        + compute_left_to_right(&expr[n-i..]);
                }
            },
            ')' => {
                brace_count += 1;
                if brace_start == None { brace_start = Some(i); }
            },
            '(' => {
                if brace_count == 0 {
                    panic!("Mismatched braces in \"{}\"", &expr);
                }
                brace_count -= 1;
                brace_end = i;
            },
            _ => {}  // spaces or digits -> ignore in this loop
        }
    }

    // Found no addition or multiplication outside of braces.
    // -> Enter outer braces if braces exist, otherwise parse number:
    if let Some(brace_start) = brace_start {
        return compute_left_to_right(&expr[n-brace_end..n-brace_start-1]);
    }
    expr.trim().parse::<i64>().expect(
        format!("\"{}\" is not a number", &expr).as_str()
    )
}

fn compute_addition_precedence(expr: &str, ignore_sums: bool) -> i64 {
    // ignore_sums is used to first handle the products while
    // recursively moving inwards…
    let mut brace_count: usize = 0;
    // Counting start and end from the end of the string:
    let mut brace_start: Option<usize> = None;
    let mut brace_end: usize = 0;
    let n = expr.len();
    let mut addition_count: usize = 0;
    for (i, c) in expr.chars().rev().enumerate() {
        match c {
            '*' => {
                if brace_count == 0 {
                    return compute_addition_precedence(&expr[..n-i-1], true)
                        * compute_addition_precedence(&expr[n-i..], true);
                }
            },
            '+' => {
                if brace_count != 0 { continue; }
                if !ignore_sums {
                    return compute_addition_precedence(&expr[..n-i-1], false)
                        + compute_addition_precedence(&expr[n-i..], false);
                }
                addition_count += 1;
            },
            ')' => {
                brace_count += 1;
                if brace_start == None { brace_start = Some(i); }
            },
            '(' => {
                if brace_count == 0 {
                    panic!("Mismatched braces in \"{}\"", &expr);
                }
                brace_count -= 1;
                brace_end = i;
            },
            _ => {}  // spaces or digits -> ignore in this loop
        }
    }
    if addition_count > 0 {
        // All products outside of braces have been handled, and
        // there are some additions left that should be handled before
        // recursing into braces:
        return compute_addition_precedence(&expr, false);
    }

    // Found no addition or multiplication outside of braces.
    // -> Enter outer braces if braces exist, otherwise parse number:
    if let Some(brace_start) = brace_start {
        return compute_addition_precedence(
            &expr[n-brace_end..n-brace_start-1], true
        );
    }
    expr.trim().parse::<i64>().expect(
        format!("\"{}\" is not a number", &expr).as_str()
    )
}
