struct user{
    username: String,
    email: String,
    age: u32,
    active:bool,
}





fn main() {
    //now we create an instance 
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
    
}
