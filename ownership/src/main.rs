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
// fn main(){
//     let x = 1;
//     let y = x;
//     println!("x={}, y={}", x,y);

// }
fn main(){
    // let s = String::from("takes"); //create a variable with a string "takes"
    // takes_ownership(s); // give ownership to the function
    // // println!("{}", s); // variable does not own value

    // let val = 1;
    // make_copy(val);

    // let str1: String = give_ownership();
    // println!("{}",str1);

    // let str3:String = take_and_give(str1);
    // println!("{}",str3);

    // if true{
    //     let _str4 = str3;
    // } else{
    //     let _str5 = str3;
    // }
    // println!("{}", str3); macros.rs(137, 28): Error originated from macro call here
                            // main.rs(60, 5): Error originated from macro call here
                            // main.rs(58, 20): value moved here
                            // main.rs(56, 20): value moved here
                            // main.rs(52, 9): move occurs because `str3` has type `String`, which does not implement the `Copy` trait
                            // main.rs(58, 24): consider cloning the value if the performance cost is acceptable: `.clone()`
                            // main.rs(56, 24): consider cloning the value if the performance cost is acceptable: `.clone()`

    // let mut str1 = String::from("Tyler");
    // let mut str2:String;
    // loop{
    //     str2 = str1; 
    // }


    let mut s = String::from("hello");
    change_string(&mut s);
    println!("{}", s);

}

fn change_string(some_string: &mut String){
    some_string.push_str(", world");
}

// fn takes_ownership(s:String){
//     let strin = s;
//     println!("{}", strin);
// }

// fn make_copy(one:i32){ //i32 implements *copy* trait
//     let val1 = one;
//     println!("{}", val1);
// }

// fn give_ownership()-> String{
//     "given".to_string()
// }

// fn take_and_give(str2:String)->String{
//     str2
// }