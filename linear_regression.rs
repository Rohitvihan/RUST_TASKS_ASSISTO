fn mean(arr: &Vec<f64>) -> f64 
{
    let mut sum = 0.0;
    for x in arr 
    {
        sum += x;
    }
    sum / arr.len() as f64
}

fn std_dev(arr: &Vec<f64>) -> f64 
{
    let m = mean(arr);
    let mut sum = 0.0;

    for x in arr
    {
        sum += (x - m) * (x - m);
    }

    (sum / arr.len() as f64).sqrt()
}

fn covariance(x: &Vec<f64>, y: &Vec<f64>) -> f64 
{
    let mx = mean(x);
    let my = mean(y);
    let mut sum = 0.0;

    for i in 0..x.len() 
    {
        sum += (x[i] - mx) * (y[i] - my);
    }

    sum / x.len() as f64
}

fn correlation(x: &Vec<f64>, y: &Vec<f64>) -> f64 
{
    covariance(x, y) / (std_dev(x) * std_dev(y))
}

fn linear_regression(x: &Vec<f64>, y: &Vec<f64>) -> (f64, f64, f64) 
{
    let mx = mean(x);
    let my = mean(y);

    let mut num = 0.0;
    let mut den = 0.0;

    for i in 0..x.len() 
    {
        num += (x[i] - mx) * (y[i] - my);
        den += (x[i] - mx) * (x[i] - mx);
    }

    let slope = num / den;
    let intercept = my - slope * mx;
    let r2 = correlation(x, y).powi(2);

    (slope, intercept, r2)
}

fn main()
{
    let x: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let y: Vec<f64> = vec![2.0, 4.0, 5.0, 4.0, 5.0];

    let (m, c, r2) = linear_regression(&x, &y);

    println!("Slope (m) = {}", m);
    println!("Intercept (c) = {}", c);
    println!("R² = {}", r2);
}
