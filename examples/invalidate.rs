use std::sync::Mutex;

use turbosloth::*;

static REFORBLE: Mutex<Option<Box<dyn Fn() + Send + Sync>>> = Mutex::new(None);

#[derive(Clone, Hash)]
struct Forble;

#[async_trait]
impl LazyWorker for Forble {
    type Output = anyhow::Result<String>;

    async fn run(self, ctx: RunContext) -> Self::Output {
        (*REFORBLE.lock().unwrap()) = Some(Box::new(ctx.get_invalidation_trigger()));

        println!("Forbling");
        Ok("forble".to_owned())
    }
}

#[derive(Clone, Hash)]
struct Borble {
    forble: Lazy<String>,
}

#[async_trait]
impl LazyWorker for Borble {
    type Output = anyhow::Result<String>;

    async fn run(self, ctx: RunContext) -> Self::Output {
        let forble = self.forble.eval(&ctx).await?;
        println!("Borbling the forble");
        Ok((*forble).clone() + "borble")
    }
}

fn main() -> anyhow::Result<()> {
    let cache = LazyCache::create();

    let boop = Borble {
        forble: Forble.into_lazy(),
    }
    .into_lazy();
    dbg!(smol::block_on(boop.eval(&cache))?);
    dbg!(smol::block_on(boop.eval(&cache))?);

    println!("Invalidating the forble!");
    (REFORBLE.lock().unwrap().as_ref().unwrap())();

    dbg!(smol::block_on(boop.eval(&cache))?);

    Ok(())
}
