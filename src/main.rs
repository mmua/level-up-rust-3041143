fn median(mut a: Vec<f32>) -> Option<f32> {
    if a.is_empty() {
        return None;
    }
    a.sort_by(|x, y| {x.partial_cmp(y).unwrap()});
    let n_elements = a.len();
    let middle = n_elements / 2;
    let is_even = n_elements % 2 == 0;

    if is_even {
        Some((a[middle] + a[middle - 1]) / 2.0)
    }
    else {
        Some(a[middle])
    }
}
fn main() {
    let answer: Option<f32> = median(vec![1.0, 2.0, 5.0]);
    println!("Hello, Rust learner!");
}

#[test]
fn empty_list() {
    let input: Vec<f32> = vec![];
    let expected_output: Option<f32> = None;
    let actual_output: Option<f32> = median(input);
    assert_eq!(expected_output, actual_output);
}

#[test]
fn sorted_list() {
    let input: Vec<f32> = vec![1.0, 4.0, 5.0];
    let expected_output: Option<f32> = Some(4.0);
    let actual_output: Option<f32> = median(input);
    assert_eq!(expected_output, actual_output);
}

#[test]
fn unsorted_list() {
    let input: Vec<f32> = vec![1.0, 5.0, 2.0];
    let expected_output: Option<f32> = Some(2.0);
    let actual_output: Option<f32> = median(input);
    assert_eq!(expected_output, actual_output);
}

#[test]
fn even_length() {
    let input: Vec<f32> = vec![1.0, 3.0, 5.0, 6.0];
    let expected_output: Option<f32> = Some(4.0);
    let actual_output: Option<f32> = median(input);
    assert_eq!(expected_output, actual_output);
}
