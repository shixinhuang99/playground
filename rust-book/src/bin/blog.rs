use blog::{Post, DraftPost};

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.reject();
    assert_eq!("", post.content());

    post.add_text("!");
    assert_eq!("", post.content());

    post.request_review();

    post.approve();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today!", post.content());

    post.add_text("djfldjf");
    assert_eq!("I ate a salad for lunch today!", post.content());

    let mut post2 = DraftPost::new();

    post2.add_text("hi");

    let post2 = post2.request_review();

    let post2 = post2.approve();

    assert_eq!("hi", post2.content());
}
