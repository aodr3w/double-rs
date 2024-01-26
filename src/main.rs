mod doubly;

use doubly::DoublyLinkedList;

fn main() {
    // Demonstrating with integers
    println!("----- Integer DoublyLinkedList Demo -----");
    let mut nums = DoublyLinkedList::<i32>::new();

    // Append integers
    for i in 1..=5 {
        nums.append(i);
    }
    println!("After appending 1-5:");
    nums.print();

    // Remove a specific value
    nums.remove_value(3);
    println!("After removing 3:");
    nums.print();

    // Remove all values of 2 (if there were duplicates)
    nums.append(2);
    nums.append(2);
    println!("After appending two 2s:");
    nums.print();
    nums.remove_values(2);
    println!("After removing all 2s:");
    nums.print();

    // Remove head and tail
    nums.remove_head();
    println!("After removing head:");
    nums.print();
    nums.remove_tail();
    println!("After removing tail:");
    nums.print();

    // Remove all
    nums.remove_all();
    println!("After removing all elements:");
    nums.print();

    // Demonstrating with strings
    println!("\n----- String DoublyLinkedList Demo -----");
    let mut words = DoublyLinkedList::<String>::new();

    // Append strings
    for word in vec!["apple", "banana", "citrus", "date", "elderberry"] {
        words.append(word.to_string());
    }
    println!("After appending fruits:");
    words.print();

    // Remove a specific value
    words.remove_value("citrus".to_string());
    println!("After removing 'citrus':");
    words.print();

    // Remove all values of 'banana' (if there were duplicates)
    words.append("banana".to_string());
    words.append("banana".to_string());
    println!("After appending two 'banana's:");
    words.print();
    words.remove_values("banana".to_string());
    println!("After removing all 'banana's:");
    words.print();

    // Remove head and tail
    words.remove_head();
    println!("After removing head:");
    words.print();
    words.remove_tail();
    println!("After removing tail:");
    words.print();

    // Remove all
    words.remove_all();
    println!("After removing all elements:");
    words.print();
}
