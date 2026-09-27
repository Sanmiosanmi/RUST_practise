/*
A tuple struct is a struct whose fields have types and positions, but no field names.
It combines:
1. the structure of a tuple
2. the meaningful type name of a struct
Rust’s official book describes tuple structs as useful when you want to give 
an entire tuple a name and make it a distinct type, but naming every field would be unnecessary.
*/

//defining a tuple 
pub struct color(pub u8, pub u8,pub u8); // tuple struct fields do not have a key-value pair. their meaning depends on thier posuiton 
//let create an instance in the main. line 150



// we can also have methods on tuple struct
pub struct Numbers(pub u32,pub u32);

impl Numbers{
    pub fn multiplication(&self) -> u32 {
        self.0 * self.1  // we call it in line 163
    }
}

//Different tuple structs are different types. Color and points contain the same exact filed, but they are differen types. 
pub struct Color(pub i32,pub i32,pub i32); // its type is (i32,i32,i32)
pub struct Point(pub i32,pub i32,pub i32);
pub fn display_color(newcolor:Color){
    println!("{},{},{}", newcolor.0,newcolor.1,newcolor.2);
  //calling it in line 174
}

//Tuple with one field: a tuple structs can contain just one field
pub struct UserId(pub u32);
pub struct ProductId(pub u32);
//let create a function
pub fn find_user(id: UserId){
    println!("Finding User {}",id.0) // let call it in line 192

}


