use neon::prelude::*;
use neon::{
    result::{JsResult, NeonResult},
    types::JsPromise,
};
use once_cell::sync::OnceCell;

use tokio::runtime::Runtime;

fn runtime<'a, C: Context<'a>>(cx: &mut C) -> NeonResult<&'static Runtime> {
    static RUNTIME: OnceCell<Runtime> = OnceCell::new();
    RUNTIME.get_or_try_init(|| Runtime::new().or_else(|err| cx.throw_error(err.to_string())))
}

pub fn async_wrapper<V, F>(mut cx: FunctionContext, fun: F) -> JsResult<JsPromise>
where
    V: Value,
    F: FnOnce(TaskContext) -> JsResult<V> + Send + 'static,
{
    let rt = runtime(&mut cx)?;
    let channel = cx.channel();

    let (deferred, promise) = cx.promise();

    rt.spawn(async move { deferred.settle_with(&channel, fun) });

    Ok(promise)
}

// //template
// fn temp_async(mut cx: FunctionContext) -> JsResult<JsPromise> {
//     let db = cx.argument::<JsBox<DBWrapper<Any>>>(0)?.deref().0.clone();

//     let rt = runtime(&mut cx)?;
//     let channel = cx.channel();

//     let (deferred, promise) = cx.promise();

//     rt.spawn(async move {
//         //Do async stuff here

//         deferred.settle_with(&channel, move |mut cx| {
//             //handle errors here

//             // return value here
//             // return Ok(cx.boxed(DBWrapper(db)));
//         })
//     });

//     Ok(promise)
// }
