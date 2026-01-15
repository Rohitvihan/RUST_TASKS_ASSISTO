fn normal_pdf(x: f64, mean: f64, std: f64) -> f64 
{
    let exp = -((x - mean).powi(2)) / (2.0 * std.powi(2));
    (1.0 / (std * (2.0 * std::f64::consts::PI).sqrt())) * exp.exp()
}

fn main() 
{
    let x = 75.0;
    let mean = 70.0;
    let std = 10.0;

    let pdf = normal_pdf(x, mean, std);

    println!("Normal PDF at x = {} is {}", x, pdf);
}
