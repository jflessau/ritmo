use crate::prelude::*;

#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Eq, Hash)]
pub struct MyState {
    pub hello: String,
}

#[component]
pub fn View() -> impl IntoView {
    let (state, _, _) = use_local_storage::<State, JsonSerdeWasmCodec>("state");

    view! {
        <div class="export-import">
            <textarea rows=10>
                {move || { serde_json::to_string_pretty(&state.get().habits).unwrap() }}
            </textarea>
            <div class="buttons">
                <a href="/">
                    <button>"Back"</button>
                </a>
            </div>
        </div>
    }
}
