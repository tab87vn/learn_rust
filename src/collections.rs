
pub fn test_list() {
    let mut v = Vec::new();

    v.push("Hello");
    v.push("World");
    v.push("Dumbass");

    // let third = v[2];

    println!("{}", &v[2]);

    for i in v.iter() {
        println!("{}", i);
    }
}

pub fn test_list_two() {
    let mut v = vec!["one", "two", "four", "five", "three"];

    let third = v[2];
    println!("The third element is {}", third);

    let second = v.get(1);

    if second.is_some() {
        println!("the second item: {}", second.unwrap());
    }

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let discard = v[0];
    
    println!("Throw away {}", discard);

    for i in v {
        println!("Take ownership of the vector and its element {}", i);
    }

    // for i in v {
    //     println!("Take ownership of the vector and its element {}", i);
    // }
}