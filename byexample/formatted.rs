fn main() {
    // In general, the `{}` will be automatically replaced with
    // any arguments. These will be stringified.
    println!("{} days", 2147483647);
    // Without a suffix, 31 becomes an i32. You can change what type 31 is,
    // with a suffix.
    println!("{} days", 2147483648u32);


    // There are various optional patterns this works with. Positional
    // arguments can be used.
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // As can named arguments.
    println!("{subject} {verb} {predicate}",
             predicate = "over the lazy dog",
             subject = "the quick brown fox",
             verb="jumps");

    // Special formatting can be specified after a `:`. Here 2 is printed
    // as a Binary `10`
    println!("{} of {:b} people know binary, the other half don't", 1, 2);

    // It will even check to make sure the correct number of arguments are
    // used.
    // println!("My name is {0}, {1} {0}", "Bond")
    // FIXME ^ Add the missing argument: "James"


    // Create a structure which contains a `i32`. Name it `Structure`
    struct Structure(i32);

    // However, custom types such as this structure require more complicated
    // handling. This will not work.
    // println!("This struct {} won't print...", Structure(3));
}
