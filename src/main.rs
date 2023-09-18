use std::borrow::Cow;
use std::io::{self, Write, BufWriter};

fn fizzbuzz(i: i32) -> Cow<'static, str> {
    match (i % 3, i % 5) {
            (0,0) => "FizzBuzz".into(),
            (1,0) => "Fizz".into(),
            (0,1)=> "Buzz".into(),
            _ => i.to_string().into(),
    }
}

fn main() {
    let mut stdout = BufWriter::new(io::stdout().lock());
    for i in (1..=1000000).map(fizzbuzz) {
        //println!("{}", i);
        let _ = stdout.write_fmt(format_args!("{}\n", i.as_ref()));
    }
}
