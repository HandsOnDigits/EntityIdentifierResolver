#[derive(Debug, Clone)]
pub enum Filter {
    Attribute { key: Box<str>, value: Box<str> },

    Tag { tag: Box<str> },

    Source { source: Box<str> },

    Relationship { kind: Box<str>, target: Box<str> },

    EntityType { kind: Box<str> },
}

pub fn parse_filters(query: &str) -> Vec<Filter> {
    let mut filters = Vec::new();

    for part in query.split_whitespace() {
        let parts: Vec<&str> = part.split(':').collect();

        if parts.len() == 3 && parts[0] == "relation" {
            filters.push(Filter::Relationship {
                kind: parts[1].into(),
                target: parts[2].into(),
            });

            continue;
        }

        if let Some((key, value)) = part.split_once(':') {
            filters.push(Filter::Attribute {
                key: key.into(),
                value: value.into(),
            });
        }
    }

    filters
}
