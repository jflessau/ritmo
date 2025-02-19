mod export;
mod habit;
mod home;
mod import;
mod prelude;

use prelude::*;

fn main() {
    provide_meta_context();
    console_log::init_with_level(Level::Debug).expect("Failed to initialize logger");
    leptos::mount::mount_to_body(|| view! { <App /> })
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    view! {
        // meta info
        <Title text="Ritmo" />
        <meta name="viewport" content="width=device-width, initial-scale=1, maximum-scale=1" />

        <link rel="apple-touch-icon" sizes="180x180" href="/favicon/apple-touch-icon.png" />
        <link rel="icon" type="image/png" sizes="32x32" href="/favicon/favicon-32x32.png" />
        <link rel="icon" type="image/png" sizes="16x16" href="/favicon/favicon-16x16.png" />
        <link rel="manifest" href="/favicon/site.webmanifest" />
        <link rel="mask-icon" href="/favicon/safari-pinned-tab.svg" />

        <Router>
            <main>
                <Routes fallback=|| "Not found.">
                    <Route path=path!("") view=home::View />
                    <Route path=path!("/export") view=export::View />
                    <Route path=path!("/import") view=import::View />
                    <Route path=path!("/habit/:id") view=habit::View />
                    <Route path=path!("/habit") view=habit::View />
                </Routes>
            </main>
        </Router>
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct State {
    pub habits: Vec<Habit>,
}

impl std::default::Default for State {
    fn default() -> Self {
        Self {
            habits: vec![
                Habit {
                    id: Uuid::new_v4(),
                    title: "Drink water".to_string(),
                    days: vec![],
                },
                Habit {
                    id: Uuid::new_v4(),
                    title: "Read a book".to_string(),
                    days: vec![Day::today()],
                },
            ],
        }
    }
}

impl State {
    pub fn add_habit(&mut self, habit: Habit) {
        self.habits.push(habit.clone());
    }

    pub fn remove_habit(&mut self, id: Uuid) {
        self.habits.retain(|habit| habit.id != id);
    }

    pub fn toggle_day(&mut self, habit_id: Uuid, day: Day) {
        log::info!("in fn toggle_day: habit_id: {:?}, day: {:?}", habit_id, day);
        if let Some(habit) = self.habits.iter_mut().find(|habit| habit.id == habit_id) {
            if habit.days.iter().find(|d| d == &&day).is_some() {
                habit.days.retain(|d| d != &day);
            } else {
                habit.days.push(day);
            }
        }
    }

    pub fn rename_habit(&mut self, habit_id: Uuid, title: String) {
        log::info!("in fn rename_habit");
        if let Some(habit) = self.habits.iter_mut().find(|habit| habit.id == habit_id) {
            habit.title = title;
        }
    }

    pub fn try_get_habit(&self, id: Uuid) -> Option<Habit> {
        self.habits.iter().find(|habit| habit.id == id).cloned()
    }

    pub fn get_or_create_habit(&mut self, id: Uuid) -> Habit {
        let existing_habit = self.try_get_habit(id);
        existing_habit.unwrap_or_else(|| {
            let mut habit = Habit::default();
            habit.id = id;
            self.habits.push(habit.clone());
            habit
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Habit {
    pub id: Uuid,
    pub title: String,
    pub days: Vec<Day>,
}

impl Default for Habit {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            title: "New Habit".to_string(),
            days: vec![],
        }
    }
}

impl Habit {
    pub fn state_for_day(&self, day: &Day) -> bool {
        self.days.iter().find(|d| d == &day).is_some()
    }

    pub fn metric_past_30_days(&self) -> u32 {
        let today = Day::today();
        let thirty_days_ago = today.local_date() - chrono::Duration::days(30);
        self.days
            .iter()
            .filter(|day| day.local_date() >= thirty_days_ago)
            .count() as u32
    }

    pub fn metric_total(&self) -> u32 {
        self.days.len() as u32
    }

    pub fn metric_age(&self) -> u32 {
        let today = Day::today();
        let first_day = self.days.iter().min().unwrap_or(&today);
        let days = today
            .local_date()
            .signed_duration_since(first_day.local_date())
            .num_days();
        days as u32
    }

    pub fn metric_best_weekday(&self) -> String {
        let mut days = vec![0; 7];
        for day in &self.days.iter().collect::<Vec<_>>() {
            days[day.local_date().weekday().num_days_from_monday() as usize] += 1;
        }
        let best_day = days
            .iter()
            .enumerate()
            .filter(|(_, &count)| count > 0)
            .max_by_key(|(_, &count)| count)
            .map(|(i, _)| i)
            .unwrap_or(7);
        match best_day {
            0 => "Monday".to_string(),
            1 => "Tuesday".to_string(),
            2 => "Wednesday".to_string(),
            3 => "Thursday".to_string(),
            4 => "Friday".to_string(),
            5 => "Saturday".to_string(),
            6 => "Sunday".to_string(),
            _ => "No data".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialOrd, PartialEq, Eq, Hash)]
pub struct Day {
    pub day_of_month: u32,
    pub month: u32,
    pub year: i32,
}

impl Day {
    fn today() -> Self {
        Self::from_local_date(&Local::now())
    }

    fn is_today(&self) -> bool {
        self == &Self::today()
    }

    fn is_future(&self) -> bool {
        self.local_date() > Local::now()
    }

    fn from_local_date(date: &DateTime<Local>) -> Self {
        Self {
            day_of_month: date.day(),
            month: date.month(),
            year: date.year(),
        }
    }

    fn local_date(&self) -> DateTime<Local> {
        let mut r = Local::now();
        r = r.with_year(self.year).expect("Invalid year");
        r = r.with_month(self.month).expect("Invalid month");
        r = r.with_day(self.day_of_month).expect("Invalid day");
        r
    }
}

impl std::cmp::Ord for Day {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.year != other.year {
            return self.year.cmp(&other.year);
        }
        if self.month != other.month {
            return self.month.cmp(&other.month);
        }
        self.day_of_month.cmp(&other.day_of_month)
    }
}
