fn main() {

    let var = 1; // created on the stack
    let mut s = "hello".to_string(); //created on the heap
    s.push_str(",world");

}

// var is dropped, heap is dropped