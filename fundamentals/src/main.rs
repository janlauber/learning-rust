const SOME_CONST: isize = 3+2;

fn main() {
    // VARIABLES
    let answer = 22; // immutable
    println!("The answer is {}!", answer);
    //answer = 23 // doesn't work!

    // STRING
    let msg = "Not a number";
    let var: usize = "33".parse().expect(msg);
    println!("number: {}", var);

    // CONST
    println!("const: {}", SOME_CONST);

    // SHADOWING
    let shadow: u8 = 12;
    println!("before shadowing: {}", shadow);
    {
        let shadow = 0;
        println!("after shadowing: {}", shadow);
    }
    println!("after shadowing 2: {}", shadow);

    // TUPLES
    let tup = (32, 42_424u16, 42.424);

    println!("tuple: {:?}", tup);

    // ARRAY
    let arr: [u8; 5] = [0, 1, 2, 3, 4];
    

    // STATEMENTS
    {
        let x: i32 = 43;
        x - 1
    };

    // FUNCTIONS
    func(2);
    println!("{}", answer1(4,10));

    // EXPRESSIONS
    let everything: bool = true;
    let answer = if everything { 42 } else { -1 };
    println!("expression answer: {}", answer);

    // LOOP
    let mut i: i32 = 0;
    loop {
        if i <= 2 {
            println!("Get me {} coffee.", i);
            i += 1
        } else {
            break
        }
    }

    // WHILE
    let mut x = u8::MAX;
    while i != 0 {
        x -= 1;

        if x == 42 {
            println!("The answer is: 42");
            break;
        }
    }

    // FOR
    for el in arr.iter() {
        print!("{},", el);
    }
    println!("\n{}", arr[2]);

    let arr = [0, 1, 2];
    let mut index = 0; 
    while index < arr.len() {
        println!("element: {}", arr[index]);
        index += 1
    }
}

// FUNCTIONS
fn func(arg1: u8) {
    println!("arg1: {}", arg1);
}

fn answer1(arg1: u8, arg2: u8) -> u8 {
    let x = 43;
    x - arg1 - arg2
}