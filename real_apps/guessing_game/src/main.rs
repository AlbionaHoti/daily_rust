use guessing_game::greet;


// the hello binary
/* 
    since the use statement is outside of any smaller scope, 
    this brings greet into scope for all of main.

    - use statemnt is how you'll bring into scope anything 
        from standard library or from any other project that you want to use.
    - use std::collections::HashMap;

    - shortcuts:
        google search: rust std vector
*/

fn main() {
    /* 
        library name, the same name as the project name, 
    the scope operator ::, then the function greet
        
    - Specificing the absolute path of something at every call, can be painful.
    - use statement comes in. `use` brings an item from some path into soma scope.
     
    */
    greet(); // won't work, yet
}