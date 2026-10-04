

//implementing the derived propertiy for trait
#[derive(Debug, Clone, PartialEq)]
/*
Debug	println!("{:?}", router1)	Prints a debugging representation of the router
Clone	router1.clone()	Creates a separate router with matching field values
PartialEq	router1 == router2	Checks whether both fields match
*/
pub struct Router {
   pub name: String,
   pub ip_address: String,
}

//--see line 248
/*
fn main() {
   

    // Debug: print the struct's fields.
    println!("{:?}", router1);

    // Clone: create another Router with the same field values.
    let mut router2 = router1.clone();

    // PartialEq: compare the field values of both routers.
    println!("Are they equal? {}", router1 == router2);

    // Change one field in router2.
    router2.name = String::from("Home");

    println!("Are they still equal? {}", router1 == router2);

    // router1 is still available and unchanged.
    println!("Original name: {}", router1.name);
}
*/