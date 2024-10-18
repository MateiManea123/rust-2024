use std::{fmt::Error, string};

use thiserror::Error;

fn ex1() {
    let mut x:u16 = 10;
    
    let mut  r = next_prime(x);
    while r.is_some()
    {
        let value = r.unwrap();
        print!("{} ", value);
        x= value;
        r = next_prime(x);
        if r.is_none()
        {
            print!("\n No more prime numbers after in u16 x");
        }
    }
}fn ex2() {
    let x :u32 = u32::MAX;
    let y = 2;
    check_size_add(x, y);
    check_size_mult(x, y);
}
fn ex3() {
    let x = 42949;
    let y = 2;
    let r = check_size_add_result(x, y);
    match r {
        Ok(value) => print!("The result is {} \n", value),
        Err(e) => print!("{}", e),
    }
    let r = check_size_mult_result(x, y);
    match r {
        Ok(value) => print!("The result is {} \n", value),
        Err(e) => print!("{}", e),
    }
}
fn ex4() {
    let c = 'a';
    match to_uppercase(c) {
        Ok(value) => println!("The uppercase of {} is {}", c, value),
        Err(e) => println!("Error: {}", e),
    }
    match print_char('\u{0}')
    {
        Ok(value ) => (),
        Err(e) => println!("Error: {}", e),
    }
    match char_to_number('g')
    {
        Ok(value) => println!("{}", value),
        Err(e) => print_error(e),
    }
    match char_to_number_hex('7')
    {
        Ok(value) => println!("{}", value),
        Err(e) => println!("Error : {}", e), 
    }
}
fn ex5() {
    let x=1235;
    let e = my_function(x);
    match e {
        Some(value) => println!("{}",value),
        None => println!("The sum of the digits is not even"),
    } 
}

fn isprime(x: u16) -> bool
{   
    if x<2
    {
        return false;
    }
    for i in 2..x/2
    {
        if x%i==0
        {
            return false;
        }
            
    }
    return true;
}
fn next_prime(x: u16) -> Option<u16>
{
    let mut cx = x+1;
    let mut ok =0;
    while ok==0
    {
        if cx>=65535
        {
            return None;
        }
        if isprime(cx)
        {
            ok=1;
        } 
        else
        {
            cx=cx+1;
        }
    }
    return Some(cx);
}
fn check_size_add(x:u32,y:u32) ->bool
{
    let max32 :u32 = u32::MAX;
    if x+y>max32 {
        panic!("The result is greater than the size of u32");
    } 
    return true;
}
fn check_size_add_result(x:u32,y:u32) ->Result<u32, &'static str>
{
    let max32 :u32 = u32::MAX;
    if x+y>max32 {
        return Err("The result is greater than the size of u32");
    } 
    else {
        return Ok(x+y);
    }
}
fn check_size_mult(x:u32,y:u32) ->bool
{
    let max32 :u32 = u32::MAX;
    if  x*y > max32{
        panic!("The result is greater than the size of u32");
    }
    return true;
}
fn check_size_mult_result(x:u32,y:u32) ->Result<u32, &'static str>
{
    let max32 :u32 = u32::MAX;
    if  x*y > max32{
        return Err("The result is greater than the size of u32");
    }
    else {
        return Ok(x*y);
    }
}

#[derive(Debug, Error)]
enum CharError {
    #[error("Character '{0}' is not an ASCII character")]
    NotAscii(char),

    #[error("Character '{0}' is not a digit")]
    NotDigit(char),

    #[error("Character '{0}' is not a base16 digit")]
    NotBase16(char),

    #[error("Character '{0}' is not a letter")]
    NotLetter(char),

    #[error("Character is not printable")]
    NotPrintable(char),
}


fn check_ascii(c: char) -> Result<(), CharError> {
    if c.is_ascii() {
        Ok(())
    } else {
        Err(CharError::NotAscii(c))
    }
}

fn check_digit(c: char) -> Result<(), CharError> {
    if c.is_digit(10) {
        Ok(())
    } else {
        Err(CharError::NotDigit(c))
    }
}

fn check_base16(c: char) -> Result<(), CharError> {
    if c.is_digit(16) {
        Ok(())
    } else {
        Err(CharError::NotBase16(c))
    }
}

fn check_letter(c: char) -> Result<(), CharError> {
    if c.is_alphabetic() {
        Ok(())
    } else {
        Err(CharError::NotLetter(c))
    }
}

fn check_printable(c: char) -> Result<(), CharError> {
    if c.is_ascii_graphic() || c.is_ascii_whitespace() {
        Ok(())
    } else {
        Err(CharError::NotPrintable(c))
    }
}

fn to_uppercase(c: char) -> Result<char, CharError> {
   let e = check_ascii(c);
    match e {
         Ok(_) => Ok(c.to_ascii_uppercase()),
         Err(e) => Err(e), 
         }
    }
fn to_lowercase(c: char) -> Result<char, CharError>{
    let e = check_ascii(c);
    match e{
        Ok(_) => Ok(c.to_ascii_lowercase()),
        Err(e) => Err(e),
    }
}
fn print_char(c:char) -> Result<(), CharError>
{
    let e = check_printable(c);
    match e{
        Ok(_) =>Ok(print!("{}",c)),
        Err(e)=>Err(e),
    }
}
fn char_to_number(c: char) -> Result<u32, CharError>
{
    let e = check_ascii(c);
    match e{
        Ok(_) =>{
            let f = check_digit(c);
            match f
            {
                Ok(_) => Ok(c.to_digit(10).unwrap()),
                Err(e) => Err(e), 
            }
        },
        Err(e) => Err(e),
    }
}
fn char_to_number_hex(c: char) -> Result<u32, CharError>
{
    let e = check_ascii(c);
    match e{
        Ok(_) =>{   
            let f = check_base16(c);
            match f
            {
                Ok(_) => Ok(c.to_digit(16).unwrap()),
                Err(e) => Err(e), 
            }
        },
        Err(e) => Err(e),
    }
}
fn print_error(e:CharError)  
{
    match e{
        CharError::NotAscii(c) => println!("Error! {} is not an ASCII character",c),
        CharError::NotBase16(c) => println!("Error! {} is not an in Base16",c),
        CharError::NotDigit(c) => println!("Error! {} is not a digit",c),
        CharError::NotLetter(c) => println!("Error! {} is not a letter",c),
        CharError::NotPrintable(c) => println!("Error! {} is not printable",c),
    }
}
fn my_function( mut n : u32) -> Option<u32> //Returns the sum of the digits of a number if it is even, and if not, it returns None
{
    let mut sum_cif =0;
    while n!=0
    {
        sum_cif=sum_cif+n%10;
        n=n/10;
    }
    if sum_cif%2 == 0
    {
       return Some(sum_cif);
    }
        
   return None;
}

fn main(){
    // ex1();   
    // ex2();
    // ex3();
    // ex4();
    // ex5();
    
}