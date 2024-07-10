use heapless::Vec;
use usbd_human_interface_device::page::Keyboard as Keyb;

use crate::{
    lex::{chord, Event, Key, Pressed, PRESS_SIZE, REPORT_SIZE, STACK_SIZE},
    parse::{parse_with, ChordEmit, Emit},
};

pub fn eval<const RULE_SIZE: usize>(
    stack: &mut Vec<Event, STACK_SIZE>,
    rules: &[ChordEmit<Keyb>; RULE_SIZE],
) -> Vec<Keyb, REPORT_SIZE> {
    let mut keyboard: Vec<Keyb, REPORT_SIZE> = Vec::new();

    let chrd = chord(stack);

    if chrd.is_empty() {
        return keyboard;
    }

    let emit = parse_with(&chrd, rules);

    let identity = if chrd.len() > 1 {
        let mut identity_chord: Vec<Pressed, PRESS_SIZE> = Vec::new();
        let last = chrd.last().unwrap();
        identity_chord.push(*last).unwrap();
        parse_with(&identity_chord, rules)
    } else {
        emit
    };
    let Pressed(first) = chrd.first().unwrap();
    build_keyboard_report(emit, identity, first, &mut keyboard);
    keyboard
}

fn build_keyboard_report(
    emit: Emit<Keyb>,
    identity: Emit<Keyb>,
    first: &Key,
    keyboard: &mut Vec<Keyb, REPORT_SIZE>,
) {
    let emit = build_keyboard_report_modifiers(emit, first, keyboard);
    build_keyboard_report_identity(emit, identity, keyboard);
}

fn build_keyboard_report_modifiers(
    emit: Emit<Keyb>,
    first: &Key,
    keyboard: &mut Vec<Keyb, REPORT_SIZE>,
) -> Emit<Keyb> {
    match emit {
        Emit::Mod(next) => {
            if let Key::Left(_) = first {
                keyboard.push(Keyb::LeftGUI).unwrap();
            } else {
                keyboard.push(Keyb::RightGUI).unwrap();
            }
            build_keyboard_report_modifiers(*next, first, keyboard)
        }
        Emit::Alt(next) => {
            if let Key::Left(_) = first {
                keyboard.push(Keyb::LeftAlt).unwrap();
            } else {
                keyboard.push(Keyb::RightAlt).unwrap();
            }
            build_keyboard_report_modifiers(*next, first, keyboard)
        }
        Emit::Shift(next) => {
            if let Key::Left(_) = first {
                keyboard.push(Keyb::LeftShift).unwrap();
            } else {
                keyboard.push(Keyb::RightShift).unwrap();
            }
            build_keyboard_report_modifiers(*next, first, keyboard)
        }
        Emit::Ctrl(next) => {
            if let Key::Left(_) = first {
                keyboard.push(Keyb::LeftControl).unwrap();
            } else {
                keyboard.push(Keyb::RightControl).unwrap();
            }
            build_keyboard_report_modifiers(*next, first, keyboard)
        }
        _ => emit,
    }
}

fn report_from_chr(chr: char) -> Keyb {
    let mut keyboard: Vec<Keyb, REPORT_SIZE> = Vec::new();
    if chr.is_uppercase() {
        keyboard.push(Keyb::LeftShift).unwrap();
    }
    match chr.to_lowercase().last().unwrap() {
        'h' => Keyb::H,
        'e' => Keyb::E,
        'l' => Keyb::L,
        'o' => Keyb::O,
        // TODO: add more keys for Emit::String
        _ => Keyb::NoEventIndicated,
    }
}

