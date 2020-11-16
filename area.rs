struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self)-> u32 {
        self.width * self.height
    }
}

fn main() {
    /* area 1 */
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        width1 * height1
    );
    /*******/

    /* area 2 */
    let rect2 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect2)
    );
    /********/

    /* area 3 */
    let rect3 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels",
        rect3.area()
    );

    println!(
        "The area of the rectangle is {} square pixels",
        rect3.area()
    );
    /**********/

    let rect4 = Rectangle {width:30, height:50};
    println!(
        "The area of the rectangle is {} square pixels.",
        area2(&rect4)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area2(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}