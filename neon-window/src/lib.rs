use neon::prelude::*;

fn open_window(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("return from rust"))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("open_window", open_window)?;
    Ok(())
}
