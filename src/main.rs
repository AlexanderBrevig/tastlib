use heapless::Vec;
use k_board::{keyboard::Keyboard, keys::Keys};
use tastlib::{
    lex::{Event, Key, STACK_SIZE},
    report::eval,
};

mod config;

fn main() {
    let mut stack: Vec<Event, STACK_SIZE> = Vec::new();

    let mut tab_toggle = false;
    let mut bck_toggle = false;
    let mut spc_toggle = false;
    let mut ret_toggle = false;

    for key in Keyboard::new() {
        match key {
            Keys::Char(chr) => {
                if stack.push(from_char_to_event(chr)).is_err() {
                    panic!("Should have enough capacity to push on stack");
                }
            }
            Keys::Delete => stack.clear(),
            Keys::Home => tab_sim(&mut tab_toggle, &mut stack),
            Keys::End => bck_sim(&mut bck_toggle, &mut stack),
            Keys::Space => ret_sim(&mut ret_toggle, &mut stack),
            Keys::Enter => spc_sim(&mut spc_toggle, &mut stack),
            Keys::Escape => {
                break;
            }
            _ => {}
        }
        let keyboard = eval(&mut stack, &config::RULES);
        if !keyboard.is_empty() {
            println!("Keyboard: {:?}", keyboard);
        }
    }
}

fn sim(key: Key, toggler: &mut bool, stack: &mut Vec<Event, 128>) {
    let evt = if *toggler {
        Event::Up(key)
    } else {
        Event::Down(key)
    };
    *toggler = !*toggler;
    stack.push(evt).unwrap();
}

fn spc_sim(toggler: &mut bool, stack: &mut Vec<Event, 128>) {
    sim(Key::R16, toggler, stack);
}

fn ret_sim(toggler: &mut bool, stack: &mut Vec<Event, 128>) {
    sim(Key::R17, toggler, stack);
}

fn bck_sim(toggler: &mut bool, stack: &mut Vec<Event, 128>) {
    sim(Key::L17, toggler, stack);
}

fn tab_sim(toggler: &mut bool, stack: &mut Vec<Event, 128>) {
    sim(Key::L16, toggler, stack);
}

#[rustfmt::skip]
fn from_char_to_event(value: char) -> Event {
    use tastlib::lex::Event::Down as D;
    use tastlib::lex::Event::Up as U;
    match value {
        // DOWN
        'Q' => D(Key::L1), 'W' => D(Key::L2), 'E' => D(Key::L3), 'R' => D(Key::L4), 'T' => D(Key::L5),
        'A' => D(Key::L6), 'S' => D(Key::L7), 'D' => D(Key::L8), 'F' => D(Key::L9), 'G' => D(Key::L10),
        'Z' => D(Key::L11), 'X' => D(Key::L12), 'C' => D(Key::L13), 'V' => D(Key::L14), 'B' => D(Key::L15),
        'Y' => D(Key::R5), 'U' => D(Key::R4), 'I' => D(Key::R3), 'O' => D(Key::R2), 'P' => D(Key::R1),
        'H' => D(Key::R10), 'J' => D(Key::R9), 'K' => D(Key::R8), 'L' => D(Key::R7), ':' => D(Key::R6),
        'N' => D(Key::R15), 'M' => D(Key::R14), '<' => D(Key::R13), '>' => D(Key::R12), '?' => D(Key::R11),
        // UP
        'q' => U(Key::L1), 'w' => U(Key::L2), 'e' => U(Key::L3), 'r' => U(Key::L4), 't' => U(Key::L5),
        'a' => U(Key::L6), 's' => U(Key::L7), 'd' => U(Key::L8), 'f' => U(Key::L9), 'g' => U(Key::L10),
        'z' => U(Key::L11), 'x' => U(Key::L12), 'c' => U(Key::L13), 'v' => U(Key::L14), 'b' => U(Key::L15),
        'y' => U(Key::R5), 'u' => U(Key::R4), 'i' => U(Key::R3), 'o' => U(Key::R2), 'p' => U(Key::R1),
        'h' => U(Key::R10), 'j' => U(Key::R9), 'k' => U(Key::R8), 'l' => U(Key::R7), ';' => U(Key::R6),
        'n' => U(Key::R15), 'm' => U(Key::R14), ',' => U(Key::R13), '.' => U(Key::R12), '/' => U(Key::R11),
        _ => todo!("Key {} not implemented yet", value),
    }
}

#[cfg(test)]
mod tests {
    use tastlib::lex::qwerty::*;
    use usbd_human_interface_device::page::Keyboard as Keyb;

    use super::Event::*;
    use super::*;
    use crate::config::*;

    #[test]
    fn test_empty() {
        let mut stack: Vec<Event, STACK_SIZE> = Vec::new();
        let keyboard = eval(&mut stack, &config::RULES);
        assert!(keyboard.is_empty());
    }

    #[test]
    fn test_single() {
        let mut stack: Vec<Event, STACK_SIZE> = Vec::new();
        stack.push(Down(Q.into())).unwrap();
        stack.push(Up(Q.into())).unwrap();

        let keyboard = eval(&mut stack, &config::RULES);
        assert_eq!(Keyb::Q, keyboard[0]);
    }

    #[test]
    fn test_copy() {
        let mut stack: Vec<Event, STACK_SIZE> = Vec::new();
        stack.push(Down(F.into())).unwrap();
        stack.push(Down(C.into())).unwrap();
        stack.push(Up(C.into())).unwrap();
        stack.push(Up(F.into())).unwrap();

        let keyboard = eval(&mut stack, &config::RULES);
        assert_eq!(Keyb::RightControl, keyboard[0]);
        assert_eq!(Keyb::C, keyboard[1]);
    }

    #[test]
    fn test_layer_pipe() {
        let mut stack: Vec<Event, STACK_SIZE> = Vec::new();
        stack.push(Down(RET.into())).unwrap();
        stack.push(Down(G.into())).unwrap();
        stack.push(Up(G.into())).unwrap();
        stack.push(Up(RET.into())).unwrap();

        let keyboard = eval(&mut stack, &config::RULES);
        assert_eq!(Keyb::RightShift, keyboard[0]);
        assert_eq!(Keyb::Backslash, keyboard[1]);
    }

    #[test]
    fn test_tab_only() {
        let mut stack: Vec<Event, STACK_SIZE> = Vec::new();
        stack.push(Down(TAB.into())).unwrap();
        stack.push(Up(TAB.into())).unwrap();

        let keyboard = eval(&mut stack, &config::RULES);
        assert_eq!(Keyb::Tab, keyboard[0]);
    }
}
