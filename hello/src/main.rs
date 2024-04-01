use std::collections::HashMap;

fn main() {

    // Arrays
    // Arrays need the type of item in the array and the size
    let arr = [1,2,3,4,5];

    // a slice must de-reference by using  - & - and [ inclusive .. exclusive ]
    // ampersand gets us the pointer
    let slice = &arr[1..4]; // [1,2,3,4]

    // for the parameters, you have to define the type and length of array
    // The array length of the function must be the same as what is called in
    fn borrowing_slice(arr:[u8;5], slice:&[u8]){

        println!("{:?}", arr);
        println!("{:?}", slice);
        println!("length: {}", slice.len());
        println!("{} {}", slice[0], slice[1])


    }
    borrowing_slice(arr, slice);


    // Strings : strings are immutable in Rust. To make them mutable you have to add a mut

    // There are two ways to make a string
    let mut s: String =  String::from( "Hello James");
    // the string object has functions like push('') and push_str("")

    // and this way
    let str: &str = "Howdy";
    // this string has no functions
    
    s.push('1');
    println!("{}, {}", s, str);

    //tuples 
    // hold more the one kind of item, arrays, ints chars, etc
    //accessed by dot notation

    let tuple = (5,false, 3.5);

    let tuple1 = (1,'3',[1]);
    let tuple2 = (1,"reg");

    println!("tuple1: {}, {}, {:?}", tuple1.0, tuple1.1, tuple1.2);
    println!("tuple2: {:?}", tuple2);

    let (a,b,c) = tuple;
    
    println!("a: {}, b: {}, c: {}", a,b,c);


    // functions
    // arrow is for return value
    //return value is implied by not using a semi colon isn the return statement

    println!("Are they even? 2: {}, 3: {}", is_even(2), is_even(0));
    pub fn is_even(num:u8) -> bool {
        let dig = num % 2; 
         dig == 0 // return a bool
    }


    // for loops
    
    for x in 0..7{
        println!(" loop num: {}",x);

    }

    // match
    let i = 7;
    match i {
        0 => println!("you matched 0"),
        1 => println!("you matched 1"),
        2 => println!("you matched 2"),
        _ => println!("default")

    }
    let brand =  String::from("Toyata");
    let model = String::from("prius");
    let prius = Car { brand, year: 2012, model};
    prius.print_name();


    // vectors are like arrays, but with more functionality
    let mut vec:Vec<i64> = vec![0,1,2,3,5];

    println!("{:?}, length is {} ", vec, vec.len());
    vec.push(1);
    println!("now vec is:{:?}", vec);


    //Hash Maps
    //key pair objects but require you to 
    //import them from library

    let mut map = HashMap::new();

    map.insert(0, "item1");
    map.insert(1, "item2");

    println!("{:?}", map);


    match map.get(&0){
        Some(str) => println!("Matched {}", str),
        _ => println!("No Matches found")
        };


        
    }






// struct === class
// traits are things you can add to structs
// impl adds functions to structscd
 
struct Car {
    brand: String,
    year: i32,
    model:String
}

impl Car {
    fn print_name(&self){
        println!("{}", self.model)
    }
}

impl Truck for Car{
    fn is_electric(&self) -> bool {
        false
    }
}
trait Truck {
    fn bed(&self) -> bool{
        true
    }
    fn is_electric(&self) -> bool;
}