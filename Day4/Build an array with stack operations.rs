fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
    let mut result = vec![];
    let mut idx = 0;

    for i in 1..=n {
        if target[idx] == i {
            result.push(String::from("Push"));
            idx += 1;
        } else {
            result.push(String::from("Push"));
            result.push(String::from("Pop"));
        }

        if idx == target.len() {
            break;
        }
    }

    result
}

fn main() {
    let target = vec![1, 3];
    let n = 3;
    let result = build_array(target, n);
    println!("{:?}", result);   // ["Push", "Push", "Pop", "Push"]
}