fn main() {
    let spaces = "   ";
    let spaces = spaces.len();

    // The following results in compilation error:
    let mut spaces = "   ";
    spaces = spaces.len();
     
}
