#![allow(dead_code)]

fn main(){

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

    let point: Point = Point { x: 0.3, y: 0.4, z: 0.5};

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

    //let root = Voxel::Branch;
    //root.a = Voxel::Leaf;

    //match root {
    //    Leaf => println!("Is a leaf"),
    //    Branch => println!("Is a branch"),
    //}
}