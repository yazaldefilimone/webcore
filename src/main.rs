use html::HTMLParser;

#[allow(dead_code)]
mod css;
mod dom;
mod html;
mod layout;
mod paint;
mod utils;
fn main() {
  let html = r#"<html>
    <body>
      <h1>Title</h1>
      <div id="main" class="test">
        <p>Hello <em>world</em>!</p>
      </div>
    </body>
  </html>"#;
  let mut html = HTMLParser::new(html.to_string());
  let node = html.parse();
  println!("{:#?}", node);
}
