
fn add_chars_n( mut s : String, ch: char, mut n : u32) -> String
{
        while n>0 
        {
            s.push(ch);
            n=n-1;
        }
        return s;
}

fn add_chars_n2( s : &mut String, ch: char, mut n : u32) 
{
        while n>0 
        {
            s.push(ch);
            n=n-1;
        }
      
}
fn add_space(s : &mut String, mut n : i32)
{   while n>0
    {
        s.push(' ');
        n=n-1;
    }
}
fn add_str ( s : &mut String, s2 : &str)
{
    s.push_str(s2);
}
fn add_integer(s : &mut String, mut n : i32)
{
   let mut i = 0;
   let mut count=0;
    while n>0
    {

        i=n%10;
        s.push((i as u8 + '0' as u8) as char);
        n=n/10;
        count=count+1;
        if count%3==0 && n!=0
        {
            s.push('_');
        }
    }
}
fn add_float(s: &mut String, f : f32)
{
    s.push_str(&f.to_string());

}

fn main() {
    let ok= 3;// de aici schimbi daca vrei sa testezi functiile
    let mut s = String::from("");
    let mut i = 0;
    let ref_to_s = &mut s;
    if ok==0 
    {
        while i < 26 {
            let c = (i as u8 + 'a' as u8) as char;
            s = add_chars_n(s, c, 26 - i);
            i += 1;
            print!("{}", s);
        }
    }
    else if ok==1{
        while i < 26 {
            let c = (i as u8 + 'a' as u8) as char;
            add_chars_n2(ref_to_s, c, 26 - i);
            i += 1;
            print!("{}", ref_to_s);
        }
    }
    else if ok==3
    {
        let mut text = String::from("");
        add_space(&mut text, 40);
        add_str(&mut text, "I   💚 \n ");
        add_space(&mut text, 40);
        add_str(&mut text, "Rust \n");
        add_str(&mut text, "\n Most");
        add_space(&mut text, 15);
        add_str(&mut text, "crate");
        add_space(&mut text, 15);
        add_integer(&mut text, 306437968);
        add_space(&mut text, 11);
        add_str(&mut text, "and");
        add_space(&mut text, 10);
        add_str(&mut text, "latest");
        add_space(&mut text, 15);
        add_str(&mut text, "is \n");
        add_space(&mut text, 7);
        add_str(&mut text, "downloaded");
        add_space(&mut text, 15);
        add_str(&mut text, "has");
        add_space(&mut text, 15);
        add_str(&mut text, "downloads");
        add_space(&mut text, 8);
        add_str(&mut text, "the");
        add_space(&mut text, 13);
        add_str(&mut text, "version");
        add_space(&mut text, 10);
        add_float(&mut text, 2.038);
        add_str(&mut text, ".");
        print!("{}", text);
    }
    

    
}