use askama::Template;
use crate::database::model::post::Post;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub title: String,
    pub posts: Vec<Post>
}

#[derive(Template)]
#[template(path = "editor.html")]
pub struct EditorTemplate {
    pub post: Post
}