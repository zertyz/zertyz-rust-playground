fn match_trick() {

    eprintln!("## match_trick: here we demonstrate how using 'match' may elongate statements for the purposes of lifetimes");
    eprintln!("## more info: https://stackoverflow.com/questions/48732263/why-is-rusts-assert-eq-implemented-using-a-match/48732525#48732525");

    fn f(x: &u32) -> &u32 {
        &x
    }
    fn g() -> u32 {
        10
    }
    // the following line fails with "temporary value is freed at the end of this statement" for '&g()' as of rust 1.53
    //let y = f(&g());
    //println!("y = {}", y);

    // trick to make the statement last longer
    match &g() {
        x => println!("y = {}", f(x)),
    };
}

fn main() {
    eprintln!("###############");
    eprintln!("## LIFETIMES ##");
    eprintln!("###############");
    eprintln!();
    match_trick();
}
