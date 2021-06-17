//BRING INTO SCOPE 
use hello::greet;
use std::collections::HashMap;
use rand::thread_rng;
fn main() {
    //SINGLE VARIABLE. IMUTABLE!
    let bunnies = 2;
    //MULTIBLE VARIABLES. IMUTABLE!
    let (varOne, varTwo) = (8,50);
    //MUTABLE VARIABLES
    let mut carrots = 32;
    //CONST      BIT TYPE    VALUE
    const WARP_FACTOR: f64 = 9.9;
    //SCOPES
    let y = 11;
    {
        let y = 99;
        println!("{}",y); //PRINTS 99 
    } //Y VAR IS DROPPED
    {
        let mut x = 5; //MUTABLE VARIABLE
        let x = x; //MAKES IMMUTABLE
    }
    println!("{}",y);
    //PRINT STRING
    println!("{}", bunnies);
    //RUNTIME
    let enigma: i32;
    if true{
        enigma = 42;
    }else{
        enigma  = 7;
    }
    println!("Enigmas value is {}", enigma);
    //FUNCTIONS

    //DO_STUFF ARGUMENTS ASSIGNMENT
    let z = do_stuff(2.0,12.5);
    //          ARGUMENTATIONS   RETURN TYPE
    fn do_stuff(qty: f64, oz:f64) -> f64{
        println!("QTY: {} OZ: {}", qty,oz);
        return qty*oz;
    }

    //AREA FUNCTION
    let height = 11;
    let width =  9;
    let depth = 6;
    let area = area_of(width,height);
    println!("Area is {}",area);

    fn area_of(x: i32, y: i32) -> i32{
        return x*y;
    }
    //VOLUME FN

    fn volume_of(x: i32, y:i32, d:i32) -> i32{
        return x*y*d;
    }
    let volume = volume_of(width,height,depth);
    println!("The volume is: {}", volume);


    
    fn call_lib(){
        hello::greet();
    }
    call_lib();
    
    //Mutlible Variables
    let info = (1,2,3);
    let jets = info.0;//returns 1
    let ammo = info.1;//returns 2
    let fuel = info.2;//returns 3

    println!("{},{},{}",jets,ammo,fuel );
    
    //ACCESS ALL FROM TUPEL
    let(jets,ammo,fuel) = info;
    
    //Buf
    let buf: [u8; 3] = [1,2,3];
    
    //Indexing Array
    println!("{}", buf[0])
    
}
