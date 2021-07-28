
fn main() {


    let normal_vec = vec![1111 2333, 4242, 24112, 5333, 10, 2245, 124];
    let sum_of_normal_vec = sum_of_vec(&normal_vec);
    let sum_of_normal_vec_value = match sum_of_normal_vec {
        Some(value) => value,
        None => 0,
    };
    println!("sum_of_normal_vec_value = {}", sum_of_normal_vec_value);

    let empty_vec = vec![];
    let sum_of_empty_vec = sum_of_vec(&empty_vec);
    let sum_of_empty_vec_value = match sum_of_empty_vec {
        Some(value) => value,
        None => 0,
    };
    println!("sum_of_empty_vec_value = {}", sum_of_empty_vec_value);
}


fn sum_of_vec(some_vec: &[u32]) -> Option<u32> {
    let mut result: u32 = 0;
    for i in some_vec.iter() {
        match result.checked_add(*i) {
            Some(added_result) => result = added_result,
            None => return None,
        }
    }
    Some(result)
}

