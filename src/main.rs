fn add(num_1: i32, num_2: i32) -> i32 {
    return num_1 + num_2;
}

fn main() {
    let mut total = add(10,5);
    let mut free_shipping = false;

    // Boolean syntax
    if total > 50 {
        println!("You qualify for free shipping!");
        free_shipping = true;
    }
    else if total > 20 {
        println!("If you add more items, you can qualify for free shipping!");
    }
    else {
        println!("No free shipping");
    }

    // Match Expressions - alternative to conditional statements

    // match free_shipping {
    //     true => total = total + 0,
    //     false => total = total + 5
    // }

    total = match free_shipping {
        true =>  total + 0,
        false => total + 5
    };

    match total {
        1 => println!("1"),
        2 => println!("2"),
        _ => println!("No match found")
    }


    println!("Total: {:?}", total);

    // Arrays - ONLY for fixed value amounts
    let items: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", items);

    // Vectors - more flexible than arrays (but take up more storage)
    
    // Option 1 for vectors
    let vector_items = vec![1, 2, 3, 4, 5];
    // Option 2 for vectors
    let mut vector_items_2 = Vec::new();
    vector_items_2.push(1);
    vector_items_2.push(2);
    vector_items_2.push(3);
    vector_items_2.push(4);
    vector_items_2.push(5);

    println!("{:?}", vector_items);
    println!("{:?}", vector_items_2);


}


