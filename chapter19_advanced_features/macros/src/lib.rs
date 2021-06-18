// Declarative Macros with macro_rules! for General Metaprogramming
#[macro_export]
macro_rules! vec {
    ( $( $x:expr),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
