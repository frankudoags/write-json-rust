use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Paragraph{
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Article{
    title: String,
    author: String,
    paragraphs: Vec<Paragraph>,
}

fn main() {
    let article = Article{
        title: "How to write json in Rust".to_string(),
        author: "Frank Udoags".to_string(),
        paragraphs: vec![
            Paragraph{name: "Introduction".to_string()},
            Paragraph{name: "The code".to_string()},
            Paragraph{name: "Conclusion".to_string()},
        ],
    };

    let json = serde_json::to_string(&article).unwrap();

    println!("{}", json);
}