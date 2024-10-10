use scraper::{Html, Selector, ElementRef, Node};

fn extract_text_from_vec(html_snippets: &Vec<&str>) -> Vec<String> {
    html_snippets
        .iter()
        .map(|snippet| {
            let document = Html::parse_fragment(snippet);
            document.root_element().text().collect::<Vec<_>>().join(" ")
        })
        .collect()
}



pub fn extract_text_with_formatting(html_snippets: &Vec<&str>) -> Vec<String> {
    html_snippets
        .iter()
        .map(|snippet| {
            let document = Html::parse_fragment(snippet);
            let mut extracted_text = String::new();
            let root = document.root_element();
            process_node(&root, &mut extracted_text);
            extracted_text.trim().to_string()
        })
        .collect()
}

fn process_node(node: &ElementRef, output: &mut String) {
    for child in node.children() {
        match child.value() {
            Node::Text(text) => {
                output.push_str(text);
            }
            Node::Element(_) => {
                // Wrap the child node into an ElementRef
                if let Some(element_ref) = ElementRef::wrap(child) {
                    let tag_name = element_ref.value().name();

                    // Recursively process the child element
                    process_node(&element_ref, output);

                    // Insert newline after block-level elements
                    if is_block_element(tag_name) {
                        output.push('\n');
                    }
                }
            }
            _ => {}
        }
    }
}

fn is_block_element(tag_name: &str) -> bool {
    matches!(
        tag_name,
        "p" | "div" | "li" | "ul" | "ol" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6"
    )
}

