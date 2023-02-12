fn main() {
    let school = "University of Nairobi";           //Variable
    const NUMBER_OF_MAX_YEARS_OF_STUDY:i32 = 6;     //Constant
    
    println!("You're currently enrolled at the {}", school);
    println!("The maximum number of years that you can take in this Institution to study only one course is {} years", NUMBER_OF_MAX_YEARS_OF_STUDY);

// only variables are mutable in rust but not constants
    let mut faculty = "This one eill not be printed because it's muted";
    let faculty = "Science and Technology";

    println!("Your faculty name is {}", faculty); //runs with a warning of shadowing

    let today = "today";     //I want the total number of characters in 'today' to be displayed
    let today = today.len();

    println!("The number of characters in a word 'Today' are : {}", today);
}