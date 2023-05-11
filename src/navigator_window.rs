use std::cmp::Ordering;

use tui::{
    style::{Color, Style},
    text::{Span, Spans},
    widgets::Paragraph,
};

pub fn generate_content(files: &Vec<String>) -> Paragraph {
    let mut content_as_spans: Vec<Spans> = files
        .iter()
        .map(|el| match el.chars().last() {
            Some('/') => {
                let prefix: String = el.chars().take(el.len() - 1).collect();
                let suffix = String::from("/");

                return Spans::from(vec![
                    Span::styled(prefix, Style::default().fg(Color::LightBlue)),
                    Span::styled(suffix, Style::default().fg(Color::LightRed)),
                ]);
            }
            _ => return Spans::from(Span::raw(el)),
        })
        .collect();

    // sort by folder, then name
    content_as_spans.sort_by(|a, b| {
        let spans_length = b.0.len().cmp(&a.0.len());
        if spans_length == Ordering::Equal {
            // if both spans are directories (ie. consist of 2 spans)
            let first_span_content1 = a.0.first().map(|span| span.content.to_owned());
            let first_span_content2 = b.0.first().map(|span| span.content.to_owned());

            return first_span_content1.cmp(&first_span_content2);
        } else {
            return spans_length;
        }
    });

    // first two options should be '../', './' ... not './', '../'
    content_as_spans.swap(0, 1);

    return Paragraph::new(content_as_spans);
}
