use eir_core::Engine;

pub fn run(query: String) -> anyhow::Result<()> {
    let engine = Engine::open("eir.db")?;

    let results = engine.search(&query)?;

    for result in results {
        println!("{} ({})", result.entity.id, result.score);
    }

    Ok(())
}
