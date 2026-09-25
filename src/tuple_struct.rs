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






