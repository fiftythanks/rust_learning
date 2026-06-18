struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }

    fn new(width: u32, height: Option<u32>) -> Self {
        match height {
            Some(v) => Self { width, height: v },
            None => Self {
                width,
                height: width,
            },
        }
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    fn max(self, other: Rectangle) -> Rectangle {
        Rectangle {
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }
}

fn main() {
    let rect1 = Rectangle::new(70, Some(3));
    let rect2 = Rectangle::new(13, None);

    println!("The area of rectangle 1 is {} square pixels", rect1.area());

    println!(
        "Rectangle 2 has area {}, and rectangle 1 {} fit it in",
        rect2.area(),
        if rect1.can_hold(&rect2) {
            "can"
        } else {
            "can't"
        }
    );

    let max_rect = rect1.max(rect2);
    println!(
        "Max rectangle width is {}, height is {}",
        max_rect.width, max_rect.height
    );
}
