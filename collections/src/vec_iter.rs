fn print_vector(v: &Vec<i32>) {
    for i in v {
        println!("{i}");
    }
}

fn main(){
    let v = vec![100, 32, 57];
    print_vector(&v);

    let mut v2 = vec![100, 32, 57];
    for i in &mut v2 {
        *i += 50;
    }
    print_vector(&v2);
}
