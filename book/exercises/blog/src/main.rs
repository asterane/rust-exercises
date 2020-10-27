use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.add_text("This text won't make it in");
    assert_eq!("", post.content());

    post.approve();
    assert_ne!("I ate a salad for lunch today", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
