use std::borrow::Cow;

fn fizzbuzz(i: i32) -> Cow<'static, str> {
    match (i % 3, i % 5) {
            (0,0) => "FizzBuzz".into(),
            (1,0) => "Fizz".into(),
            (0,1)=> "Buzz".into(),
            _ => i.to_string().into(),
    }
}

fn main() {
    for i in (1..=1000000).map(fizzbuzz) {
        println!("{}", i);
    }
}
