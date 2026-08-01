use anyhow::Result;

use crate::loader::*;

use eir_core::entity::*;

pub fn generate_entities() -> Result<Vec<Entity>> {
    let products = load_products("fixtures/products.json")?;

    let entities = products
        .into_iter()
        .map(|p| {
            Entity {
                id: p.id,

                names: vec![
                    p.name.into(),
                    // aliases added here
                ],

                tags: p.tags,

                properties: vec![],

                relationships: vec![p.company, p.category, p.country],

                sources: vec![],
            }
        })
        .collect();

    Ok(entities)
}
