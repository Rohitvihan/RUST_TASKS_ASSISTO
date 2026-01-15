fn factorial(n: u64) -> u64 
{
    (1..=n).product()
}

fn binomial_pmf(k: u64, n: u64, p: f64) -> f64 
{
    let comb = factorial(n) as f64 /
        (factorial(k) * factorial(n - k)) as f64;

    comb * p.powi(k as i32) * (1.0 - p).powi((n - k) as i32)
}

fn main() 
{
    let n = 10;      
    let k = 3;       
    let p = 0.5;     

    let prob = binomial_pmf(k, n, p);

    println!("P(X = {}) = {}", k, prob);
}
