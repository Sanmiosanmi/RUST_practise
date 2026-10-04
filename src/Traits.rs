

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
/*{
   

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

*/

    //CONCEPT OF SUPERTRAIT
// WE DEFINE TWO Traitstrait
trait Describe {
    fn describe(&self) -> String;
}
// Describe is supertrait of Connect
pub trait Connect: Describe {
    fn connect(&self);
}
// Next we implement both traits for a type Router1
pub struct Router1 {
    pub name: String,
}

impl Describe for Router1 {
    fn describe(&self) -> String {
        format!("Router1: {}", self.name)
    }
}

impl Connect for Router1 {
    fn connect(&self) {
        println!("{} is connecting...", self.describe());
    }
}  //see line 280



// we cnân write a fixed function that run both traits 
fn use_device<T: Connect>(device: &T) {
    println!("{}", device.describe());
    device.connect();
}

/*

// we can more thsn one supertrait
trait Connect1: Describe + Clone {
    fn Connect1(&self);
}
*/
    //Traits Assignment
/*Let’s design a simple logging utility, using a trait Logger with a log method. Code which
 might log its progress can then take an &impl Logger. In testing, this might put messages in 
 the test logfile, while in a production build it would send messages to a log server.

However, the StderrLogger given below logs all messages, regardless of verbosity. 

Your task is to write a VerbosityFilter type that will ignore messages above a maximum verbosity.
This is a common pattern: a struct wrapping a trait implementation and implementing that same 
trait, adding behavior in the process. In the Generics segment, we will see how to make the wrapper generic over the wrapped type.
*/

pub trait Logger {
  /// Log a message at the given verbosity level.
  fn log(&self, verbosity: u8, message: &str);
}

pub struct StderrLogger;

impl Logger for StderrLogger {
  fn log(&self, verbosity: u8, message: &str) {
      eprintln!("verbosity={verbosity}: {message}");
  }
}

/// Only log messages up to the given verbosity level.
pub struct VerbosityFilter {
  pub max_verbosity: u8,
  pub inner: StderrLogger,
}

// TODO: Implement the `Logger` trait for `VerbosityFilter`.
impl Logger for VerbosityFilter{
    fn log(&self, verbosity: u8, message: &str){
        if verbosity <= self.max_verbosity {
            self.inner.log(verbosity, message);
        }  
    }

}