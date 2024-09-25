macro_rules! my_vec {
    () => {{
        Vec::new()
    }};

    (make an empty vec) => {{
        Vec::new()
    }};

    // not tsrictly needed
    ($x:expr) => {{
        vec![$x]
    }};

    ($($x:expr),+) => {{
        let mut v = Vec::new();
        $(
            v.push($x);
        )+
            v
    }}
}

fn main() {
    let empty: Vec<i32> = my_vec![];
    println!("{empty:#?}");

    let also_empty: Vec<i32> = my_vec!(make an empty vec);
    println!("{also_empty:#?}");

    let single_number = my_vec![1];
    println!("{single_number:#?}");

    let three_numbers = my_vec![1, 2, 3];
    println!("{three_numbers:#?}");
}
