mod Method_struct;
mod tuple_struct;
mod enums;
mod Traits;


use std::io::ErrorKind::NetworkDown;
use Method_struct::Rectangle;
use Method_struct::Cube;
use Method_struct::Rectangle1;
use tuple_struct::color;
use tuple_struct::Color;
use tuple_struct::UserId;
use tuple_struct::ProductId;
use tuple_struct::Numbers;
use crate::Traits::Router1;
use crate::Traits::Connect;
use crate::enums::_status_code;
use crate::enums::_status_code_new;
use crate::enums::ConnectionState;
use crate::enums::NetworkEvent;
use crate::enums::describe;
use crate::enums::handle_event;
use crate::tuple_struct::Point;
use crate::tuple_struct::display_color;
use crate::tuple_struct::find_user;
use enums::_Interface;
use Traits::Router;



//structs
struct user{
    username: String,
    email: String,
    age: u32,
    active:bool,
}
//let craete a function, When a variable and a struct field have
// the same name, you can avoid repeating the name:
fn create_user(username:String, email:String) -> user {
    user{
        username,
        email,
        age:25,
        active: true,
    }
}


    //generally 

fn main() {
    /*  */
    /*   //now we create an instance 
    let user1 = user{
        username:String::from("Sunky"),
        email:String:: from("old.com"),
        age:32,
        active:true,
     };

     println!("username:{}", user1.username);

      //we can change a field by using the mut keyword
    let mut  user2 = user{
        username:String::from("Sunky"),
        email:String:: from("old.com"),
        age:32,
        active:true,
     };
    user2.username = String::from("smart");

    println!("username:{}", user2.username)
*/

//section 2
  /*  
// let call the function
let user3 = create_user(
    String:: from("sanmi.com"),
    String::from("xample@ist.com"),

);
 println!("username3:{}", user3.username);
 println!("username3:{}", user3.email);
 println!("username3:{}", user3.age);
 println!("username3:{}", user3.active);
 //STRUCTURE UPDATE SYTAX
 //You can create a new struct by copying the remaining fields from an existing instance:

 let user_3_new = user{
    username:String::from("Sunky"),
    ..user3
     };

 println!("username:{}", user_3_new.username);
 println!("username:{}", user_3_new.email);
 println!("username:{}", user_3_new.age);
 println!("username:{}", user_3_new.active);
*/

/* 



//Adding methods with impl
//You can define behaviour for a struct using an impl block:
// see function and method above
let fisrt_rectangle = Rectangle {
    width: 10,
    height: 5,
};
//let call the methods
println!("Area{}",fisrt_rectangle.area());
println!("Area{}",fisrt_rectangle.perimeter());
println!("Area{}",fisrt_rectangle.is_square());

*/


/* 





//Self

// 1. Create a 10x5 rectangle using Cube::new(...)
let cube_new = Cube::new(2,3,4);
// Call calculation methods
    println!("--- Initial Cube (2x3x4) ---");
    println!("Volume: {} sq units", cube_new.volume());         
    println!("Perimeter: {} units", cube_new.perimeter());    
    println!("Is it a square? {}\n", cube_new.is_square());   
// 2. Call scale(&self) to create a new double-sized Cube
    let scaled_rect =cube_new.scale(2); 

    println!("--- Scaled Rectangle (20x10) ---");
    println!("Scaled Area: {} sq units", scaled_rect.volume()); 
    // 3. Create a square using Cube::square(...)
    let my_square = Cube::square(7); 

    println!("\n--- Square Instance (7x7) ---");
    println!("Square Area: {} sq units", my_square.volume());    
    println!("Is it a square? {}", my_square.is_square());  
*/




/* 
   //&mut self

     // IMPORTANT: The variable MUST be declared as `mut` to call &mut self methods!
    let mut rect = Rectangle1::new(10, 5);

    println!("Original Area: {}", rect.area()); // 10 * 5 = 50

    // Modifies `rect` directly
    rect.scale_in_place(2);
    println!("After Scaling: {}x{}", rect.width, rect.height); // 20x10
    println!("New Area: {}", rect.area()); // 20 * 10 = 200

    // Modifies `rect` again
    rect.expand(5, 5);
    println!("After Expanding: {}x{}", rect.width, rect.height); // 25x15  
    */


/* 
    // tuple structs

    let orange = color(255,165,0);  // here we say the type of orange is color. the tuple struct provides a meaniful and distinct
    //type name
    // accessing the fields

println!("Red:{}", orange.0);
println!("Green:{}", orange.1);
println!("Blue:{}", orange.2);
  //Destructuring a tuple struct : You can extract its fields into separate variables:
  let color(red,green,blue) = orange; //means: put orange.0 into red put orange.1 into green put orange.2 into blue
    println!("Red: {red}");
    println!("Green: {green}");
    println!("Blue: {}",blue);
*/

/* 
// tuple Methods 

let multiplication_new = Numbers(2,5);

println!("first_number: {}", multiplication_new.0);
println!("secon_number: {}", multiplication_new.1);
println!("{}", multiplication_new.multiplication());



let color1 = Color(2,3,4);
    display_color(color1);
let Point1 = Point(5,6,7);
 //   display_color(Point1);//wont run, tuple structs prevent unrelated values from mixing 



let new_userid = UserId(23);
find_user(new_userid);
*/


/* 

            //ENUMS
let state = ConnectionState::Connecting; // we can only assign the variants we define in the enum
 describe(state);

 let event_new = NetworkEvent::Connected(String:: from ("Router 01"));
    handle_event(event_new);

let state = ConnectionState::Connected;
let code = _status_code(state);
println!("ststus code: {code}");


let state_new = ConnectionState::Connected;
let state_new_2 = ConnectionState::Connecting;
_status_code_new(state_new);
_status_code_new(state_new_2);


    let state = _Interface::Int0;
    if let _Interface::Int0 = state{
    println!("Connected to interface 0")        
    } 


let state1 = _Interface::Int2 (String::from("Connected to interface 2"));
    if let _Interface::Int2(content) = state1{
    println!("{content}");        
    }    

*/
/*
    //using the methods define
let state = ConnectionState::Connected;
println!("{}",state.description());
println!("{}",state.is_connected());

*/

/*

 let router1 = Router {
        name: String::from("Office"),
        ip_address: String::from("192.168.1.1"),
    };
  // Debug: print the struct's fields.
    println!("{:?}", router1);

     // Clone: create another Router with the same field values.
  let mut router2 = router1.clone();
    println!("{:?}", router2);

    // PartialEq: compare the field values of both routers.
    println!("Are they equal? {}", router1 == router2);

  // Change one field in router2. since we already declear it as mut in line 258
    router2.name = String::from("Home");
    println!("{:?}", router2);

    println!("Are they still equal? {}", router1 == router2);

    // router1 is still available and unchanged.
    println!("Original name: {}", router1.name);
*/



    let router = Router1 {
        name: String::from("Office"),
    };

    router.connect();
}

