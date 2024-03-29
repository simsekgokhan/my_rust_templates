
trait Shape {
    fn area(&self) -> f64;
}

struct Rectangle { height: f64, width: f64 }

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

struct Circle { radius: f64 }

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

// 1. Static dispatch - aka generics
fn print_area<S: Shape>(shape: S) {
    println!("\n--- {}\n", shape.area());
}

#[test] 
fn ex1_static_dispatch() {
    let rec = Rectangle { width: 4.0, height: 3.0};
    print_area(rec);
}

// 2. Dyn dispatch
// This only works if every element in the vector is 
// the same type (Shape type). Not the behavior we want!
fn sum_areas_<S: Shape>(shapes: Vec<S>) -> f64 {
    42.0
}

// With trait objects multiple different shapes can 
// be contained in the vector.
fn sum_areas(shapes: Vec<Box<dyn Shape>>) -> f64 {
    shapes.iter().fold(0., |acc, shape| {
        acc + shape.area()
    })
}

#[test] 
fn ex2_dyn_dispatch() {
    let rec: Box<dyn Shape> = Box::new(Rectangle { width: 4.0, height: 3.0});
    let cir: Box<dyn Shape> = Box::new(Circle { radius: 3.0});
    let vec = vec![rec, cir];
    assert_eq!(sum_areas(vec), 40.27433388230814);    
}

// Src:  
// https://www.mattkennedy.io/blog/rust_polymorphism/