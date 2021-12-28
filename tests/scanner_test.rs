use std::error::Error;

use purua::scanner::Scanner;

extern crate purua;

#[test]
fn test_scanned_1() -> Result<(), Box<dyn Error>> {
    let source = include_str!("scanner/test_1.lua");
    let mut scanner = Scanner::new(source);
    scanner.scan()?;

    assert_eq!(1, 1);
    Ok(())
}

#[test]
fn test_scanned_2() -> Result<(), Box<dyn Error>> {
    let source = include_str!("scanner/test_2.lua");
    let mut scanner = Scanner::new(source);
    scanner.scan()?;

    assert_eq!(1, 1);
    Ok(())
}

#[test]
fn test_scan_failed_1() -> Result<(), Box<dyn Error>> {
    let source = include_str!("scanner/error_1.lua");
    let mut scanner = Scanner::new(source);

    assert!(scanner.scan().is_err());
    Ok(())
}
