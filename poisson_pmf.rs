fn factorial(n: u64) -> u64 
{
    (1..=n).product()
}

fn poisson_pmf(k: u64, lambda: f64) -> f64 
{
    (lambda.powi(k as i32) * (-lambda).exp()) / factorial(k) as f64
}

fn main() 
{
    let lambda = 3.0;   
    let k = 2;         
    let prob = poisson_pmf(k, lambda);

    println!("P(X = {}) = {}", k, prob);
}
