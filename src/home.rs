use crate::prelude::*;

#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Eq, Hash)]
pub struct MyState {
    pub hello: String,
}

#[component]
pub fn View() -> impl IntoView {
    let (state, set_state, _) = use_local_storage::<State, JsonSerdeWasmCodec>("state");
    let (show_controls, set_show_controls) = signal(false);
    let today = Local::now();

    let days = move || {
        let num_of_days = if show_controls.get() { 3 } else { 5 };
        (0..num_of_days)
            .map(|i| today - chrono::Duration::days(i))
            .rev()
            .collect::<Vec<_>>()
    };

    view! {
        <div class={move || format!("habit-list {}", show_controls.get().then(|| "show-controls").unwrap_or_default())}>
            <div />
            {move || days()
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
                            <Habit habit=habit.clone() days=days().clone() set_state=set_state show_controls=show_controls />
                        }
                    })
                    .collect::<Vec<_>>()
            }}
        </div>
        <div class="buttons">
            <a href="/habit">
                <button>"Add"</button>
            </a>
            <button on:click=move |_| set_show_controls.update(|v| *v = !*v)>"Sort"</button>
            <a href="/export">
                <button>"Export"</button>
            </a>
            <a href="/import">
                <button>"Import"</button>
            </a>
        </div>
    }
}

fn is_movable(habit_id: Uuid, move_down: bool) -> bool {
    let (state, _, _) = use_local_storage::<State, JsonSerdeWasmCodec>("state");
    let habits = state.get().habits;

    habits
        .iter()
        .position(|h| h.id == habit_id)
        .is_some_and(|index| {
            if move_down {
                index < habits.len() - 1
            } else {
                index > 0
            }
        })
}

fn update_habit_index(habit_id: Uuid, move_down: bool) {
    let (state, set_state, _) = use_local_storage::<State, JsonSerdeWasmCodec>("state");

    let mut habits = state.get().habits;

    if let Some(pos) = habits.iter().position(|h| h.id == habit_id) {
        let new_index = if move_down {
            pos.saturating_add(1)
        } else {
            pos.saturating_sub(1)
        };

        if new_index < habits.len() {
            habits.swap(pos, new_index);
            set_state.update(|state| state.habits = habits);
        }
    }
}

#[component]
fn Habit(
    habit: Habit,
    days: Vec<DateTime<Local>>,
    set_state: WriteSignal<State>,
    show_controls: ReadSignal<bool>,
) -> impl IntoView {
    let days_len = days.len();
    view! {
        <div class="caption">
            {move || show_controls.get().then(|| view! {
                <div class="position-controls">
                    <button
                        class="left"
                        on:click=move |_| { update_habit_index(habit.id, false); }
                        disabled=move || !is_movable(habit.id, false)
                    >
                        "↑"
                    </button>
                    <button
                        on:click=move |_| { update_habit_index(habit.id, true); }
                        disabled=move || !is_movable(habit.id, true)
                    >
                        "↓"
                    </button>
                </div>
            })}
            <a href=format!("/habit/{}", habit.id)>
                <p>{habit.title.clone()}</p>
            </a>
        </div>
        {days
            .into_iter()
            .enumerate()
            .map(|(n, d)| {
                let rounded = n == days_len - 1;
                let checked = habit.state_for_day(&Day::from_local_date(&d));
                let src = if checked { "/lucide-icon/check.svg" } else { "/lucide-icon/x.svg" };
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
