use std::io;
use std::cmp::Ordering;
use rand::Rng; //trait - import

fn main(){
    let guess1 = rand::thread_rng().gen_range(1,101); // const
    let mut guess = String::new(); //mut = variable var
    io::stdin().read_line(&mut guess).expect("read err");
    println!("hello {}", guess);
    // no return

    match guess.cmp(& guess1){
        Ordering::Less => println!("too small"),
        Ordering::Equal => println!("same"),
        Ordering::Greater => println!("too largel"),
    }
}
//rust best for web app + web assembly

// .pdb middleware
// rustc main.rs
// .\main.exe or .\main

// project:
// cargo new <folder> //compiler + package
// src = source
// cargo check
// cargo build --release


// rust + cargo
// python + pip
// node + npm + npx
//react /app/ = source