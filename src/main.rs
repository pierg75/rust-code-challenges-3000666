// The best would be, I think, having a &mut Vec here 
// but that would require to change all the tests.
// So I'll have a tmp mut var to overcome this.
fn median(numbers: Vec<f32>) -> Option<f32> {
    let mut tmp_n = numbers;
    if tmp_n.is_empty() {
        return None;
    }
    tmp_n.sort_by(|x, y| x.partial_cmp(y).unwrap());
    tmp_n.remove(0);
    tmp_n.remove(tmp_n.len() - 1);
    let median: f32 = tmp_n.iter().sum::<f32>()/tmp_n.len() as f32;
    Some(median)
}

fn main() {
    let answer = median(vec![1.0, 2.0, 5.0]);

    println!("median([1,2,5]) = {:?}", answer);
}

#[test]
fn empty_list() {
    let input = vec![];
    let expected_output = None;
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list() {
    let input = vec![1.0, 4.0, 5.0];
    let expected_output = Some(4.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn even_length() {
    let input = vec![1.0, 3.0, 5.0, 6.0];
    let expected_output = Some(4.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unsorted_list() {
    let input = vec![1.0, 5.0, 2.0];
    let expected_output = Some(2.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}
