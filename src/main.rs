//
mod your;
mod rustlab;
static LANGUAGE: &'static str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    n > THRESHOLD
}

fn main() {
    let n = 160877;

    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n){ "big" }else {"small"});

    let _unused = 3u32;

    println!(" Integer {} and Float {}", n, n as f64);

    let m: f32 = 16.9 + 0.2;

    println!(" Integer {} and Float {}", m as i32, m);
    println!(" Binary {:b}", 12345);

    let reference = &4;

    println!(" Ref {}",reference);

    match reference {
        &val => println!("Destructuring: {}", val),
    }
    match *reference {
        val => println!("Dereferencing {}", val),
    }

    struct Foo { x: (u32, u32), y: f64}

    let foo = Foo {x: (1,2), y: 4.0};
    let Foo { x: (a,b),y} = foo;

    println!("{} {} {}",a,b,y);
    let Foo {y: valu, ..} = foo;
    println!("{}",valu);

    let pair = (3,3);

    match pair {
        (x,y) if x==y => println!("Twins!"),
        (x,y) if x+y == 0 => println!("Negatives"),
        (x,_) if x % 2 == 1 => println!("Odd"),
        _ => println!("Who knows..."),
    }

    let age = 20;

    match age {
        0 => println!("Not born"),
        n @ 1 ... 12 => println!("Not yet a teen at {}",n),
        n @ 13 ... 19 => println!("Teen at {}", n),
        n => println!("Older than teen at {}",n),
    }

    for n in 1..100 {
        fizzbuzz(n);
    }

    let rectangle = Rectangle {
        p1: Point::origin(),
        p2: Point::new(2.0,4.0),
    };

    println!("Perimeter {}", rectangle.perimeter());
    println!("Area {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0,1.0),
    };

    square.translate(1.0,00.0);

    fn function (i: i32) -> i32 { i+1}

    let closure_annotated = |i: i32| -> i32{i+1};
    let closure_inferred = |i| i+1;

    let i=1;
    println!("function {}", function(i));
    println!("cl {}", closure_annotated(i));
    println!("cl2 {}", closure_inferred(i));

    let color = "green";

    let print = || println!(" color: {}", color);

    print();
    print();

    let mut count = 0;
    let mut inc = || {
        count += 1;
        println!("count {}", count);
    };

    inc();
    inc();

    let mut farewell = String::from("goodbye");

    farewell.push_str("!!!");
    println!("String = {}",farewell);

    let vec1 = vec![1, 2, 3, 4];
    let vec2 = vec![1, 2, 3, 4];
    let mut iter = vec1.iter();
    let mut into_iter = vec2.into_iter();

    println!("Find 3 in vec1: {:?}", iter.find(|&&x| x == 3));
    println!("Find 3 in vec1: {:?}", into_iter.find(|&x| x == 3));

    fn is_odd(n: u64) -> bool {
        n % 2 == 1
    }

    let upper: u64 = 10000000;

    let mut acc: u64 = 0;
    for n in 0u64 .. {
        let n_squared: u64 = n * n;

        if n_squared >= upper {
            break;
        } else if is_odd(n_squared) {
            acc += n_squared;
        }
    }
    println!("imperative {}", acc);

    let sum_of_squared_odd_numbers: u64 =
        (0..).map(|n| n*n)
            .take_while(|&n| n < upper)
            .filter(|&n| is_odd(n))
            .fold(0, | sum, i | sum + i);
    println!("Functional {}",sum_of_squared_odd_numbers);

    my::function();



    your::yourfunc();
}


fn fizzbuzz(n: u32) -> () {
    if n % 15 == 0 {
        println!("FizzBuzz {} ", n);
    } else if n % 3 == 0 {
        println!("Fizz {}", n);
    } else if n % 5 == 0 {
        println!("Buzz {}", n);
    }
}

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0}
    }
    fn new(x: f64, y: f64) -> Point {
        Point{ x: x, y: y}
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    fn area(self: &Rectangle) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;
        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}
mod my {
    fn private_function() {
        println!("Private");

    }

    pub fn function() {
        println!("public function");
        private_function();
    }


}