// 受け取った配列やスライスの合計値を返す
fn calculate_sum(arr: &[i64]) -> i64{
    let mut total:i64 = 0;

    for val in arr {
        total += *val;
    }
    total
}

#[test]
fn test_calculate_sum(){
    assert_eq!(calculate_sum(&[1, 2]), 3);
    assert_eq!(calculate_sum(&[0, 4, 5]), 9);
    assert_eq!(calculate_sum(&[-1, 4, 6]), 9);
}

// 受け取った配列やスライスの合計値を返す（再帰的に）
fn calculate_sum_recursion(arr: &[i64]) -> i64{
    if arr.len() == 1 {
        arr[0]
    } else {
        arr[0] + calculate_sum_recursion(&arr[1..])
    }
}

#[test]
fn test_calculate_sum_recursion(){
    assert_eq!(calculate_sum_recursion(&[1, 2]), 3);
    assert_eq!(calculate_sum_recursion(&[0, 4, 5]), 9);
    assert_eq!(calculate_sum_recursion(&[-1, 4, 6]), 9);
}

fn main() {
    println!("Hello, world!");
}
