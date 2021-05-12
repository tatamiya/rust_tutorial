//fn largest<T>(list: &[T]) -> T {
//    let mut largest = list[0];
//
//    for &item in list {
//        // > cannot be applied to T
//        if item > largest {
//            largest = item
//        }
//    }
//
//    largest
//}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Point2<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point2<T, U> {
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    //    let number_list = vec![34, 50, 25, 100, 65];
    //    let result = largest(&number_list);
    //    println!("The largest number is {}", result);
    //
    //    let char_list = vec!['y', 'm', 'a', 'q'];
    //    let result = largest(&char_list);
    //    println!("The largest char is {}", result);

    // In Struct
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    // let not_work = Point { x: 5, y: 4.0 };
    let integer_and_float = Point2 { x: 5, y: 4.0 };

    // In Method
    let p1 = Point2 { x: 5, y: 10.4 };
    let p2 = Point2 { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