fn build_keyboard_report_identity(
    emit: Emit<Keyb>,
    identity: Emit<Keyb>,
    keyboard: &mut Vec<Keyb, REPORT_SIZE>,
) {
    match emit {
        Emit::String(str) => {
            for chr in str.chars() {
                keyboard.push(report_from_chr(chr)).unwrap();
                keyboard.push(Keyb::Out).unwrap();
            }
        }
        Emit::Code(code) => {
            keyboard.push(code).unwrap();
        }
        Emit::Identity => {
            build_keyboard_report_identity(identity, identity, keyboard);
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use crate::lex::KeyId;
    use crate::parse::Emit::*;
    use crate::{
        lex::{Key, REPORT_SIZE},
        parse::Emit,
        report::{
            build_keyboard_report, build_keyboard_report_identity, build_keyboard_report_modifiers,
        },
    };
    use heapless::Vec;
    use usbd_human_interface_device::page::Keyboard as Keyb;

    #[test]
    fn test_report_identity() {
        let mut keyboard: Vec<Keyb, REPORT_SIZE> = Vec::new();
        let emit = crate::parse::Emit::Identity;
        let identity = Emit::Code(Keyb::Q);
        build_keyboard_report_identity(emit, identity, &mut keyboard);
        assert_eq!(Keyb::Q, keyboard[0]);
    }

    #[test]
    fn test_report_string() {
        let mut keyboard: Vec<Keyb, REPORT_SIZE> = Vec::new();
        let emit = Emit::String("Hello");
        let identity = crate::parse::Emit::Identity;
        build_keyboard_report_identity(emit, identity, &mut keyboard);
        assert_eq!(Keyb::H, keyboard[0]);
        assert_eq!(Keyb::Out, keyboard[1]);
        assert_eq!(Keyb::E, keyboard[2]);
        assert_eq!(Keyb::Out, keyboard[3]);
        assert_eq!(Keyb::L, keyboard[4]);
        assert_eq!(Keyb::Out, keyboard[5]);
        assert_eq!(Keyb::L, keyboard[6]);
        assert_eq!(Keyb::Out, keyboard[5]);
        assert_eq!(Keyb::O, keyboard[8]);
    }

    #[test]
    fn test_report_code() {
        let mut keyboard: Vec<Keyb, REPORT_SIZE> = Vec::new();
        let emit = Emit::Identity;
        let identity = Emit::Code(Keyb::A);
        build_keyboard_report_identity(emit, identity, &mut keyboard);
        assert_eq!(Keyb::A, keyboard[0]);
    }

    #[test]
    fn test_report_modifiers_left() {
        let mut keyboard: Vec<Keyb, REPORT_SIZE> = Vec::new();
        let emit = Emit::Shift(&Emit::Identity);
        let first = &crate::lex::Key::Left(KeyId::K8);
        let emit = build_keyboard_report_modifiers(emit, first, &mut keyboard);
        assert_eq!(Keyb::LeftShift, keyboard[0]);
        assert_eq!(Emit::Identity, emit);
    }

    #[test]
    fn test_report_modifiers_right() {
        let mut keyboard: Vec<Keyb, REPORT_SIZE> = Vec::new();
        let emit = Emit::Ctrl(&Emit::Identity);
        let first = &crate::lex::Key::Right(KeyId::K8);
        let emit = build_keyboard_report_modifiers(emit, first, &mut keyboard);
        assert_eq!(Keyb::RightControl, keyboard[0]);
        assert_eq!(Emit::Identity, emit);
    }

    #[test]
    fn test_build_keyboard_report_shift() {
        let mut keyboard: Vec<Keyb, REPORT_SIZE> = Vec::new();
        let emit = Emit::Shift(&Emit::Identity);
        let first = &Key::Right(KeyId::K6); // right gui
        let identity = Emit::Code(Keyb::Keyboard1);
        build_keyboard_report(emit, identity, first, &mut keyboard);
        assert_eq!(Keyb::RightShift, keyboard[0]);
        assert_eq!(Keyb::Keyboard1, keyboard[1]);
    }

    #[test]
    fn test_build_keyboard_report_god() {
        let mut keyboard: Vec<Keyb, REPORT_SIZE> = Vec::new();
        let emit = Mod(&Ctrl(&Alt(&Shift(&Emit::Identity))));
        let first = &Key::Right(KeyId::K6); // right gui
        let identity = Emit::Code(Keyb::Q);
        build_keyboard_report(emit, identity, first, &mut keyboard);
        assert_eq!(Keyb::RightGUI, keyboard[0]);
        assert_eq!(Keyb::RightControl, keyboard[1]);
        assert_eq!(Keyb::RightAlt, keyboard[2]);
        assert_eq!(Keyb::RightShift, keyboard[3]);
        assert_eq!(Keyb::Q, keyboard[4]);
    }
}
