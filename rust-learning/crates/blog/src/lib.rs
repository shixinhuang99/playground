pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        if self.state.as_ref().unwrap().can_add_text() {
            self.content.push_str(text)
        }
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review());
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve());
        }
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject());
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;

    fn approve(self: Box<Self>) -> Box<dyn State>;

    fn content<'a>(&self, post: &'a Post) -> &'a str;

    fn reject(self: Box<Self>) -> Box<dyn State>;

    fn can_add_text(&self) -> bool;
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview { approve_count: 0 })
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn can_add_text(&self) -> bool {
        true
    }
}

struct PendingReview {
    approve_count: u32,
}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(mut self: Box<Self>) -> Box<dyn State> {
        self.approve_count += 1;
        if self.approve_count >= 2 {
            return Box::new(Published {});
        }
        self
    }

    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }

    fn can_add_text(&self) -> bool {
        false
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn can_add_text(&self) -> bool {
        false
    }
}

pub struct DraftPost {
    content: String,
}

impl DraftPost {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> PublishedPost {
        PublishedPost {
            content: self.content,
        }
    }
}

pub struct PublishedPost {
    content: String,
}

impl PublishedPost {
    pub fn content(&self) -> &str {
        &self.content
    }
}
