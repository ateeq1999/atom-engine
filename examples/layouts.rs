use atom_engine::Atom;
use serde_json::json;

fn main() {
    let mut engine = Atom::new();

    engine
        .add_template(
            "base.html",
            r#"<!DOCTYPE html>
<html>
<head>
  <title>{% block title %}My Site{% endblock %}</title>
  {% block head %}{% endblock %}
</head>
<body>
  <header>
    {% block header %}
      <h1>Welcome</h1>
    {% endblock %}
  </header>
  
  <main>
    {% block content %}{% endblock %}
  </main>
  
  <footer>
    {% block footer %}
      <p>&copy; 2024</p>
    {% endblock %}
  </footer>
</body>
</html>"#,
        )
        .unwrap();

    engine
        .add_template(
            "page.html",
            r#"{% extends "base.html" %}

{% block title %}About Us{% endblock %}

{% block content %}
  <h2>About Us</h2>
  <p>We are a company that builds great things.</p>
{% endblock %}"#,
        )
        .unwrap();

    engine
        .add_template(
            "blog.html",
            r#"{% extends "base.html" %}

{% block head %}
  <link rel="stylesheet" href="blog.css">
{% endblock %}

{% block content %}
  <h2>Blog</h2>
  <ul>
  {% for post in posts %}
    <article>
      <h3>{{ post.title }}</h3>
      <p>{{ post.excerpt }}</p>
      <a href="{{ post.url }}">Read more</a>
    </article>
  {% endfor %}
  </ul>
{% endblock %}"#,
        )
        .unwrap();

    let result = engine.render("page.html", &json!({})).unwrap();
    println!("=== Page Template ===\n{}", result);

    let result = engine
        .render(
            "blog.html",
            &json!({
                "posts": [
                    {"title": "Hello World", "excerpt": "First post", "url": "/posts/1"},
                    {"title": "Rust is Fast", "excerpt": "Second post", "url": "/posts/2"}
                ]
            }),
        )
        .unwrap();
    println!("\n=== Blog Template ===\n{}", result);

    engine
        .add_template(
            "include_example.html",
            r#"<div class="sidebar">
  {% include "sidebar.html" %}
</div>"#,
        )
        .unwrap();

    engine
        .add_template(
            "sidebar.html",
            r#"<ul>
  <li><a href="/">Home</a></li>
  <li><a href="/about">About</a></li>
</ul>"#,
        )
        .unwrap();

    let result = engine.render("include_example.html", &json!({})).unwrap();
    println!("\n=== Include Example ===\n{}", result);

    engine
        .add_template(
            "macro_example.html",
            r#"{% macro input(name, type="text", placeholder="") %}
<input type="{{ type }}" name="{{ name }}" placeholder="{{ placeholder }}">
{% endmacro %}

{% macro button(text, variant="primary") %}
<button class="btn btn-{{ variant }}">{{ text }}</button>
{% endmacro %}

<form>
  {{ input("username", placeholder="Enter username") }}
  {{ input("email", type="email", placeholder="Enter email") }}
  {{ button("Submit") }}
</form>"#,
        )
        .unwrap();

    let result = engine.render("macro_example.html", &json!({})).unwrap();
    println!("\n=== Macro Example ===\n{}", result);
}
