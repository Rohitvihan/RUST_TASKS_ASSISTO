fn percentile(arr: &Vec<f64>, p: f64) -> f64
{
    let mut v = arr.clone();
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let index = ((p / 100.0) * (v.len() as f64 - 1.0)).round() as usize;
    v[index]
}

fn main() 
{
    let data: Vec<f64> = vec![5.0, 10.0, 15.0, 20.0, 25.0, 30.0, 35.0];

    let p = 75.0; // 75th percentile
    let result = percentile(&data, p);

    println!("{}th Percentile = {}", p, result);
}
