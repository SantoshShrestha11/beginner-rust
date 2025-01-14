use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
    Ok(())
}

fn function2() -> io::Result<()> {
    // --snip--
    Ok(())
}
//this is the normal way , it should have the different parent name for getting 
//the proper access
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
    ok(())
}

fn function2() -> IoResult<()> {
    // --snip--
    ok(())
}
//this is the way we can use the as keyword and give the new local name 
//this is how we are bringing two types of the same name into the same scope with 
