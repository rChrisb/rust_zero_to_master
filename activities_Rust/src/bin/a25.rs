// Topic: Traits
//
// Requirements:
// * Calculate the perimeter of a square and triangle:
//   * The perimeter of a square is the length of any side*4.
//   * The perimeter of a triangle is a+b+c where each variable
//     represents the length of a side.
// * Print out the perimeter of the shapes
//
// Notes:
// * Use a trait to declare a perimeter calculation function
// * Use a single function to print out the perimeter of the shapes
//   * The function must utilize impl trait as a function parameter
trait Perim {
    fn perimeter(&self);
}
struct Square {
    side: f64,
}
impl Perim for Square {
    fn perimeter(&self) {
        println!("{:?}", self.side * 4.0);
    }
}
struct Triangle {
    a: f64,
    b: f64,
    c: f64,
}
impl Perim for Triangle {
    fn perimeter(&self) {
        println!("{:?}", self.a + self.b + self.c);
    }
}
fn print_perim(figure: impl Perim) {
    figure.perimeter()
}
fn main() {
    let square = Square { side: 3.0 };
    print_perim(square);
    let triangle = Triangle {
        a: 4.5,
        b: 10.5,
        c: 11.5,
    };
    print_perim(triangle)
}