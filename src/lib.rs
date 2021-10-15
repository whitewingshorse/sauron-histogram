// #![deny(warnings)]
use log::info;
use sauron::jss::jss;
use sauron::{html::attributes::style, prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Series<T> {
    pub name: String,
    pub color: String,
    pub values: Vec<Option<T>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Histogram {
    pub w: i32,
    pub h: i32,
    pub description: String,
    pub caption: String,
    pub labels_x: Vec<String>,
    pub series: Vec<Series<u64>>,
    /// effective width
    ew: i32,
    /// effective height
    eh: i32,
    pad_left: i32,
    pad_right: i32,
    pad_top: i32,
    pad_bottom: i32,
}

impl Histogram {
    pub fn new(
        w: i32,
        h: i32,
        description: String,
        caption: String,
        labels_x: Vec<String>,
        series: Vec<Series<u64>>,
    ) -> Self {
        let pad_left = 60;
        let pad_right = 15;
        let pad_top = 45;
        let pad_bottom = 25;
        let ew = w - pad_left - pad_right;
        let eh = h - pad_top - pad_bottom;
        Self {
            w,
            h,
            description,
            caption,
            labels_x,
            series,
            ew,
            eh,
            pad_left,
            pad_right,
            pad_top,
            pad_bottom,
        }
    }

    pub fn background_def<T>(
        &self,
    ) -> sauron::mt_dom::Node<&'static str, &'static str, &'static str, AttributeValue<T>> {
        radialGradient(
            [
                id("gradient-1"),
                gradientUnits("userSpaceOnUse"),
                cx(self.w * 2 / 3),
                cy(self.h / 2),
                r(self.h),
                gradientTransform("matrix(0.8, 0, 0, 0.4642, 0, 130)"),
            ],
            [
                stop([style("stop-color", "rgb(99, 84, 84)"), offset(0)], []),
                stop([style("stop-color", "rgb(19, 19, 19)"), offset(1)], []),
            ],
        )
    }

    pub fn background<T>(
        &self,
    ) -> sauron::mt_dom::Node<&'static str, &'static str, &'static str, AttributeValue<T>> {
        g(
            [],
            [rect(
                [
                    width(self.w),
                    height(self.h),
                    style("fill", "url(#gradient-1)"),
                ],
                [],
            )],
        )
    }

    pub fn border<T>(
        &self,
    ) -> sauron::mt_dom::Node<&'static str, &'static str, &'static str, AttributeValue<T>> {
        rect(
            [
                x(self.pad_left),
                y(self.pad_top),
                width(self.ew),
                height(self.eh),
                style("fill", "url(#pattern-2)"),
                style("fill-opacity", "0.2"),
                style("stroke", "rgb(105, 105, 104)"),
            ],
            [],
        )
    }

    pub fn styles(&self) -> String {
        jss! {
            "text": {
                font_family: "arial, monospace",
            },

            ".y-axis text, .x-axis text": {
                text_anchor: "middle",
                fill: "rgb(103, 102, 102)",
                font_size: px(12),
            },

            ".label_starwars": {
                white_space: "pre",
                font_size: px(15),
                fill: "rgb(253, 200, 39)",
                text_anchor: "end",
                word_spacing: 0,
            },

            ".label_startrek": {
                white_space: "pre",
                font_size: 15,
                fill: "rgb(33, 125, 245)",
                text_anchor: "end",
                word_spacing: 0,
            },

            "@media (max-width: 500px)": {
                ".x-axis text:nth-of-type(2n), .y-axis text:nth-of-type(2n)": {
                    transition: "opacity 1s ease-in-out",
                    opacity: 0,
                 },

                ".label_startrek, .label_starwars": {
                    font_size: percent(170),
                },
                ".y-axis text": {
                    font_size: percent(120),
                },
                ".x-axis text": {
                    font_size: percent(120),
                },
            },
        }
    }

    pub fn caption<T>(
        &self,
    ) -> Option<sauron::mt_dom::Node<&'static str, &'static str, &'static str, AttributeValue<T>>>
    {
        if self.caption.len() > 0 {
            Some(svg::tags::text(
                [
                    x(percent(50)),
                    y(30),
                    style("dominant-baseline", "middle"),
                    style("text-anchor", "middle"),
                    style("font-size", "16px"),
                    style("font-family", "inherited"),
                    style("fill", "rgb(251, 251, 251)"),
                    style("word-spacing", "0px"),
                ],
                [text(self.caption.clone())],
            ))
        } else {
            None
        }
    }

    pub fn y_axis<T>(
        &self,
    ) -> sauron::mt_dom::Node<&'static str, &'static str, &'static str, AttributeValue<T>> {
        g(
            [class("y-axis")],
            [
                svg::tags::text([y(420), x(40)], [text("0.00%")]),
                svg::tags::text([y(375), x(40)], [text("0.02%")]),
                svg::tags::text([y(330), x(40)], [text("0.04%")]),
                svg::tags::text([y(285), x(40)], [text("0.06%")]),
                svg::tags::text([y(240), x(40)], [text("0.08%")]),
                svg::tags::text([y(195), x(40)], [text("0.010%")]),
                svg::tags::text([y(150), x(40)], [text("0.012%")]),
                svg::tags::text([y(105), x(40)], [text("0.014%")]),
                svg::tags::text([y(60), x(40)], [text("0.016%")]),
            ],
        )
    }

    pub fn x_axis_labels<T>(
        &self,
    ) -> sauron::mt_dom::Node<&'static str, &'static str, &'static str, AttributeValue<T>> {
        let yy = self.h - self.pad_bottom + 15;
        let dx = (self.ew / self.labels_x.len() as i32) as usize;
        let xoffs = self.pad_left as usize + 20;
        let children: Vec<
            sauron::mt_dom::Node<&'static str, &'static str, &'static str, AttributeValue<T>>,
        > = self
            .labels_x
            .iter()
            .enumerate()
            .map(|(i, s)| svg::tags::text([y(yy), x(xoffs + i * dx)], [text(s.clone())]))
            .collect();
        g([class("x-axis")], children)
    }

    pub fn view<T>(&self) -> Node<T> {
        let mut children = vec![
            desc([], [text(self.description.clone())]),
            defs(
                [],
                [
                    self.background_def(),
                    html::tags::style([], [html::text(self.styles())]),
                ],
            ),
            self.background(),
            self.border(),
            self.x_axis_labels(),
            self.y_axis(),
        ];
        self.caption().iter().for_each(|c| children.push(c.clone()));
        svg(
            [
                viewBox([0, 0, self.w, self.h]),
                xmlns("http://www.w3.org/2000/svg"),
            ],
            children,
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Screen {
    /// server side state
    pub chart: Histogram,
}

impl Screen {
    pub fn new(chart: Histogram) -> Self {
        Self {
            chart: chart.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Msg {}

impl Application<Msg> for Screen {
    fn view(&self) -> Node<Msg> {
        node!(<div style="width: 800px; margin: 0px auto;">{self.chart.view()}</div>)
    }
    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        info!("MSG: {:?}", msg);
        Cmd::none()
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_log::init_with_level(log::Level::Trace).unwrap();
    console_error_panic_hook::set_once();

    let s_minted = Series {
        name: "Minted".to_owned(),
        color: "#ff8800".to_owned(),
        values: vec![
            Some(261061),
            Some(323513),
            Some(349766),
            Some(364933),
            Some(392283),
            Some(410370),
            Some(439536),
            Some(434376),
            Some(421039),
            Some(413656),
            Some(410692),
            Some(404840),
            Some(400168),
            Some(388520),
        ],
    };
    let s_staked = Series {
        name: "Staked".to_owned(),
        color: "#ffaa88".to_owned(),
        values: vec![
            Some(35129025),
            Some(42437593),
            Some(44755393),
            Some(45577661),
            Some(47847475),
            Some(48909435),
            Some(51214954),
            Some(51770573),
            Some(52194485),
            Some(52584924),
            Some(52930614),
            Some(53873470),
            Some(54476245),
            Some(55273992),
            Some(55125308),
        ],
    };
    let appstate = Histogram::new(
        800,
        400,
        "Histogram Example".to_owned(),
        "Rewards Distribution".to_owned(),
        vec![
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
        vec![s_staked, s_minted],
    );
    // if serialized_state.len() > 0 {
    //     match serde_json::from_str::<AppState>(&serialized_state) {
    //         Ok(state) => {
    //             info!("parsing state ok");
    //             appstate = state;
    //         }
    //         Err(e) => {
    //             info!("parsing error {}", e);
    //         }
    //     };
    // }
    // let document = sauron::dom::document();
    // let root = document.query_selector_all("main").unwrap().get(0).unwrap();

    Program::mount_to_body(Screen::new(appstate));
}
