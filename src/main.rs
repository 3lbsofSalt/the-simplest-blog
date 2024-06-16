use std::fs;

use markdown;

use anyhow::Context;
use askama::Template;
use axum::{
    extract::Path, http::HeaderMap, response::{Html, IntoResponse}, routing::get, Router
};

use tower_http::services::ServeDir;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "with_axum_htmx_askama=debug".into())
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("initializing router...");


    let assets_path = std::env::current_dir().unwrap();
    let address = "0.0.0.0:3000";

    info!("router initialized, now listening at {}", address);
    let router = Router::new()
        .route("/", get(index))
        .route("/posts", get(posts))
        .route("/post/:index", get(post))
        .route("/projects", get(projects))
        .route("/project/:index", get(project))
        .route("/about", get(about))
        .nest_service(
            "/assets",
            ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap()))
        );

    let listener = tokio::net::TcpListener::bind(address).await.context("Error initializing address")?;
    axum::serve(listener, router).await.context("Error while starting server")?;

    Ok(())
}


async fn index_with_url(url: String) -> impl IntoResponse {
    let template = IndexTemplate { start_link: url };
    let reply_html = template.render().unwrap();

    Html(reply_html)
}




#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    start_link: String
}

async fn index() -> impl IntoResponse {
    let template = IndexTemplate { start_link: "/posts".to_string() };


    let reply_html = template.render().unwrap();

    Html(reply_html).into_response()
}




struct Post {
    id: String,
    file: String,
    title: String,
    content: String,
    tags: Vec<String>,
    publish_date: String
}

#[derive(Template)]
#[template(path = "post.html", escape = "none")]
struct PostTemplate {
    post: Post
}

async fn post(Path(id): Path<String>, headers: HeaderMap) -> impl IntoResponse {
    println!("ID: {}", id);
    if headers.contains_key("HX-Request") {
        let post_index: String = fs::read_to_string("posts/index.json").unwrap();
        let mut post_index = json::parse(post_index.as_str()).unwrap();

        let post_reference = post_index["posts"].members_mut().find(|post| post["id"].as_str().unwrap().eq(&id)).unwrap();
        println!("{:?}", post_reference);
        let content = fs::read_to_string(format!("posts/{}", post_reference["file"])).unwrap();
        let post = Post {
            id: post_reference["id"].take_string().unwrap(),
            file: post_reference["file"].take_string().unwrap(),
            title: post_reference["title"].take_string().unwrap(),
            content: markdown::to_html_with_options(
                &content.as_str(),
                &markdown::Options::gfm()
            ).unwrap(),
            tags: vec![],
            publish_date: post_reference["publish date"].take_string().unwrap()
        };

        let template = PostTemplate { post };

        let reply_html = template.render().unwrap();
        Html(reply_html).into_response()
    } else {
        index_with_url(format!("/post/{}", id)).await.into_response()
    }
}

#[derive(Template)]
#[template(path = "posts.html")]
struct PostsTemplate {
    posts: Vec<Post>
}

async fn posts(headers: HeaderMap) -> impl IntoResponse {

    if headers.contains_key("HX-Request") {
        let post_index: String = fs::read_to_string("posts/index.json").unwrap();
        let mut post_index = json::parse(post_index.as_str()).unwrap();

        let mut posts: Vec<Post> = Vec::new();
        for post in post_index["posts"].members_mut() {
            let content = fs::read_to_string(format!("posts/{}", post["file"])).unwrap();
            let temp_post = Post { 
                id: post["id"].take_string().unwrap(),
                file: post["file"].take_string().unwrap(),
                title: post["title"].take_string().unwrap(), 
                content: content.to_string(),
                tags: vec![],
                publish_date: post["publish date"].take_string().unwrap()
            };
            posts.push(temp_post);
        }

        let template = PostsTemplate { posts };
        let reply_html = template.render().unwrap();

        Html(reply_html).into_response()
    } else {
        index_with_url("/posts".to_string()).await.into_response()
    }

}


struct Project {
    id: String,
    github_link: String,
    title: String,
    content: String,
    file: String
}


#[derive(Template)]
#[template(path = "project.html", escape = "none")]
struct ProjectTemplate {
    project: Project
}

async fn project(Path(id): Path<String>, headers: HeaderMap) -> impl IntoResponse {
    if headers.contains_key("HX-Request") {
        let project_index: String = fs::read_to_string("projects/index.json").unwrap();
        let mut project_index = json::parse(project_index.as_str()).unwrap();

        let project_reference = project_index["projects"].members_mut().find(|project| project["id"].as_str().unwrap().eq(&id)).unwrap();
        let content = fs::read_to_string(format!("projects/{}", project_reference["file"])).unwrap();
        let project = Project {
            id: project_reference["id"].take_string().unwrap(),
            file: project_reference["file"].take_string().unwrap(),
            github_link: project_reference["github_link"].take_string().unwrap(),
            title: project_reference["title"].take_string().unwrap(),
            content: markdown::to_html_with_options(
                &content.as_str(), 
                &markdown::Options::gfm()
            ).unwrap()
        };

        let template = ProjectTemplate { project };

        let reply_html = template.render().unwrap();
        Html(reply_html).into_response()
    } else {
        index_with_url(format!("/project/{}", id)).await.into_response()
    }
}

#[derive(Template)]
#[template(path = "projects.html")]
struct ProjectsTemplate {
    projects: Vec<Project>
}

async fn projects(headers: HeaderMap) -> impl IntoResponse {
    if headers.contains_key("HX-Request") {
        let project_index: String = fs::read_to_string("projects/index.json").unwrap();
        let mut project_index = json::parse(project_index.as_str()).unwrap();

        let mut projects: Vec<Project> = Vec::new();
        for project in project_index["projects"].members_mut() {
            let content = fs::read_to_string(format!("projects/{}", project["file"])).unwrap();
            let temp_project = Project {
                id: project["id"].take_string().unwrap(),
                file: project["file"].take_string().unwrap(),
                title: project["title"].take_string().unwrap(),
                github_link: project["github_link"].take_string().unwrap(),
                content
            };
            projects.push(temp_project);
        }

        let template = ProjectsTemplate { projects };
        let reply_html = template.render().unwrap();

        Html(reply_html).into_response()
    } else {
        index_with_url("/projects".to_string()).await.into_response()
    }
}



#[derive(Template)]
#[template(path = "about.html")]
struct AboutTemplate {}

async fn about(headers: HeaderMap) -> impl IntoResponse {
    if headers.contains_key("HX-Request") {
        let template = AboutTemplate {};
        let reply_html = template.render().unwrap();

        Html(reply_html).into_response()
    } else {
        index_with_url("/about".to_string()).await.into_response()
    }
}
