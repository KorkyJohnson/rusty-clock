use chrono::Utc;
use chrono_tz::{America::Toronto, Asia::Tokyo, Europe::Amsterdam};
use yew::prelude::*;

use gloo::timers::callback::Interval;

use crate::components::*;

#[function_component]
pub fn App() -> Html {

    // owned by the App.rs function
    let time = use_state(|| Utc::now());

    // Since we want to use timer in our HTML we cannot "move" it to
    // to the clouser. Let's borrow it by cloning its reference
    let time_state: UseStateHandle<chrono::DateTime<Utc>> = time.clone();

    use_effect_with_deps(
        |_| {
            // Create interval
            let interval = Interval::new(1000, move || time_state.set(Utc::now()));

            // And pass it to the clean up closer to keep it alive
            || drop(interval)
        },
        (),
    );

    html! {
        <div class="board">
            <div>
                <Clock time={Utc::now().with_timezone(&Tokyo)} />
                <Plate>{"Tokyo"}</Plate>
            </div>
            <div>
                <Clock time={Utc::now().with_timezone(&Toronto)} />
                <Plate>{"Ottawa"}</Plate>
            </div>

            <div>
                <Clock time={Utc::now().with_timezone(&Amsterdam)} />
                <Plate>{"Schrepennisse"}</Plate>
            </div>
        </div>
    }
}
