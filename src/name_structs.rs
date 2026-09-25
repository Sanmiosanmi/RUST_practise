pub struct user{
    username: String,
    email: String,
    age: u32,
    active:bool,
}

//let craete a function, When a variable and a struct field have
// the same name, you can avoid repeating the name:
pub fn create_user(username:String, email:String) -> user {
    user{
        username,
        email,
        age:25,
        active: true,
    }
}