pub fn run_base() {
    let mut v = vec![1, 2, 3, 4, 5];

    v.push(123);

    
    let third: &i32 = &v[2];
    
    println!("The third element is {}", third);

    let elem: Option<&i32> = v.get(2);
    match elem {
        Some(elem) => println!("The selected element is {}", elem),
        None => println!("There is no such element."),
    }

    println!("All elements");
    for i in &v {
        println!("{}",i)
    }

    println!("Changing elements");
    for i in &mut v {
        *i *= 10;
        println!("{}",i)
    }

}