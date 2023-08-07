use yew::prelude::*;
use chrono::Utc;
use chrono_tz::{
    Asia::Tokyo,
    America::Toronto,
    Europe::Amsterdam,
};

use crate::components::*;

#[function_component]
pub fn App() -> Html {
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