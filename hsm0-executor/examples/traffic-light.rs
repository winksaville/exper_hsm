use std::{
    collections::HashMap,
    time::{Duration, Instant}
};

use custom_logger::env_logger_init;

use hsm0_executor::{DynError, Executor, StateInfo, StateResult};


#[derive(Debug)]
enum Messages {
    #[allow(unused)]
    Initialize {
        color: LightColor,
        red_timer: Duration,
        yellow_timer: Duration,
        green_timer: Duration,
    },
    GetColor {
        tx: std::sync::mpsc::Sender<Messages>,
    },
    GetColorResponse {
        color: LightColor,
    },
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum LightColor {
    Red,
    #[allow(unused)]
    Yellow,
    #[allow(unused)]
    Green,
}

impl Default for LightColor {
    fn default() -> Self {
        Self::Red
    }
}

#[derive(Debug)]
struct InstantWrapper {
    instant: Instant,
}

impl Default for InstantWrapper {
    fn default() -> Self {
        Self {
            instant: Instant::now(),
        }
    }
}


#[derive(Debug)]
struct TrafficLight {
    color: LightColor,
    change_color_instant: InstantWrapper,
    durations: HashMap<LightColor, Duration>,
}

const MAX_STATES: usize = 4;
const IDX_BASE: usize = 0;
const IDX_GREEN: usize = 1;
const IDX_YELLOW: usize = 2;
const IDX_RED: usize = 3;

impl Default for TrafficLight {
    fn default() -> Self {
        TrafficLight { color: LightColor::Yellow,
            change_color_instant: InstantWrapper { instant: Instant::now() },
            durations: HashMap::<LightColor, Duration>::new(),
        }
    }
}

impl TrafficLight {
    pub fn new() -> Result<Executor<Self, Messages>, DynError> {
        let sm = TrafficLight::default();
        let mut sme = Executor::new(sm, MAX_STATES);

        sme.state(StateInfo::new(
            "base",
            Some(Self::base_enter),
            Self::base,
            None,
            None,
        ))
        .state(StateInfo::new(
            "green",
            Some(Self::green_enter),
            Self::green,
            None,
            Some(IDX_BASE),
        ))
        .state(StateInfo::new(
            "yellow",
            Some(Self::yellow_enter),
            Self::yellow,
            None,
            Some(IDX_BASE),
        ))
        .state(StateInfo::new(
            "red",
            Some(Self::red_enter),
            Self::red,
            None,
            Some(IDX_BASE),
        ))
        .initialize(IDX_YELLOW)
        .expect("Unexpected error initializing");

        log::trace!(
            "new: inital state={} idxs_enter_fns={:?}",
            sme.get_current_state_name(),
            sme.idxs_enter_fns
        );

        Ok(sme)
    }

    fn set_color(&mut self, color: LightColor) {
        self.color = color.clone();
        self.change_color_instant.instant = Instant::now() + *self.durations.get(&color).unwrap();
    }

    fn base_enter(&mut self, _msg: &Messages) {
        println!("initial_enter:+");
        self.durations.insert(LightColor::Red, Duration::new(10,0));
        self.durations.insert(LightColor::Yellow, Duration::new(3, 0));
        self.durations.insert(LightColor::Green, Duration::new(8, 0));
        self.set_color(self.color.clone());
        println!("initial_enter:- {:?}", self);
    }

    fn base(&mut self, msg: &Messages) -> StateResult {
        match msg {
            Messages::Initialize {
                color,
                red_timer,
                yellow_timer,
                green_timer,
            } => {
                self.durations.insert(LightColor::Red, *red_timer);
                self.durations
                    .insert(LightColor::Yellow, *yellow_timer);
                self.durations.insert(LightColor::Green, *green_timer);

                self.change_color_instant.instant =
                    Instant::now() + *self.durations.get(&color).unwrap();

                println!("initial: {:?}", self);

                //match color {
                //    LightColor::Green => StateResult::HandledTransitionTo(IDX_GREEN),
                //    LightColor::Yellow => StateResult::HandledTransitionTo(IDX_YELLOW),
                //    LightColor::Red => StateResult::HandledTransitionTo(IDX_RED),
                //}
                StateResult::Handled
            }
            Messages::GetColor { tx } => {
                tx.send(Messages::GetColorResponse {
                    color: self.color.clone(),
                }).unwrap();
                StateResult::Handled
            }
            Messages::GetColorResponse { color: _ } => {
                println!("Ignoring Messages::GetColorResponse, not allowed");
                StateResult::Handled
            }
        }
    }

    fn yellow_enter(&mut self, _msg: &Messages) {
        self.set_color(LightColor::Yellow);
    }

    fn yellow(&mut self, _msg: &Messages) -> StateResult {
        if Instant::now() > self.change_color_instant.instant {
            StateResult::NotHandledTransitionTo(IDX_RED)
        } else {
            StateResult::NotHandled
        }
    }

    fn red_enter(&mut self, _msg: &Messages) {
        println!("red_enter");
        self.set_color(LightColor::Red);
    }

    fn red(&mut self, _msg: &Messages) -> StateResult {
        //let now = Instant::now();
        //let change = &self.change_color_instant;
        //println!("red: now={:?} change={:?}", now, change);
        if Instant::now() > self.change_color_instant.instant {
            StateResult::NotHandledTransitionTo(IDX_GREEN)
        } else {
            StateResult::NotHandled
        }
    }

    fn green_enter(&mut self, _msg: &Messages) {
        self.set_color(LightColor::Green);
    }

    fn green(&mut self, _msg: &Messages) -> StateResult {
        if Instant::now() > self.change_color_instant.instant {
            StateResult::NotHandledTransitionTo(IDX_YELLOW)
        } else {
            StateResult::NotHandled
        }
    }
}

fn main() {
    println!("main");
    env_logger_init("info");
    log::info!("main:+");

    let (tx, rx) = std::sync::mpsc::channel::<Messages>();
    let mut sme = TrafficLight::new().unwrap();

    // Two Bugs:
    //   1) This does not change the defaults setup in TraficLight::new,
    //      with yellow still the fist color and it's duration is 3s.
    //   2) The number of colors is one more than the number of seconds.
    let msg = Messages::Initialize {
        color: LightColor::Green,
        red_timer: Duration::new(3, 0),
        yellow_timer: Duration::new(1, 0),
        green_timer: Duration::new(3, 0),
    };
    sme.dispatch(&msg);

    let msg = Messages::GetColor { tx };
    for i in 1..=25 {
        sme.dispatch(&msg);
        let rsp = rx.recv().unwrap();
        let color = match &rsp {
            Messages::GetColorResponse { color } => color,
            _ => panic!("Unexpected Message {rsp:?}"),
        };
        println!("{i:3}: rsp.color={color:?}");

        std::thread::sleep(Duration::new(1, 0));
    }
}
