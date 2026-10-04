//Enums Defines one custom type that can be one of several possible variants

pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
} // see page 202

//in enum each variants can store its own data and each variants can store its own datatypes.

pub enum NetworkEvent {
    Connected(String),
    Disconnected, // contains no dada
    DataReceived(Vec<u8>),//contains vector of bytes
    Sent (u8,u8),
    Error{
        code: u32,
        message: String,
    } //However all variants are values of the same type NetworkEvent
  // one enums can combine features of unit struct,name struct and tuple structs

}

// Reading enum values with match 
pub fn describe (state:ConnectionState) {
    match state {
        ConnectionState::Disconnected => {  //each line inside the match is called a match arm
            println!("There is no connection")
        }
        ConnectionState::Connecting => {
            println!("connecting to the internet")
        }
        ConnectionState::Connected => {
           println!("Now Connected")
        }
    } //see line 203 
} // NB: Rust require us to match every variants in the enum, if  ConnectionState::Connected  match arm was ommited, it wont compile


                //Using MATCH in enums
        //Extractin the data stored in the variants 
pub fn handle_event (event: NetworkEvent) {
    match event {
         NetworkEvent::Connected(device_name) =>{
             println!("{device_name} is Connected");
        }
        NetworkEvent::DataReceived(_) => {
            println!("vector omitted for DataReceived")
        }
        NetworkEvent::Error { code, message } =>{
            println!("Error {code}: {message}")
        }
        NetworkEvent::Disconnected => {
            println!("Disconnected")
        }
        NetworkEvent::Sent(_,_) => {
            println!("data sent")
        }
        
    }
}


// match can retrun a value: it can calculate and return a value
pub fn _status_code(state: ConnectionState) -> u32{
    match state{
        ConnectionState::Connected =>1,
        ConnectionState::Connecting => 2,
        ConnectionState::Disconnected => 0,
    } //see line 213
}

            //match want us to define a match for every variants.if we want to define for only one, we can use the _

pub fn _status_code_new(state_new: ConnectionState) {
    match state_new{
        ConnectionState::Connected => println!("welcome connected"),
        _ => println!("Nothing"), //for other variants, this is printed
    } // see lin 220
}// careful how u use the _ because it can hide new variants

   // another method to run only a selected variant in match is to use the IF LET
                //IF LET
//we define a new enums
pub enum _Interface {
    Int0,
    Int1,
    Int2(String),
    Int3,
} //see line 227

// we can use the let if to extract stored data see line 233



                //Addig methods to Enums
//Enums can have methods through an impl block, just like structs. 
impl ConnectionState {
   pub  fn description(&self) -> &str {
        match self {
            Self::Connected => "No Connection",
            Self::Connecting => "Now connecting",
            _=> "welcome",
        }
   }
   // under the impl we define another method
   pub fn is_connected(&self) -> bool {
    match self {
         Self::Connected => true,
         _=> false,
    }
   }
} // see line 240

/*
            //option<T> : Rust alternative to null
    // it represnt a value that may or may not exist 

fn find_device(available:bool) -> &Str {
    if available {
        Some(String::from("Router-01"))
    }else {
        None
    }
}

*/