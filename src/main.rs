mod Method_struct;
use Method_struct::Rectangle;


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


}