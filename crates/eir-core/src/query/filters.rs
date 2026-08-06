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
        let Some((key, value)) = part.split_once(':') else {
            continue;
        };

        match key {
            "tag" => {
                filters.push(Filter::Tag { tag: value.into() });
            }

            "source" => {
                filters.push(Filter::Source {
                    source: value.into(),
                });
            }

            "relation" => {
                filters.push(Filter::Relationship {
                    kind: "related".into(),
                    target: value.into(),
                });
            }

            _ => {
                filters.push(Filter::Attribute {
                    key: key.into(),
                    value: value.into(),
                });
            }
        }
    }

    filters
}
