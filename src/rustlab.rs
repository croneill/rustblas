use std::ops::{Add,Sub,AddAssign};
use std::cmp::{PartialEq};

#[derive(Clone,Copy,Debug)]
pub struct Vec3 (f64, f64, f64);

pub fn dot_product(v1: Vec3, v2: Vec3) -> f64 {
        v1.0 * v2.0 + v1.1 * v2.1 + v1.2 * v2.2
    }

pub fn cross_product(v1: Vec3, v2: Vec3) -> Vec3 {
        Vec3( v1.1*v2.2-v1.2*v2.1, -v1.0*v2.2+v1.2*v2.0, v1.0*v2.1-v1.1*v2.0 )
    }


impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3( self.0 + other.0,  self.1 + other.1, self.2 + other.2)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3( self.0 - other.0,  self.1 - other.1, self.2 - other.2)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Vec3( self.0 + rhs.0,  self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Vec3) -> bool {
        let v_diff = *self - *other;
        dot_product(v_diff,v_diff)<0.0001
    }
}

#[test]
fn addvec3(){
    let v1 = Vec3(1.0,1.0,1.0);
    let v2 = Vec3(0.5,0.0,-1.0);

    println!("{:?}",v1);
    println!("{:?}",v2);
    println!("{:?}",v1+v2);

    println!("{:?}",dot_product(v1,v2));
    println!("{:?}",cross_product(v1,v2));

    assert!(v1+v2 == Vec3(1.5,1.0,0.0));
    assert!(v1-v2 == Vec3(0.5,1.0,2.0));

    assert!(dot_product(v1,v2)==-0.5);
    assert!(cross_product(v1,v2)==Vec3(-1.0,1.5,-0.5));

}
