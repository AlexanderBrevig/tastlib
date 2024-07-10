pub mod lex;
pub mod parse;
pub mod report;

#[allow(clippy::crate_in_macro_def)]
#[macro_export]
macro_rules! chord {
    ($rule:ident, $len:expr, $chord_event:expr, $emit:expr) => {
        #[allow(non_snake_case)]
        mod $rule {
            use crate::config::*;
            use usbd_human_interface_device::page::Keyboard as Keyb;
            use $crate::lex::qwerty::*;
            use $crate::parse::ChordEvent;
            use $crate::parse::ChordEvent::*;
            use $crate::parse::Emit;
            use $crate::parse::Emit::*;
            pub const CHORD_EVENTS: [ChordEvent; $len] = $chord_event;
            pub const CHORD_EMIT: Emit<Keyb> = $emit;
        }
        pub const $rule: $crate::parse::ChordEmit<usbd_human_interface_device::page::Keyboard> =
            $crate::parse::ChordEmit(&$rule::CHORD_EVENTS, $rule::CHORD_EMIT);
    }; /*($rule:ident, $chord_event:expr, $emit:expr, $layout:ident) => {
           #[allow(non_snake_case)]
           mod $rule {
               use crate::config::*;
               use usbd_human_interface_device::page::Keyboard as Keyb;
               use $crate::lex::$layout::*;
               use $crate::parse::ChordEvent;
               use $crate::parse::ChordEvent::{Both, RAny};
               use $crate::parse::Emit;
               use $crate::parse::Emit::{Ctrl, Identity, Shift};
               pub const CHORD_EVENTS: [ChordEvent; 2] = $chord_event;
               pub const CHORD_EMIT: Emit<Keyb> = $emit;
           }
           pub const $rule: $crate::parse::ChordEmit<usbd_human_interface_device::page::Keyboard> =
               $crate::parse::ChordEmit(&$rule::CHORD_EVENTS, $rule::CHORD_EMIT);
       };*/
}
#[macro_export]
macro_rules! alias {
    ($alias:ident, $side:ident, $key:ident) => {
        pub const $alias: $crate::lex::Pressed =
            $crate::lex::Pressed($crate::lex::Key::$side($crate::lex::KeyId::$key));
    };
}
