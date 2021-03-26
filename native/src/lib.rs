use num_cpus;
use neon::prelude::*;

fn thread_count(mut ctx: FunctionContext) -> JsResult<JsNumber> {
    Ok(ctx.number(num_cpus::get() as f64))
}

register_module!(mut m, {
    m.export_function("threadCount", thread_count)
});
