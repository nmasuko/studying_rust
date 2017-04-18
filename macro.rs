macro_rules! max {
    ($x: expr, $y: expr) => {{
        if $x > $y{
            $x
        }
        else{
            $y
        }
    }};
}

macro_rules! array_sum {
    ( $( $x:expr ),* ) => {
        {
            let mut i = 0;
            $(
                i += $x;
            )*
            i
        }
    };
}

fn main() {
    let z = max!(2, 3);
    println!("{}", z);
    let s = array_sum!(1, 3, 5);
    println!("{}", s);
}