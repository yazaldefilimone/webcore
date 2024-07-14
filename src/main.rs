use html::HTMLParser;

mod css;
mod diagnostics;
mod dom;
mod html;
mod layout;
mod network;
mod paint;
mod utils;
fn main() {
  let html = r#"
  <!DOCTYPE html>
    <html>
    <body>
      <h1>Title</h1>
      <div id="main" class="test">
        <p>Hello <em>world</em>!</p>
      </div>
    </body>
  </html>"#;
  let mut html = HTMLParser::new(html.to_string());
  let node = html.parse_root();
  println!("{:#?}", node);
}
