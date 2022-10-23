trait Drawable {
    fn draw(&self);
}

struct Button {
    width: u32,
    height: u32,
    label: String,
}

struct TextField {
    width: u32,
    height: u32,
    text: String,
    font: String,
}

struct Component {
    components: Vec<Box<dyn Drawable>>,
}

impl Component {
    fn draw(&self) {
        for c in self.components.iter() {
            c.draw();
        }
    }
}

impl Drawable for Button {
    fn draw(&self) {
        println!("Button: [{}, {}, {}]", self.width, self.height, self.label);
    }
}

impl Drawable for TextField {
    fn draw(&self) {
        println!("TextField: [{}, {}] text: {}, font: {}", self.width, self.height, self.text, self.font);
    }
}

fn main() {
    let component = Component {
        components: vec![
            Box::new(Button{ width: 100, height: 200, label: String::from("button label")}),
            Box::new(TextField{ width: 500, height: 250, text: String::from("text field"), font: String::from("MS Gothic")})
        ]        
    };

    component.draw();
}
