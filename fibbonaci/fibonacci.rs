
/// naive recursive implementation
fn nth_fibonacci(n: u64) -> u64 {
    if n <= 1 {
        return n
    }
    
    return nth_fibonacci(n - 1) + nth_fibonacci(n - 2)
}

// ---------------

/// memoization utility
fn nth_fibonacci_util(n: usize, memo: &mut Vec<i64>) -> i64 {
    if n <= 1 {
        return n as i64
    }

    if memo[n] != -1 {
        return memo[n];
    }

    memo[n] = nth_fibonacci_util(n - 1, memo) + nth_fibonacci_util(n - 2, memo);
    memo[n]
}

/// recursive with memoization implementation
fn nth_fibonacci(n: usize) -> i64 {
    let mut memo = vec![-1; n + 1];
    nth_fibonacci_util(n, &mut memo)
}

// ---------------

/// dynamic programming implementation
fn nth_fibonacci(n: usize) -> usize {
    if n <= 1 {
        return n;
    }
    
    let mut dp = vec![0; n + 1];
    dp[0] = 0;
    dp[1] = 1;
    
    for i in 2..=n {
        dp[i] = dp[i - 1] + dp[i - 2];
    } 
    
    dp[n]
}

// ---------------

/// golden ratio implementation (Binet's formula)
fn nth_fibonacci(n: usize) -> usize {
    let sqrt5 = 5_f64.sqrt();
    let phi = (1.0 + sqrt5) / 2.0;

    ((phi.powf(n as f64)) / sqrt5).round() as usize
}

// ---------------


/// space-optimized implementation
fn nth_fibonacci(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }

    let mut curr = 0;
    let mut prev1 = 1;
    let mut prev2 = 0;

    for _ in 2..=n  {
        curr = prev1 + prev2;
        prev2 = prev1;
        prev1 = curr;
    }

    return curr;
}
