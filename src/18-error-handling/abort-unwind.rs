// Different code paths can be conditionally compiled based on the panic setting.
// The current values available are unwind and abort.

// To run with abort or unwind mechanism :
// rustc ./src/18-error-handling/abort-unwind.rs -C panic=abort

// conditional configuration on abort panic machenism
fn drink(beverage: &str) {
   // You shouldn't drink too much sugary beverages.
    if beverage == "lemonade" {
        if cfg!(panic="abort"){ println!("This is not your party. Run!!!!");}
        else{ println!("Spit it out!!!!");}
    }
    else{ println!("Some refreshing {} is all I need.", beverage); }
}

// conditional configuration on unwind panic machenism
// #[cfg(panic = "unwind")]
// fn ah(){ println!("Spit it out!!!!");}

// #[cfg(not(panic="unwind"))]
// fn ah(){ println!("This is not your party. Run!!!!");}

// fn drink(beverage: &str){
//     if beverage == "lemonade"{ ah();}
//     else{println!("Some refreshing {} is all I need.", beverage);}
// }

fn main() {
    drink("water");
    drink("lemonade");
}
