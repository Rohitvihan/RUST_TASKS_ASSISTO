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

    for x in arr {
        sum += (x - m) * (x - m);
    }

    (sum / arr.len() as f64).sqrt()
}

fn covariance(x: &Vec<f64>, y: &Vec<f64>) -> f64 
{
    let mx = mean(x);
    let my = mean(y);
    let mut sum = 0.0;

    for i in 0..x.len() {
        sum += (x[i] - mx) * (y[i] - my);
    }

    sum / x.len() as f64
}

fn correlation(x: &Vec<f64>, y: &Vec<f64>) -> f64 
{
    covariance(x, y) / (std_dev(x) * std_dev(y))
}

// Linear regression: y = b*x + a
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

    let b = num / den;
    let a = my - b * mx;
    let r2 = correlation(x, y).powi(2);

    (b, a, r2)
}

// Logarithmic regression: y = a + b ln(x)
fn logarithmic_regression(x: &Vec<f64>, y: &Vec<f64>) -> (f64, f64) 
{
    let mut ln_x = Vec::new();
    for v in x {
        ln_x.push(v.ln());
    }

    let (b, a, _) = linear_regression(&ln_x, y);
    (a, b)
}

fn main() 
{
    let x: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let y: Vec<f64> = vec![2.0, 4.0, 5.5, 6.5, 7.2];

    let (a, b) = logarithmic_regression(&x, &y);

    println!("Logarithmic Regression Model:");
    println!("y = {} + {} ln(x)", a, b);
}
