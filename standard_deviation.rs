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

fn main() 
{
    let data: Vec<f64> = vec![10.0, 20.0, 30.0, 40.0, 50.0];

    let result = std_dev(&data);

    println!("Standard Deviation = {}", result);
}
