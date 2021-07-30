// [[file:../gut.note::*basic.rs][basic.rs:1]]
use gchemol_gut::prelude::*;

#[test]
fn test_writeln() -> Result<()> {
    gchemol_gut::logger::setup_logger_for_test();

    let mut lines = String::new();
    write!(&mut lines, "this is ")?;
    writeln!(&mut lines, "line {}", 1)?;
    writeln!(&mut lines, "this is line {}", 2)?;
    assert_eq!(lines, "this is line 1\nthis is line 2\n");

    info!("xx");
    debug!("xx");
    trace!("xx");

    Ok(())
}
// basic.rs:1 ends here
