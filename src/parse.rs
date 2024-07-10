use crate::lex::{Key, Pressed, PRESS_SIZE};
use heapless::Vec;

#[derive(Debug, Clone, Copy)]
pub enum ChordEvent {
    Both(Pressed, Pressed),
    On(Pressed),
    Optional(&'static ChordEvent),
    RAny,
    LAny,
    Any,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Emit<T: 'static + std::marker::Copy> {
    Mod(&'static Emit<T>),
    Ctrl(&'static Emit<T>),
    Shift(&'static Emit<T>),
    Alt(&'static Emit<T>),
    String(&'static str),
    Code(T),
    Identity,
}

#[derive(Debug)]
pub struct ChordEmit<T: 'static + std::marker::Copy>(pub &'static [ChordEvent], pub Emit<T>);

fn rule_match(chord: &Vec<Pressed, PRESS_SIZE>, rule_events: &[ChordEvent]) -> bool {
    let mut ixoffset: i8 = 0;

    //TODO: skip if chord.len != events.len
    for (ix, event) in rule_events.iter().enumerate() {
        let ix = (ix as i8 + ixoffset) as usize;
        if ix >= chord.len() {
            return false;
        }
        // println!("ix {} chrd {:?} evt {:?}", ix, chord[ix], event);
        match event {
            ChordEvent::Optional(opt) => {
                let mut subchord: Vec<Pressed, PRESS_SIZE> = Vec::new();
                subchord.extend_from_slice(&chord[ix..]).unwrap(); //TODO: fix me
                let event = *(*opt);
                if !rule_match(&subchord, &[event]) {
                    ixoffset -= 1;
                }
            }
            ChordEvent::Any => {
                // NO OP
            }
            ChordEvent::Both(p1, p2) => {
                // we need to look ahead when encountering a both scenario
                if chord.len() < ix + 2 {
                    return false;
                }
                let ch1 = chord[ix];
                let ch2 = chord[ix + 1];
                if ch1 != *p1 && ch2 != *p2 && ch1 != *p2 && ch2 != *p1 {
                    return false;
                }
                ixoffset += 1;
            }
            ChordEvent::On(pressed) => {
                if *pressed != chord[ix] {
                    return false;
                }
            }
            ChordEvent::RAny => {
                let Pressed(key) = chord[ix];
                if let Key::Left(_) = key {
                    return false;
                }
            }
            ChordEvent::LAny => {
                let Pressed(key) = chord[ix];
                if let Key::Right(_) = key {
                    return false;
                }
            }
        }
    }
    true
}

pub fn parse_with<T: 'static + std::marker::Copy, const RULE_SIZE: usize>(
    chord: &Vec<Pressed, PRESS_SIZE>,
    rules: &[ChordEmit<T>; RULE_SIZE],
) -> Emit<T> {
    for rule in rules {
        if rule_match(chord, rule.0) {
            return rule.1;
        }
    }
    Emit::Identity
}

#[cfg(test)]
mod tests {
    use heapless::Vec;
    use usbd_human_interface_device::page::Keyboard;

    use super::*;
    use crate::{
        lex::{qwerty::*, KeyId, PRESS_SIZE},
        parse::{
            ChordEmit,
            ChordEvent::{self, *},
            Emit::{self, *},
        },
    };

    // chord!(SHIFT_L, 2, [On(D), RAny], Shift(&Identity));
    const SHIFT_L_EVENTS: [ChordEvent; 2] = [On(D), RAny];
    const SHIFT_L: ChordEmit<Keyboard> = ChordEmit(&SHIFT_L_EVENTS, Shift(&Identity));

    const CONTROL_SHIFT_R_EVENTS: [ChordEvent; 2] = [Both(H, J), LAny];
    const CONTROL_SHIFT_R: ChordEmit<Keyboard> =
        ChordEmit(&CONTROL_SHIFT_R_EVENTS, Ctrl(&Shift(&Identity)));

    const Q_CODE_EVENTS: [ChordEvent; 2] = [Both(H, J), On(Q)];
    const Q_CODE: ChordEmit<Keyboard> = ChordEmit(&Q_CODE_EVENTS, Ctrl(&Shift(&Code(Keyboard::A))));

    const W_STRING_EVENTS: [ChordEvent; 2] = [Both(H, J), On(W)];
    const W_STRING: ChordEmit<Keyboard> =
        ChordEmit(&W_STRING_EVENTS, Ctrl(&Shift(&String("Hello World"))));

    const OPT_CTRL_R1_EVENTS: [ChordEvent; 3] = [
        Optional(&On(Pressed(Key::Left(KeyId::K16)))),
        On(Pressed(Key::Left(KeyId::K9))),
        On(Pressed(Key::Right(KeyId::K1))),
    ];
    const OPT_CTRL_R1: ChordEmit<Keyboard> =
        ChordEmit(&OPT_CTRL_R1_EVENTS, Ctrl(&String("Optional")));

    // NB: order matters
    const RULES: [ChordEmit<Keyboard>; 5] =
        [OPT_CTRL_R1, Q_CODE, W_STRING, CONTROL_SHIFT_R, SHIFT_L];

    #[test]
    fn single_key() {
        let mut chord: Vec<Pressed, PRESS_SIZE> = Vec::new();
        chord.push(D).unwrap();

        let emit = parse_with(&chord, &RULES);
        assert_eq!(Emit::Identity, emit);
    }

    #[test]
    fn two_chord() {
        let mut chord: Vec<Pressed, PRESS_SIZE> = Vec::new();
        chord.extend_from_slice(&[D, H]).unwrap();

        let emit = parse_with(&chord, &RULES);
        assert_eq!(Emit::Shift(&Emit::Identity), emit);
    }

    #[test]
    fn three_chord() {
        let mut chord: Vec<Pressed, PRESS_SIZE> = Vec::new();
        chord.extend_from_slice(&[H, J, D]).unwrap();

        let emit = parse_with(&chord, &RULES);
        assert_eq!(Ctrl(&Shift(&Identity)), emit);
    }

    #[test]
    fn code_chord() {
        let mut chord: Vec<Pressed, PRESS_SIZE> = Vec::new();
        chord.extend_from_slice(&[H, J, Q]).unwrap();

        let emit = parse_with(&chord, &RULES);
        assert_eq!(Ctrl(&Shift(&Code(Keyboard::A))), emit);
    }

    #[test]
    fn string_chord() {
        let mut chord: Vec<Pressed, PRESS_SIZE> = Vec::new();
        chord.extend_from_slice(&[H, J, W]).unwrap();

        let emit = parse_with(&chord, &RULES);
        assert_eq!(Ctrl(&Shift(&String("Hello World"))), emit);
    }

    #[test]
    fn optional_chord() {
        let mut chord: Vec<Pressed, PRESS_SIZE> = Vec::new();
        chord
            .extend_from_slice(&[
                Pressed(Key::Left(KeyId::K16)),
                Pressed(Key::Left(KeyId::K9)),
                P,
            ])
            .unwrap();

        let emit = parse_with(&chord, &RULES);
        assert_eq!(Ctrl(&String("Optional")), emit);

        chord.clear();
        chord
            .extend_from_slice(&[
                Pressed(Key::Left(KeyId::K17)),
                Pressed(Key::Left(KeyId::K9)),
                P,
            ])
            .unwrap();

        let emit = parse_with(&chord, &RULES);
        assert_eq!(Identity, emit);
    }
}
