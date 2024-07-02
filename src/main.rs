#[macro_use] extern crate rocket;
use rocket::fs::NamedFile;
use rocket_dyn_templates::{Template, context};
use rocket_ws::{WebSocket, Stream};
use safe_path::scoped_join;
use std::path::PathBuf;

mod load_md;
mod parse_md;

#[get("/<path..>", rank = 2, format="markdown")]
async fn markdown_version(path: PathBuf) -> Option<String> {
    load_md::load("./pages", path).await
}

#[get("/<path..>", rank = 3)]
async fn page(path: PathBuf) -> Option<Template> {
    let md_src = load_md::load("./pages", path).await?;
    let md = parse_md::parse(&md_src);
    let template = md.meta.template.clone();

    if md.meta.visibility == "public" {
        Some(Template::render(template, md))
    } else {
        None
    }
}

#[get("/static/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(scoped_join("./static", file).ok()?).await.ok()
}

#[get("/favicon.ico")]
async fn favicon() -> Option<NamedFile> {
    NamedFile::open("./static/favicon.ico").await.ok()
}

#[get("/resume")]
async fn resume() -> Option<Template> {
    let path = "Resume/resume";
    let md_src = load_md::load("./pages", path).await?;
    let md_html = markdown::to_html(&md_src);
    Some(Template::render("resume", context! { content: md_html }))
}

#[get("/stream")]
fn watch_stream_md(ws: WebSocket) -> Stream!['static] {
    Stream! { ws =>
        for await _message in ws {
            let md_src = load_md::load("./pages", "stream").await;
            if md_src.is_some() {
                let md = parse_md::parse(md_src.unwrap().as_str());
                yield md.html.into();
            }
        }
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![
            markdown_version,
            page,
            files,
            favicon,
            resume,
            watch_stream_md,
        ])
        .attach(Template::fairing())
}

