fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
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
}
