fn main() {

    // Arrays
    // Arrays need the type of item in the array and the size
    let arr = [1,2,3,4,5];

    // a slice must de-reference by using  - & - and [ inclusive .. exclusive ]
    let slice = &arr[1..4]; // [1,2,3,4]

    // for the parameters, you have to define the type and length of array
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

    println!("Are they even? 2: {}, 3: {}", is_even(2), is_even(3));
    pub fn is_even(num:u8) -> bool {
        let dig = num % 2; 
         dig == 0
    }

}
