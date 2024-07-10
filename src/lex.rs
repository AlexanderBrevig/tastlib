#[rustfmt::skip]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum KeyId {
    K1,  K2,  K3,  K4,  K5,  K6,  K7,  K8,  K9,  K10, 
    K11, K12, K13, K14, K15, K16, K17, K18, K19, K20,
    K21, K22, K23, K24, K25, K26, K27, K28, K29, K30,
    K31, K32, K33, K34, K35, K36, K37, K38, K39, K40,
    // Never go above 63, as 63 is max for still serializing to one byte
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Key {
    Left(KeyId),
    Right(KeyId),
}

impl From<Key> for KeyId {
    fn from(value: Key) -> Self {
        match value {
            Key::Left(key) => key,
            Key::Right(key) => key,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Event {
    Down(Key),
    Up(Key),
}

impl From<Event> for Key {
    fn from(value: Event) -> Self {
        match value {
            Event::Down(key) => key,
            Event::Up(key) => key,
        }
    }
}

impl From<Event> for u8 {
    fn from(value: Event) -> Self {
        let is_down = matches!(value, Event::Down(_)) as u8;
        let key = value.into();
        let is_left = matches!(key, Key::Left(_)) as u8;
        let id: KeyId = key.into();
        is_down << 7 | is_left << 6 | ((id as u8) & 0b0011_1111)
    }
}
impl From<u8> for Event {
    fn from(value: u8) -> Self {
        let is_down = (value & 0b1000_0000) != 0;
        let is_left = (value & 0b0100_0000) != 0;
        let id = value & 0b0011_1111;
        // TODO: there's probably a cleaner way to do this
        use KeyId::*;
        #[rustfmt::skip]
        let key_id = match id {
            0 => K1, 1 => K2, 2 => K3, 3 => K4, 4 => K5,
            5 => K6, 6 => K7, 7 => K8, 8 => K9, 9 => K10,
            10 => K11, 11 => K12, 12 => K13, 13 => K14, 14 => K15,
            15 => K16, 16 => K17, 17 => K18, 18 => K19, 19 => K20,
            20 => K21, 21 => K22, 22 => K23, 23 => K24, 24 => K25,
            25 => K26, 26 => K27, 27 => K28, 28 => K29, 29 => K30,
            30 => K31, 31 => K32, 32 => K33, 33 => K34, 34 => K35,
            35 => K36, 36 => K37, 37 => K38, 38 => K39, 39 => K40,
            _ => K1,
        };
        let key = if is_left {
            Key::Left(key_id)
        } else {
            Key::Right(key_id)
        };
        match is_down {
            true => Event::Down(key),
            false => Event::Up(key),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Pressed(pub Key);

impl From<Pressed> for Key {
    fn from(value: Pressed) -> Self {
        value.0
    }
}

pub mod qwerty {
    use crate::alias;

    alias!(Q, Left, K1);
    alias!(W, Left, K2);
    alias!(E, Left, K3);
    alias!(R, Left, K4);
    alias!(T, Left, K5);
    alias!(A, Left, K6);
    alias!(S, Left, K7);
    alias!(D, Left, K8);
    alias!(F, Left, K9);
    alias!(G, Left, K10);
    alias!(Z, Left, K11);
    alias!(X, Left, K12);
    alias!(C, Left, K13);
    alias!(V, Left, K14);
    alias!(B, Left, K15);

    alias!(Y, Right, K5);
    alias!(U, Right, K4);
    alias!(I, Right, K3);
    alias!(O, Right, K2);
    alias!(P, Right, K1);
    alias!(H, Right, K10);
    alias!(J, Right, K9);
    alias!(K, Right, K8);
    alias!(L, Right, K7);
    alias!(SEMICOLON, Right, K6);
    alias!(N, Right, K15);
    alias!(M, Right, K14);
    alias!(COMMA, Right, K13);
    alias!(DOT, Right, K12);
    alias!(FORWARDSLASH, Right, K11);
}

pub mod colemak {
    // TODO: implement colemak
}

pub mod dvorak {
    // TODO: implement colemak
}

use heapless::Vec;

pub const STACK_SIZE: usize = 128;
pub const PRESS_SIZE: usize = 64;
pub const REPORT_SIZE: usize = 32; // TODO: figure out how to handle Emit::String

pub fn chord(stack: &mut Vec<Event, STACK_SIZE>) -> Vec<Pressed, PRESS_SIZE> {
    let mut pressed: Vec<Pressed, PRESS_SIZE> = Vec::new();
    if !stack.is_empty() {
        rec_chord(stack, &mut pressed);
        let Event::Down(root) = stack[0] else {
            // Stack can never start with an Event::Up
            stack.clear(); // Something _very_ bad has happened
            pressed.clear();
            return pressed;
        };
        if pressed.is_empty() {
            return pressed;
        }
        let Pressed(first) = pressed[0];
        if root != first {
            pressed.clear();
            return pressed;
        }
        for press in &pressed {
            let Pressed(press_key) = press;
            // remove events from stack used by presses
            stack.retain(|e| match e {
                Event::Down(event_key) => *press_key != *event_key,
                Event::Up(event_key) => *press_key != *event_key,
            });
        }
    }
    pressed
}

fn rec_chord(stack: &[Event], pressed: &mut Vec<Pressed, PRESS_SIZE>) {
    assert!(!stack.is_empty(), "Stack cannot be empty in rec_chord");
    let root_key = if !pressed.is_empty() {
        Some(pressed[0])
    } else {
        None
    };
    if let Some(Pressed(root_key)) = root_key {
        if let Event::Up(key) = &stack[0] {
            if root_key == *key {
                return;
            }
        }
    }
    if let Event::Down(start_key) = &stack[0] {
        for entry in stack {
            if let Event::Up(key) = entry {
                if key == start_key {
                    if pressed.push(Pressed(*start_key)).is_err() {
                        panic!("Should have enough capacity to push pressed");
                    }
                    break;
                }
            }
        }
    }
    if stack.len() >= 2 {
        rec_chord(&stack[1..], pressed);
    }
}

#[cfg(test)]
mod tests {
    use super::Event::*;
    use super::Key::*;
    use super::*;

    #[test]
    fn single_key() {
        let mut stack: Vec<Event, STACK_SIZE> = Vec::new();
        stack.push(Down(Left(KeyId::K1))).unwrap();
        stack.push(Up(Left(KeyId::K1))).unwrap();

        let presses = chord(&mut stack);
        assert_eq!(stack.len(), 0);
        assert_eq!(presses.len(), 1);
        assert_eq!(Pressed(Left(KeyId::K1)), presses[0]);
    }

    #[test]
    fn single_key_fail() {
        let mut stack: Vec<Event, STACK_SIZE> = Vec::new();
        stack.push(Up(Left(KeyId::K1))).unwrap();
        stack.push(Down(Left(KeyId::K1))).unwrap();

        let presses = chord(&mut stack);
        // This is illegal, reset stack with no presses
        assert_eq!(stack.len(), 0);
        assert_eq!(presses.len(), 0);
    }

    #[test]
    fn single_key_with_surplus() {
        let mut stack: Vec<Event, STACK_SIZE> = Vec::new();
        stack.push(Down(Left(KeyId::K1))).unwrap();
        stack.push(Up(Left(KeyId::K1))).unwrap();
        stack.push(Down(Left(KeyId::K2))).unwrap();

        let presses = chord(&mut stack);
        assert_eq!(stack.len(), 1);
        assert_eq!(presses.len(), 1);
        assert_eq!(Pressed(Left(KeyId::K1)), presses[0]);
        assert_eq!(Some(&Down(Left(KeyId::K2))), stack.first());
    }

    #[test]
    fn two_single_weird_timing() {
        let mut stack: Vec<Event, STACK_SIZE> = Vec::new();
        stack.push(Down(Left(KeyId::K1))).unwrap();
        stack.push(Down(Left(KeyId::K2))).unwrap();
        stack.push(Up(Left(KeyId::K1))).unwrap();

        let presses = chord(&mut stack);
        assert_eq!(stack.len(), 1);
        assert_eq!(presses.len(), 1);
        assert_eq!(Pressed(Left(KeyId::K1)), presses[0]);

        stack.push(Up(Left(KeyId::K2))).unwrap();

        let presses = chord(&mut stack);
        assert_eq!(stack.len(), 0);
        assert_eq!(presses.len(), 1);
        assert_eq!(Pressed(Left(KeyId::K2)), presses[0]);
    }

    #[test]
    fn two_single_key_strokes() {
        let mut stack: Vec<Event, STACK_SIZE> = Vec::new();
        stack.push(Down(Left(KeyId::K1))).unwrap();
        stack.push(Up(Left(KeyId::K1))).unwrap();
        stack.push(Down(Left(KeyId::K2))).unwrap();
        stack.push(Up(Left(KeyId::K2))).unwrap();

        let presses = chord(&mut stack);
        assert_eq!(stack.len(), 2);
        assert_eq!(presses.len(), 1);
        assert_eq!(Pressed(Left(KeyId::K1)), presses[0]);
        let presses = chord(&mut stack);
        assert_eq!(stack.len(), 0);
        assert_eq!(presses.len(), 1);
        assert_eq!(Pressed(Left(KeyId::K2)), presses[0]);
    }

    #[test]
    fn two_key_chord() {
        let mut stack: Vec<Event, STACK_SIZE> = Vec::new();
        stack.push(Down(Left(KeyId::K1))).unwrap();
        stack.push(Down(Left(KeyId::K2))).unwrap();
        stack.push(Up(Left(KeyId::K2))).unwrap();
        stack.push(Up(Left(KeyId::K1))).unwrap();

        let presses = chord(&mut stack);
        assert_eq!(stack.len(), 0);
        assert_eq!(presses.len(), 2);
        assert_eq!(Pressed(Left(KeyId::K1)), presses[0]);
        assert_eq!(Pressed(Left(KeyId::K2)), presses[1]);
    }

    #[test]
    fn two_key_chord_in_eval() {
        let mut stack: Vec<Event, STACK_SIZE> = Vec::new();
        stack.push(Down(Left(KeyId::K1))).unwrap();
        stack.push(Down(Left(KeyId::K2))).unwrap();
        stack.push(Up(Left(KeyId::K2))).unwrap();
        let presses = chord(&mut stack);
        assert_eq!(stack.len(), 3);
        assert_eq!(presses.len(), 0);

        stack.push(Up(Left(KeyId::K1))).unwrap();

        let presses = chord(&mut stack);
        assert_eq!(stack.len(), 0);
        assert_eq!(presses.len(), 2);
        assert_eq!(Pressed(Left(KeyId::K1)), presses[0]);
        assert_eq!(Pressed(Left(KeyId::K2)), presses[1]);
    }

    #[test]
    fn two_key_chord_surplus_then_single() {
        let mut stack: Vec<Event, STACK_SIZE> = Vec::new();
        stack.push(Down(Left(KeyId::K1))).unwrap();
        stack.push(Down(Left(KeyId::K2))).unwrap();
        stack.push(Up(Left(KeyId::K2))).unwrap();
        stack.push(Up(Left(KeyId::K1))).unwrap();
        stack.push(Down(Left(KeyId::K3))).unwrap();

        let presses = chord(&mut stack);
        assert_eq!(stack.len(), 1);
        assert_eq!(presses.len(), 2);
        assert_eq!(Pressed(Left(KeyId::K1)), presses[0]);
        assert_eq!(Pressed(Left(KeyId::K2)), presses[1]);
        assert_eq!(Some(&Down(Left(KeyId::K3))), stack.first());

        stack.push(Up(Left(KeyId::K3))).unwrap();
        let presses = chord(&mut stack);
        assert_eq!(stack.len(), 0);
        assert_eq!(presses.len(), 1);
        assert_eq!(Pressed(Left(KeyId::K3)), presses[0]);
    }

    #[rustfmt::skip]
    #[allow(clippy::unusual_byte_groupings)]
    #[test]
    fn serde_test() {
        //                                                   is_down
        //                                                   | is_left
        //                                                   | | KeyId
        //                                                   | | |++++|
        serde_assert(Event::Down(Key::Left(KeyId::K40)),  0b_1_1_100111);
        serde_assert(Event::Down(Key::Left(KeyId::K10)),  0b_1_1_001001);
        serde_assert(Event::Down(Key::Left(KeyId::K1)),   0b_1_1_000000);
        serde_assert(Event::Down(Key::Right(KeyId::K40)), 0b_1_0_100111);
        serde_assert(Event::Down(Key::Right(KeyId::K10)), 0b_1_0_001001);
        serde_assert(Event::Down(Key::Right(KeyId::K1)),  0b_1_0_000000);
        serde_assert(Event::Up(Key::Left(KeyId::K40)),    0b_0_1_100111);
        serde_assert(Event::Up(Key::Right(KeyId::K10)),   0b_0_0_001001);
        serde_assert(Event::Up(Key::Right(KeyId::K1)),    0b_0_0_000000);
    }

    fn serde_assert(evt: Event, expected: u8) {
        let bin: u8 = evt.into();
        let parsed: Event = bin.into();
        assert_eq!(expected, bin);
        assert_eq!(evt, parsed);
    }
}
