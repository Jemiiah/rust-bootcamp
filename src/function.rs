struct Add<x: f64, y: f64 > {
x: f64,
y: f64,
}

impl<x,y> Add<x,y> {
    fn add_number<x,y>(self, number: Add<x,y>) -> Add<x,y> {
        x + y

    }
}
fn main() {
    let sum = Add {
        x: 10.5, y: 20.5
    };
    println!("the add up: {:?}", sum);

}