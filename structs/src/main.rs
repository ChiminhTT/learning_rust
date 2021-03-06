fn main() {
    let origin = Point { x: 0, y: 0 };
    println!("The origin is at ({}, {})", origin.x, origin.y);

    let mut point = Point { x: 0, y: 0 };
    point.x = 5;
    println!("The origin is at ({}, {})", point.x, point.y);

    // Reference struct
    let mut point = Point { x: 0, y: 0 };
    {
        let r = PointRef {
            x: &mut point.x,
            y: &mut point.y,
        };

        *r.x = 5;
        *r.y = 6;
    }

    assert_eq!(5, point.x);
    assert_eq!(6, point.y);

    // Update syntax
    let mut point = Point3d { x: 0, y: 0, z: 0 };
    point = Point3d { y: 1, ..point };

    // Struct tuple
    let black_tuple = ColorTuple(0, 0, 0);
    let origin = PointTuple(0, 0, 0);
    let black = Color {
        red: 0,
        blue: 0,
        green: 0,
    };

    // NewType pattern
    struct Inches(i32);
    let length = Inches(10);
    let Inches(integer_length) = length;
    println!("length is {} inches", integer_length);

    // Unit-like structs
    struct Electron;
    let x = Electron;

}

struct Point {
    x: i32,
    y: i32,
}

struct PointRef<'a> {
    x: &'a mut i32,
    y: &'a mut i32,
}

struct Point3d {
    x: i32,
    y: i32,
    z: i32,
}

// Struct tuple
struct ColorTuple(i32, i32, i32);
struct PointTuple(i32, i32, i32);

struct Color {
    red: i32,
    blue: i32,
    green: i32,
}
