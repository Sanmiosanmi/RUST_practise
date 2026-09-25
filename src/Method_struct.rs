// ADDING METHOD WITH impl
pub struct Rectangle{
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    //area is a method 
    //&self means the method temporarily borrows the Rectangle function
    //in the main, rectangle.area() calls the method.
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

   pub  fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }

    pub fn is_square(&self) -> bool {
       self.width == self.height
    }
}


/*
The most important distinction is:

Self with a capital S means the type Rectangle.
self with a small s means the particular rectangle calling a method.
*/
pub struct Cube{
    pub width: u32,
    pub height: u32,
    pub length: u32,
}


impl Cube{
   /*
   new does not contain any kind of self parameter:
   Therefore, it is called an associated function, not a method.  
We call associated functions with :: i.e Cube::new(2,3,4)

note that the new() oes not calculate any value. it is called the CONSTRUCTOR FUNCTION. 
By using Rectangle::new(10, 5):

Initialization: It takes your initial arguments (10 and 5) and binds them into a single Rectangle instance.

State Storage: The struct instance now holds that data (width: 10, height: 5) in memory.

Method Access: When you later call rect.area() or rect.perimeter(),
 Rust automatically passes rect as &self so those methods can read the exact width and height you set during initialization.
    */
   pub fn new( width:u32,height:u32,length:u32) -> Self{
        //the -> Self means the function we created returns a cube because we use the impl Cube
        Self{width,height,length} // this create and return a new Cube 
      }

 // Creates a square (Associated function) also a CONSTRUCTOR called ALTERNATIVE CONSTRUCTOR
    pub fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
            length : size,
            // if If size is 7, the function creates:w,h,l= 7
        }
    }

// DEFINED METHODS, borrowing self

/*
This example contains both lowercase self and capital Self.
&self means Borrow the existing rectangle that called this method.
Because it uses &self, scale is a method. called Method returing Self

in the main we use let rect = Cube::new(1,2,3)
let scaled_rect = rect.scale(2); // new 2x4x6
*/

pub fn scale(&self, factor: u32) -> Self {
    Self {
        width: self.width * factor,
        height: self.height * factor,
        length:self.height * factor,
    }


    
}

// Calculation methods (Borrowing &self)
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    pub fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }

    pub fn is_square(&self) -> bool {
        self.width == self.height
    }
}
