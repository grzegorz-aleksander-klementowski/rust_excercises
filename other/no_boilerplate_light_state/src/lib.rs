struct Light<State> {
    state: State,
}

struct On;
struct Off;

impl Light<Off> {
    fn new() -> Self {
        Light { state: Off }
    }
    fn turn_on(self) -> Light<On> {
        Light { state: On }
    }
}
impl Light<On> {
    fn turn_off(self) -> Light<Off> {
        Light { state: Off }
    }
}

pub fn correct_transition() {
    let bedroom_light = Light::new();
    let bedroom_light = bedroom_light.turn_on().turn_off().turn_on();
}
