use crate::prelude::*;

#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Eq, Hash)]
pub struct MyState {
    pub hello: String,
}

#[component]
pub fn View() -> impl IntoView {
    let (state, set_state, _) = use_local_storage::<State, JsonSerdeWasmCodec>("state");
    let today = Local::now();
    let days = (0..5)
        .map(|i| today - chrono::Duration::days(i))
        .rev()
        .collect::<Vec<_>>();

    view! {
        <div class="habit-list">
            <div />
            {days
                .iter()
                .map(|d| {
                    view! {
                        <div class="date" class:today=d.date_naive() == today.date_naive()>
                            <p>{d.format("%a").to_string()}</p>
                            <p>{d.day()}</p>
                        </div>
                    }
                })
                .collect::<Vec<_>>()}
            {move || {
                state
                    .get()
                    .habits
                    .iter()
                    .map(|habit| {
                        view! {
                            <Habit habit=habit.clone() days=days.clone() set_state=set_state />
                        }
                    })
                    .collect::<Vec<_>>()
            }}

        </div>
        <div class="home-buttons">
            <a href="/habit">
                <button>"Add habit"</button>
            </a>
            <a href="/export">
                <button>"Export Data"</button>
            </a>
            <a href="/import">
                <button>"Import Data"</button>
            </a>

        </div>
    }
}

#[component]
fn Habit(habit: Habit, days: Vec<DateTime<Local>>, set_state: WriteSignal<State>) -> impl IntoView {
    let days_len = days.len();
    view! {
        <a class="caption " href=format!("/habit/{}", habit.id)>
            <p>{habit.title.clone()}</p>
        </a>
        {days
            .into_iter()
            .enumerate()
            .map(|(n, d)| {
                let rounded = n == days_len - 1;
                let checked = habit.state_for_day(&Day::from_local_date(&d));
                let src = if checked { "/lucide-icons/check.svg" } else { "/lucide-icons/x.svg" };
                view! {
                    <div class:rounded=rounded class:x=!checked class="check clickable">
                        <img
                            on:click=move |_| {
                                set_state
                                    .update(|state| {
                                        state.toggle_day(habit.id, Day::from_local_date(&d));
                                    });
                            }
                            src=src
                            class="checked"
                            alt="Delete"
                        />
                    </div>
                }
            })
            .collect::<Vec<_>>()}
    }
}
