fn mean(arr: &Vec<f64>) -> f64 
{
    let mut sum = 0.0;

    for x in arr
    {
        sum += x;
    }

    sum / arr.len() as f64
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

fn main() {
    let x: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let y: Vec<f64> = vec![2.0, 4.0, 6.0, 8.0, 10.0];

    let cov = covariance(&x, &y);

    println!("Covariance = {}", cov);
}
