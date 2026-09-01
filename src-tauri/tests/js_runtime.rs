use reader_desktop_lib::source_engine::rule::{JsContext, JsRuntime, JsValue, QuickJsRuntime};

#[tokio::test]
async fn quickjs_runtime_executes_legado_style_script() {
    let value = QuickJsRuntime::default()
        .execute(
            "java.put('slug', result.toLowerCase()); java.get('slug')",
            JsContext {
                result: "Hello".into(),
                ..Default::default()
            },
        )
        .await
        .expect("script should execute");
    assert_eq!(value, JsValue::String("hello".into()));
}
