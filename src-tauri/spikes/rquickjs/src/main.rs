use rquickjs::{Context, Ctx, Error, Runtime};
use std::time::{Duration, Instant};

fn evaluate(ctx: Ctx<'_>, source: &str) -> Result<i32, Error> {
    ctx.eval(source)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let runtime = Runtime::new()?;
    runtime.set_interrupt_handler(Some(Box::new({
        let started = Instant::now();
        move || started.elapsed() >= Duration::from_secs(5)
    })));

    let context = Context::full(&runtime)?;
    context.with(|ctx| {
        let value = evaluate(ctx.clone(), "40 + 2")?;
        assert_eq!(value, 42);

        let error =
            evaluate(ctx.clone(), "throw new Error('spike')").expect_err("script must fail");
        assert!(error.to_string().contains("Exception") || format!("{error:?}").contains("spike"));

        let started = Instant::now();
        let interrupted = evaluate(ctx, "for (;;) {}").expect_err("loop must be interrupted");
        assert!(!interrupted.to_string().is_empty());
        assert!(started.elapsed() >= Duration::from_secs(4));
        Ok::<_, Error>(())
    })?;

    println!("rquickjs spike passed: evaluation, error propagation, interrupt handler");
    Ok(())
}
