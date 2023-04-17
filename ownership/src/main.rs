// RUST PRINCIPLES - OWNERSHIP
// fn main() {

//     let var = 1; // created on the stack
//     let mut s = "hello".to_string(); //created on the heap
//     s.push_str(",world");
    
// }

// var is dropped, heap is dropped

// RUST PRINCIPLES - MOVE
// fn main(){
//     let x = vec!["Tyler".to_string()];
//     let y = x;
//     // println!("{:?}", x);
//     //error[E0382]: borrow of moved value: `x`
//     let z = y;
//     println!("{:?}", z);

// }

// RUST PRINCIPLES - CLONE
// fn main(){
//     let x = vec!["Tyler".to_string()];
//     let y = x.clone();
//     let z = y.clone();
//     println!("{:?}", x);
//     println!("{:?}", y);
//     println!("{:?}", z);
// }

// RUST PRINCIPLES - COPY
//copy is implemented items already on stack 
fn main(){
    let x = 1;
    let y = x;
    println!("x={}, y={}", x,y);

}