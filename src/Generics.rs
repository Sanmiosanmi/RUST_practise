fn identity <T>(value:T)->T{
    value
}
//Rust CREATE A SEPERATE FUNCTION FOR EACH TYPE(Monomorphization)
println!("{}",identity(10));
println!("{}",identity(10.23));
println!("{}",identity("hello"));
println!("{}",identity(String::from("Hello")));

#[derive(Debug)]
struct user{age: u32,}

println!("{:?}",identity(user{age:12}));

//TURBOFISH OPERATOR   SPECIFYING THE TYPE
println!("{}",identity::<i8>(10));



 //multiple generics

fn make_tuple <T>(first:T,second:T)->(T,T){
    (first,second)
}

make_tuple(10, 10);// both types have to be the same 



fn make_tuple_new<T,U>(first:T,second:U)->(T,U){
    (first,second)
}
  make_tuple_new(10, "hello");//now i can use two diffrernt types



   // Generics in STRUCTS
#[derive(Debug)]
 struct user<T>{
        username:String,
        active: T,
    }

let user1 = user{
    username:String::from("Sanmi"),
    active:32
};
let user2 = user{
    username:String::from("Sanmi"),
    active:"NEW"
};
let user3 = user{
    username:String::from("Sanmi"),
    active:true
};
println!("{:?}",user1);
println!("{:?}",user2);
println!("{:?}",user3);