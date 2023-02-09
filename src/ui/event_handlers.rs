use bevy::prelude::*;
use kayak_ui::prelude::*;
use crate::state::GameState;
use crate::ui::EventInput;

#[derive(Clone, Debug)]
pub enum StateTransition {
    Pop,
    Set(GameState)
}

pub fn goto_state_event(trans: StateTransition) -> OnEvent {
    OnEvent::new(move |
        In((event_dispatcher_context, _, event, _entity)): EventInput,
        mut state: ResMut<State<GameState>>,
    | {
        match event.event_type {
            EventType::Click(_) => {
                match trans.clone() {
                    StateTransition::Pop => state.pop().unwrap(),
                    StateTransition::Set(new) => state.overwrite_set(new).unwrap()
                }
            }

            _ => {}
        }

        (event_dispatcher_context, event)
    })
}