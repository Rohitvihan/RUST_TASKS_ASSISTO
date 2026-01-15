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
    let mean_x = mean(x);
    let mean_y = mean(y);
    let mut sum = 0.0;

    for i in 0..x.len() 
    {
        sum += (x[i] - mean_x) * (y[i] - mean_y);
    }

    sum / x.len() as f64
}

fn correlation(x: &Vec<f64>, y: &Vec<f64>) -> f64 
{
    covariance(x, y) / (std_dev(x) * std_dev(y))
}

fn main() 
{
    let x: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let y: Vec<f64> = vec![2.0, 4.0, 6.0, 8.0, 10.0];

    let r = correlation(&x, &y);

    println!("Correlation Coefficient = {}", r);
}
