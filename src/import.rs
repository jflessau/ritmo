use crate::prelude::*;

#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Eq, Hash)]
pub struct MyState {
    pub hello: String,
}

#[component]
pub fn View() -> impl IntoView {
    let (state, set_state, _) = use_local_storage::<State, JsonSerdeWasmCodec>("state");
    let (text, text_set) = signal("".to_string());
    let (import, set_import) = signal(Import::Init);

    view! {
        <div class="export-import">
            <textarea
                placeholder="Paste your json data here.\nHave a look at the \"Export Data\" page to see the format."
                prop:value=move || text.get()
                on:input:target=move |ev| text_set.update(|t| *t = ev.target().value())
            >
                {text.get_untracked()}
            </textarea>
            <p class="import-status" class:hidden=move || { import.get() == Import::Init }>
                {move || match import.get() {
                    Import::Init => "Waiting for input.".to_string(),
                    Import::Error(e) => format!("Error: {}", e),
                    Import::Success => "Import successful.".to_string(),
                }}
            </p>
            <div class="buttons">
                <a href="/">
                    <button>"Back"</button>
                </a>

                <button
                    class:hidden=move || import.get() == Import::Success
                    disabled=move || { text.get().trim().is_empty() }
                    on:click=move |_| {
                        let data = text.get();
                        match serde_json::from_str::<Vec<Habit>>(&data) {
                            Ok(habits) => {
                                log::info!("Imported state: {:?}", state);
                                set_state
                                    .update(move |s| {
                                        *s = {
                                            let mut s = State::default();
                                            s.habits = habits;
                                            s
                                        };
                                    });
                                set_import.update(|s| *s = Import::Success);
                            }
                            Err(e) => {
                                log::error!("Error importing data: {:?}", e);
                                set_import.update(|s| *s = Import::Error(e.to_string()));
                            }
                        }
                    }
                >

                    "Import"
                </button>
            </div>
        </div>
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Import {
    Init,
    Error(String),
    Success,
}
