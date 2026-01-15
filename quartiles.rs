fn median(arr: &Vec<f64>) -> f64 
{
    let mut v = arr.clone();
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let n = v.len();

    if n % 2 == 0 {
        (v[n/2 - 1] + v[n/2]) / 2.0
    } else {
        v[n/2]
    }
}

fn quartiles(arr: &Vec<f64>) -> (f64, f64, f64) 
{
    let mut v = arr.clone();
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let n = v.len();
    let q2 = median(&v);
    let q1 = median(&v[0..n/2].to_vec());
    let q3 = median(&v[(n + 1)/2..].to_vec());

    (q1, q2, q3)
}

fn main() 
{
    let data: Vec<f64> = vec![7.0, 15.0, 36.0, 39.0, 40.0, 41.0];

    let (q1, q2, q3) = quartiles(&data);

    println!("Q1 = {}", q1);
    println!("Q2 (Median) = {}", q2);
    println!("Q3 = {}", q3);
}
