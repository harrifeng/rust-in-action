fn main() {
    println!("{} days", 31);
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    println!("{subject} {verb} {predicate}",
             predicate = "over the lazy dog",
             subject = "the quick brown fox",
             verb="jumps");

    println!("{} of {:b} people know binary, the other half don't", 1, 2);

    ///////////////////////////////////////////////////
    // // FIXME Add the missing argument: "James"    //
    // println!("My name is {0}, {1}, {0}", "Bond"); //
    ///////////////////////////////////////////////////

    ///////////////////////////////////////////////////////////////////////
    // //Create a structure which contains an 'i32', Name it `Structure` //
    // struct Structure(i32);                                            //
    //                                                                   //
    // //This will not work                                              //
    // println!("This struct `{}` won't print ...", Structure(3));       //
    ///////////////////////////////////////////////////////////////////////

}

//////////////////////////////////////////////////////
// <===================OUTPUT===================>   //
// 31 days                                          //
// Alice, this is Bob. Bob, this is Alice           //
// the quick brown fox jumps over the lazy dog      //
// 1 of 10 people know binary, the other half don't //
//////////////////////////////////////////////////////
