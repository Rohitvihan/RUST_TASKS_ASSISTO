fn z_score(x: f64, mean: f64, std: f64) -> f64 
{
    (x - mean) / std
}

fn main() 
{
    let x = 85.0;
    let mean = 70.0;
    let std = 10.0;

    let z = z_score(x, mean, std);

    println!("Z-Score = {}", z);
}
