#![allow(dead_code)]

use List::*;

enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        Nil
    }

    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    fn pop( &self) -> List {
        match *self {
            //Cons(_,  ref tail) => *tail,
            Nil => Nil,
            _ => Nil,
        }

    }

    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringify())
            },
            Nil => { format!("Nil")},
        }
    }

}

fn main(){
    let mut list = List::new();
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);
    list = list.prepend(4);

    println!("linked list has length {}", list.len());
    println!("{}", list.stringify());

    list = list.pop();
    list = list.pop();
    list = list.pop();

}


fn main2(){

    let xs: [i32; 5] = [1,2,3,4,5];
    println!(" {:?}", xs);

    println!(" {:?}", &xs[2 .. 4]);

    struct  Point {
        x: f32,
        y: f32,
        z: f32,
    }

    struct Line {
        p1: Point,
        p2: Point,
    }

    //let point: Point = Point { x: 0.3, y: 0.4, z: 0.5};

    struct Leaf {
        phi: f32,
        loc: Point,
    }

    struct Branch {
        a: Voxel,
    }

    enum Voxel {
        Leaf,
        Branch,
    }

    enum Number {
        Zero,
        One,
        Two,
    }

    enum Color {
        Red = 0xff_00_00,
        Green = 0x00_ff_00,
        Blue = 0x00_00_ff,
    }

    println!("zero is {}", Number::Zero as i32);

    println!("Red is {:06x}", Color::Red as i32);
    println!("Red is {:06o}", Color::Red as i32);

}