fn mean(arr: &Vec<i32>) -> f64
{
    let mut sum: i32 = 0;
    let mut count: i32 = 0;

    for x in arr.iter() 
    {
        sum = sum + x;
        count = count + 1;
    }

    sum as f64 / count as f64
}

fn main() {
    let data: Vec<i32> = vec![10, 20, 30, 40, 50];

    let result: f64 = mean(&data);
    println!("Mean = {}", result);
}
