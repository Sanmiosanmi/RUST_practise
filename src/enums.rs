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