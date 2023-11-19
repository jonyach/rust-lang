fn main() {

/* 
    Scalar types are types that represent a single value.
In rust, scalar types are:
    1. Integer: These are numbers without fraction components
    2. Floating point: Numbers with fraction components
    3. Boolean: true or false
    4. Characters: letters and emojis
Scalar types are immutable by default
*/

//Integers are: signed (storing both positive and negative values) and unsigned (stores only negative values)

    let admission_number_of_sec:i32 = 3948; //integer
    let year_you_are_currently_in:f32 = 2.1; //float is either f32 or f64 but values such as 10.00 is f64 by default
    let your_units:bool = true; //boolean
    let smile:char = '😃'; //character

    println! ("Your Adm. No. from your high school was: {}", admission_number_of_sec);
    println! ("Your year of study is {}", year_you_are_currently_in);
    println! ("Is it true you're taking 7 units: {}", your_units);
    println! ("That's great comrade {}", smile);
    
}