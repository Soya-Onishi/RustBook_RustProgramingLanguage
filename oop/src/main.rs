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

struct Post {
    state: Option<Box<dyn HasState>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self.content.as_ref())
    }

    pub fn add_text(&mut self, text: &str) {
        let pushed_text = self.state.as_ref().unwrap().add_text(text);        
        self.content.push_str(pushed_text);
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review());
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait HasState {
    fn request_review(self: Box<Self>) -> Box<dyn HasState>;    
    fn add_text<'a>(&self, text: &'a str) -> &'a str; 
    fn approve(self: Box<Self>) -> Box<dyn HasState>;
    fn reject(self: Box<Self>) -> Box<dyn HasState>;
    fn content<'a>(&self, post: &'a str) -> &'a str;
}

struct Draft {}
struct PendingReview { count: usize }
struct Published {}

impl HasState for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn HasState> {
        Box::new(PendingReview { count: 0 })
    }

    fn add_text<'a>(&self, text: &'a str) -> &'a str {
        text
    }

    fn approve(self: Box<Self>) -> Box<dyn HasState> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn HasState> {
        self
    }

    fn content<'a>(&self, _post: &'a str) -> &'a str {
        ""
    }
}

impl HasState for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn HasState> {
        self
    }

    fn add_text<'a>(&self, _text: &'a str) -> &'a str {
        ""
    }

    fn approve(self: Box<Self>) -> Box<dyn HasState> {        
        if self.count >= 1 {
            Box::new(Published{})
        } else {
            Box::new(PendingReview{ count: self.count + 1 })
        }        
    }

    fn reject(self: Box<Self>) -> Box<dyn HasState> {
        Box::new(Draft {})
    }

    fn content<'a>(&self, _post: &'a str) -> &'a str {
        ""
    }
}

impl HasState for Published {
    fn request_review(self: Box<Self>) -> Box<dyn HasState> {
        self
    }

    fn add_text<'a>(&self, _text: &'a str) -> &'a str {
        ""
    }

    fn approve(self: Box<Self>) -> Box<dyn HasState> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn HasState> {
        self
    }

    fn content<'a>(&self, post: &'a str) -> &'a str {
        post
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

    let mut post = Post::new();

    // 今日はお昼にサラダを食べた
    post.add_text("I ate a salad for lunch today");
    println!("content: {}", post.content());
    assert_eq!("", post.content());

    post.request_review();
    println!("content: {}", post.content());
    assert_eq!("", post.content());

    post.approve();
    println!("content: {}", post.content());
    assert_eq!("", post.content());    

    post.approve();
    println!("content: {}", post.content());
    assert_eq!("I ate a salad for lunch today", post.content());    

    post.add_text(", also pasta");
    println!("content: {}", post.content());
    assert_eq!("I ate a salad for lunch today", post.content());    
}
