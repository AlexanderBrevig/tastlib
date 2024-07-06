use crate::lex::{Pressed, PRESS_SIZE};
use heapless::Vec;

#[derive(Debug)]
pub enum ChordEvent {
    Both(Pressed, Pressed),
    On(Pressed),
    RAny,
    LAny,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Emit<T: 'static> {
    Mod(&'static Emit<T>),
    Ctrl(&'static Emit<T>),
    Shift(&'static Emit<T>),
    Alt(&'static Emit<T>),
    String(&'static str),
    Code(T),
    Identity,
}

#[derive(Debug)]
pub struct ChordEmit<T: 'static>(pub &'static [ChordEvent], pub Emit<T>);

fn rule_match(chord: &Vec<Pressed, PRESS_SIZE>, events: &[ChordEvent]) -> bool {
    let mut ixoffset = 0;
    for (ix, event) in events.iter().enumerate() {
        let ix = ix + ixoffset;
        if ix >= chord.len() {
            return false;
        }
        // println!("ix {} chrd {:?} evt {:?}", ix, chord[ix], event);
        match event {
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
                return matches!(
                    key,
                    crate::lex::Key::R1
                        | crate::lex::Key::R2
                        | crate::lex::Key::R3
                        | crate::lex::Key::R4
                        | crate::lex::Key::R5
                        | crate::lex::Key::R6
                        | crate::lex::Key::R7
                        | crate::lex::Key::R8
                        | crate::lex::Key::R9
                        | crate::lex::Key::R10
                        | crate::lex::Key::R11
                        | crate::lex::Key::R12
                        | crate::lex::Key::R13
                        | crate::lex::Key::R14
                        | crate::lex::Key::R15
                        | crate::lex::Key::R16
                        | crate::lex::Key::R17 // | crate::lex::Key::RAny
                );
            }
            ChordEvent::LAny => {
                let Pressed(key) = chord[ix];
                return matches!(
                    key,
                    crate::lex::Key::L1
                        | crate::lex::Key::L2
                        | crate::lex::Key::L3
                        | crate::lex::Key::L4
                        | crate::lex::Key::L5
                        | crate::lex::Key::L6
                        | crate::lex::Key::L7
                        | crate::lex::Key::L8
                        | crate::lex::Key::L9
                        | crate::lex::Key::L10
                        | crate::lex::Key::L11
                        | crate::lex::Key::L12
                        | crate::lex::Key::L13
                        | crate::lex::Key::L14
                        | crate::lex::Key::L15
                        | crate::lex::Key::L16
                        | crate::lex::Key::L17 // | crate::lex::Key::LAny
                );
            }
        }
    }
    true
}

pub fn parse_with<T: 'static, const RULE_SIZE: usize>(
    chord: &Vec<Pressed, PRESS_SIZE>,
    rules: [ChordEmit<T>; RULE_SIZE],
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

    use super::*;
    use crate::{
        lex::{qwerty::*, PRESS_SIZE},
        parse::ChordEmit,
        parse::ChordEvent,
        parse::ChordEvent::*,
        parse::Emit,
        parse::Emit::*,
    };

    const SHIFT_L_EVENTS: [ChordEvent; 2] = [On(D), RAny];
    const SHIFT_L: ChordEmit<u8> = ChordEmit(&SHIFT_L_EVENTS, Shift(&Identity));

    const CONTROL_SHIFT_R_EVENTS: [ChordEvent; 2] = [Both(H, J), LAny];
    const CONTROL_SHIFT_R: ChordEmit<u8> =
        ChordEmit(&CONTROL_SHIFT_R_EVENTS, Ctrl(&Shift(&Identity)));

    const Q_CODE_EVENTS: [ChordEvent; 2] = [Both(H, J), On(Q)];
    const Q_CODE: ChordEmit<u8> = ChordEmit(&Q_CODE_EVENTS, Ctrl(&Shift(&Code(42))));

    const W_STRING_EVENTS: [ChordEvent; 2] = [Both(H, J), On(W)];
    const W_STRING: ChordEmit<u8> =
        ChordEmit(&W_STRING_EVENTS, Ctrl(&Shift(&String("Hello World"))));

    // NB: order matters
    const RULES: [ChordEmit<u8>; 4] = [Q_CODE, W_STRING, CONTROL_SHIFT_R, SHIFT_L];

    #[test]
    fn single_key() {
        let mut chord: Vec<Pressed, PRESS_SIZE> = Vec::new();
        chord.push(D).unwrap();

        let emit = parse_with(&chord, RULES);
        assert_eq!(Emit::Identity, emit);
    }

    #[test]
    fn two_chord() {
        let mut chord: Vec<Pressed, PRESS_SIZE> = Vec::new();
        chord.extend_from_slice(&[D, H]).unwrap();

        let emit = parse_with(&chord, RULES);
        assert_eq!(Emit::Shift(&Emit::Identity), emit);
    }

    #[test]
    fn three_chord() {
        let mut chord: Vec<Pressed, PRESS_SIZE> = Vec::new();
        chord.extend_from_slice(&[H, J, D]).unwrap();

        let emit = parse_with(&chord, RULES);
        assert_eq!(Ctrl(&Shift(&Identity)), emit);
    }

    #[test]
    fn code_chord() {
        let mut chord: Vec<Pressed, PRESS_SIZE> = Vec::new();
        chord.extend_from_slice(&[H, J, Q]).unwrap();

        let emit = parse_with(&chord, RULES);
        assert_eq!(Ctrl(&Shift(&Code(42))), emit);
    }

    #[test]
    fn string_chord() {
        let mut chord: Vec<Pressed, PRESS_SIZE> = Vec::new();
        chord.extend_from_slice(&[H, J, W]).unwrap();

        let emit = parse_with(&chord, RULES);
        assert_eq!(Ctrl(&Shift(&String("Hello World"))), emit);
    }
}
