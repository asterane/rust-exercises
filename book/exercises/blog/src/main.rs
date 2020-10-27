use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let mut post = post.request_review();

    let post = {
        let new;
        loop {
            if let Some(out) = post.approve() {
                new = out;
                break
            }
        }
        new
    };

    assert_eq!("I ate a salad for lunch today", post.content());
}
