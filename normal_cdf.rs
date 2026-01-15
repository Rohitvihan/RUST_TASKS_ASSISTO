fn normal_cdf(x: f64, mean: f64, std: f64) -> f64 
{
    0.5 * (1.0 + ((x - mean) / (std * 2f64.sqrt())).tanh())
}

fn main() 
{
    let x = 75.0;
    let mean = 70.0;
    let std = 10.0;

    let cdf = normal_cdf(x, mean, std);

    println!("Normal CDF at x = {} is {}", x, cdf);
}
