// basic.rs

// [[file:~/Workspace/Programming/gchemol-rs/gut/gut.note::*basic.rs][basic.rs:1]]
use gchemol_gut::prelude::*;

#[test]
fn test_writeln() -> Result<()> {
    let mut lines = String::new();
    write!(&mut lines, "this is ")?;
    writeln!(&mut lines, "line {}", 1)?;
    writeln!(&mut lines, "this is line {}", 2)?;
    assert_eq!(lines, "this is line 1\nthis is line 2\n");

    Ok(())
}
// basic.rs:1 ends here
