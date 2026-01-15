fn range(arr: &Vec<f64>) -> f64 
{
    let mut v = arr.clone();
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v[v.len() - 1] - v[0]
}

fn main() 
{
    let data: Vec<f64> = vec![12.5, 7.0, 20.3, 15.8, 10.0];

    let result = range(&data);

    println!("Range = {}", result);
}
