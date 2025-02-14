use crate::prelude::*;

#[derive(Params, PartialEq, Default)]
struct HabitParams {
    pub id: Option<Uuid>,
}

#[component]
pub fn View() -> impl IntoView {
    let (delete, set_delete) = signal(false);
    let navigate = use_navigate();

    let (weeks_back, set_weeks_back) = signal(36);
    let element = NodeRef::<Div>::new();
    element.on_load(|el| {
        el.set_scroll_left(100000000);
    });

    let params = use_params::<HabitParams>();
    let (state, set_state, _) = use_local_storage::<State, JsonSerdeWasmCodec>("state");
    let id = params
        .read_untracked()
        .as_ref()
        .ok()
        .map(|params| params.id)
        .flatten()
        .unwrap_or(Uuid::new_v4());

    set_state.update(move |s| {
        s.get_or_create_habit(id);
    });
    let habit = move || state.get().try_get_habit(id);
    let title = move || habit().unwrap_or_default().title.clone();

    view! {
        <div class="habit">
            {move || {
                view! {
                    <div class="input-wrapper">
                        <a href="/">
                            <button>"Back"</button>
                        </a>
                        <input
                            type="text"
                            placeholder="Habit Title"
                            prop:value=title()
                            on:input:target=move |e| {
                                log::info!("Setting title to {}", e.target().value());
                                set_state
                                    .update(move |s| {
                                        s.rename_habit(id, e.target().value());
                                    });
                            }
                        />
                    </div>
                }
            }} <div class="metrics">
                <div class="metric">
                    <p class="title">"Past 30 days"</p>
                    <p class="value">{move || habit().unwrap_or_default().metric_past_30_days()}</p>
                </div>
                <div class="metric">
                    <p class="title">"Total"</p>
                    <p class="value">{move || habit().unwrap_or_default().metric_total()}</p>
                </div>
                <div class="metric">
                    <p class="title">"Age"</p>
                    <p class="value">
                        {move || format!("{} days", habit().unwrap_or_default().metric_age())}
                    </p>
                </div>
                <div class="metric">
                    <p class="title">"Best Day"</p>
                    <p class="value">{move || habit().unwrap_or_default().metric_best_weekday()}</p>
                </div>
            </div> <div class="weeks" node_ref=element>
                <div class="weekdays">
                    <p>""</p>
                    <p>"Mo"</p>
                    <p>"Tu"</p>
                    <p>"We"</p>
                    <p>"Th"</p>
                    <p>"Fr"</p>
                    <p>"Sa"</p>
                    <p>"Su"</p>
                </div>
                {move || {
                    (0..weeks_back.get())
                        .map(|n| {
                            let habit = habit();
                            match habit {
                                None => view! { <div /> }.into_any(),
                                Some(habit) => {
                                    {
                                        view! { <WeekView n=n habit=habit set_state=set_state /> }
                                    }
                                        .into_any()
                                }
                            }
                        })
                        .collect::<Vec<_>>()
                }}

            </div> <div class="buttons">
                <button
                    class="more"
                    on:click=move |_| {
                        set_weeks_back.update(|n| *n += 12);
                    }
                >
                    "More"
                </button>
            </div> <div class="buttons">
                <button
                    class="secondary"
                    class:hidden=move || delete.get()
                    on:click=move |_| {
                        set_delete.set(true);
                    }
                >
                    "Delet Habit"
                </button>
                <button
                    class:hidden=move || !delete.get()
                    on:click=move |_| {
                        set_delete.set(false);
                    }
                >
                    "No, do not delete habit"
                </button>
                <button
                    class:hidden=move || !delete.get()
                    on:click=move |_| {
                        set_state
                            .update(|s| {
                                s.remove_habit(id);
                            });
                        navigate("/", NavigateOptions::default());
                    }
                >
                    "Yes, delete habit"
                </button>
            </div>
        </div>
    }
}

#[component]
fn WeekView(n: usize, habit: Habit, set_state: WriteSignal<State>) -> impl IntoView {
    let today = Local::now();
    let day = today - Duration::days(n as i64 * 7);
    let start_of_week = day - Duration::days(day.weekday().num_days_from_monday() as i64);
    let end_of_week = start_of_week + Duration::days(6);
    let end_of_last_week = start_of_week - Duration::days(1);

    let days = move || {
        (0..7)
            .map(|d| {
                let date: DateTime<Local> = start_of_week + Duration::days(d as i64);
                Day::from_local_date(&date)
            })
            .collect::<Vec<_>>()
    };

    let mut head = ("".to_string(), "".to_string());
    if end_of_week.month() != end_of_last_week.month() {
        head = (
            month_abbr(end_of_week.month()).to_string(),
            format!("{}", end_of_week.year() % 100),
        );
    }

    view! {
        <div class="week">
            <p class="head">{head.0}<br />{head.1}</p>
            <For
                each=move || days()
                key=|d| format!("{:?}", d)
                children=move |d| {
                    let day_clone = d.clone();
                    let checked = habit.state_for_day(&d);
                    view! {
                        <button
                            class="clickable"
                            class:today=d.is_today()
                            class:checked=checked
                            disabled=d.is_future()
                            on:click=move |_| {
                                let day_clone = day_clone.clone();
                                set_state
                                    .update(move |s| {
                                        s.toggle_day(habit.id, day_clone);
                                    });
                            }
                        >
                            {d.day_of_month}
                        </button>
                    }
                }
            />
        </div>
    }
}

fn month_abbr(mut month: u32) -> &'static str {
    if month > 12 {
        month = month % 12;
    }
    match month {
        1 => "Ja",
        2 => "Fe",
        3 => "Ma",
        4 => "Ap",
        5 => "Ma",
        6 => "Ju",
        7 => "Ju",
        8 => "Au",
        9 => "Se",
        10 => "Oc",
        11 => "No",
        12 => "De",
        _ => unreachable!(),
    }
}
