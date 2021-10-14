use sauron::prelude::*;
use serde::{Deserialize, Serialize};

#[macro_use]
extern crate log;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Series<T> {
    pub name: String,
    pub color: String,
    pub values: Vec<T>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    pub labels_x: Vec<String>,
    pub series: Vec<Series<u64>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Screen {
    /// server side state
    pub state: AppState,
}

impl Screen {
    pub fn new(state: AppState) -> Self {
        Self {
            state: state.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Msg {}

impl Component<Msg> for Screen {
    fn view(&self) -> Node<Msg> {
        node! {
            <div class="screen-home">
                <h1>"Histogram Example"</h1>
            </div>
        }
    }
    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        info!("MSG: {:?}", msg);
        Cmd::none()
    }
}

#[wasm_bindgen]
pub fn main(serialized_state: String) {
    console_log::init_with_level(log::Level::Trace).unwrap();
    console_error_panic_hook::set_once();

    let s_minted = Series {
        name: "Minted".to_owned(),
        color: "#ff8800".to_owned(),
        values: vec![
            261061, 323513, 349766, 364933, 392283, 410370, 439536, 434376, 421039, 413656, 410692,
            404840, 400168, 388520,
        ],
    };
    let s_staked = Series {
        name: "Staked".to_owned(),
        color: "#ffaa88".to_owned(),
        values: vec![
            35129025, 42437593, 44755393, 45577661, 47847475, 48909435, 51214954, 51770573,
            52194485, 52584924, 52930614, 53873470, 54476245, 55273992, 55125308,
        ],
    };
    let mut appstate = AppState {
        labels_x: vec![
            "Jul 14".to_owned(),
            "Jul 21".to_owned(),
            "Jul 29".to_owned(),
            "Aug 4".to_owned(),
            "Aug 11".to_owned(),
            "Aug 18".to_owned(),
            "Aug 25".to_owned(),
            "Sep 1".to_owned(),
            "Sep 8".to_owned(),
            "Sep 15".to_owned(),
            "Sep 22".to_owned(),
            "Sep 29".to_owned(),
            "Oct 6".to_owned(),
            "Oct 13".to_owned(),
            "Oct 20".to_owned(),
        ],
        series: vec![s_staked, s_minted],
    };
    if serialized_state.len() > 4 {
        match serde_json::from_str::<AppState>(&serialized_state) {
            Ok(state) => {
                info!("parsing state ok");
                appstate = state;
            }
            Err(e) => {
                info!("parsing error {}", e);
            }
        };
    }
    let document = sauron::dom::document();
    let root = document.query_selector_all("main").unwrap().get(0).unwrap();

    Program::replace_mount(Screen::new(appstate), &root);
}
