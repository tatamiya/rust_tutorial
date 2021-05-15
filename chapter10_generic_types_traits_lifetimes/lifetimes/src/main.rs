fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    // no need to annotatie lifetime because no reference is used
    fn level(&self) -> i32 {
        3
    }

    // no need to annotate lifetime
    // because of elision rule
    fn announce_and_retrun_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

use std::fmt::Display;

fn longets_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// no need to add lifetime because of elision rule
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    {
        let string3 = String::from("xyz");
        let result2 = longest(string1.as_str(), string3.as_str());
        println!("The longest string is {}", result2);
    }

    //    let result3;
    //    {
    //        let string4 = String::from("hoge");
    //        result3 = longest(string1.as_str(), string4.as_str());
    //    } ERROR! Lifetime of string4 is shorter than result!
    //    println!("The longest string is {}", result3);

    // OK
    let result3;
    let string4;
    {
        string4 = String::from("hogehoge");
        result3 = longest(string1.as_str(), string4.as_str());
    }
    println!("The longest string is {}", result3);

    // Lifetime Annotations in Struct Definitions
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Coul not fild a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    // Lifetime Elision
    println!("{}", first_word("hogehoge fuga hoge hogefuga"));
}
