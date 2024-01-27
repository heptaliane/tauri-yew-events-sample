use serde::Deserialize;
use serde_wasm_bindgen::from_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "event"])]
    async fn listen(event: &str, handler: &Closure<dyn FnMut(JsValue)>) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "event"])]
    async fn emit(event: &str, payload: JsValue);
}

#[derive(Deserialize)]
struct Payload {
    payload: String,
}

#[function_component(App)]
pub fn app() -> Html {
    let msg = use_state(|| String::new());

    {
        let msg = msg.clone();
        spawn_local(async move {
            let handler = Closure::<dyn FnMut(JsValue)>::new(move |payload: JsValue| {
                if let Ok(res) = from_value::<Payload>(payload) {
                    msg.set(res.payload);
                }
            });
            let _ = listen("back-to-front", &handler).await;
            handler.forget();
        });
    }

    use_effect_with_deps(
        move |_| {
            spawn_local(async move {
                emit("front-to-back", JsValue::NULL).await;
            });
        },
        (),
    );

    html! {
        <span>{(*msg).clone()}</span>
    }
}
