use tastlib::alias;
use tastlib::parse::ChordEvent::*;

alias!(TAB, L16);
alias!(BCK, L17);
alias!(RET, R17);
alias!(SPC, R16);

// Homerow mods left
alias!(L_G, L6); //GUI/WIN/COMMAND
alias!(L_A, L7); //ALT/OPTION
alias!(L_S, L8);
alias!(L_C, L9);
// Homerow mods right
alias!(R_G, R6);
alias!(R_A, R7);
alias!(R_S, R8);
alias!(R_C, R9);

#[rustfmt::skip]
mod unformatted {
    use tastlib::{chord, parse::ChordEmit};
    use usbd_human_interface_device::page::Keyboard;
    // Homerow mods right
    chord!( R_GUI,          2, [On(R_G), LAny],                     Mod(&Identity));
    chord!( R_ALT,          2, [On(R_A), LAny],                     Alt(&Identity));
    chord!( R_SHIFT,        2, [On(R_S), LAny],                     Shift(&Identity));
    chord!( R_CTRL,         2, [On(R_C), LAny],                     Ctrl(&Identity));
    chord!( R_GUI_ALT,      2, [Both(R_G, R_A), LAny],              Mod(&Alt(&Identity)));
    chord!( R_GUI_SHIFT,    2, [Both(R_G, R_S), LAny],              Mod(&Shift(&Identity)));
    chord!( R_GUI_CTRL,     2, [Both(R_G, R_C), LAny],              Mod(&Ctrl(&Identity)));
    chord!( R_ALT_SHIFT,    2, [Both(R_A, R_S), LAny],              Alt(&Shift(&Identity)));
    chord!( R_CTRL_ALT,     2, [Both(R_C, R_A), LAny],              Ctrl(&Alt(&Identity)));
    chord!( R_CTRL_SHIFT,   2, [Both(R_C, R_S), LAny],              Ctrl(&Shift(&Identity)));
    chord!( R_ALLMOD,       4, [On(R_A), On(R_S), On(R_C), LAny],   Ctrl(&Alt(&Shift(&Identity))));

    // Homerow mods left
    chord!( L_GUI,          2, [On(L_G), RAny],                     Mod(&Identity));
    chord!( L_ALT,          2, [On(L_A), RAny],                     Alt(&Identity));
    chord!( L_SHIFT,        2, [On(L_S), RAny],                     Shift(&Identity));
    chord!( L_CTRL,         2, [On(L_C), RAny],                     Ctrl(&Identity));
    chord!( L_GUI_ALT,      2, [Both(L_G, L_A), RAny],              Mod(&Alt(&Identity)));
    chord!( L_GUI_SHIFT,    2, [Both(L_G, L_S), RAny],              Mod(&Shift(&Identity)));
    chord!( L_GUI_CTRL,     2, [Both(L_G, L_C), RAny],              Mod(&Ctrl(&Identity)));
    chord!( L_ALT_SHIFT,    2, [Both(L_A, L_S), RAny],              Alt(&Shift(&Identity)));
    chord!( L_CTRL_ALT,     2, [Both(L_C, L_A), RAny],              Ctrl(&Alt(&Identity)));
    chord!( L_CTRL_SHIFT,   2, [Both(L_C, L_S), RAny],              Ctrl(&Shift(&Identity)));
    chord!( L_ALLMOD,       4, [On(L_A), On(L_S), On(L_C), RAny],   Ctrl(&Alt(&Shift(&Identity))));


    chord!(TAB_SPC_ESC,   1, [Both(TAB, SPC)], Code(Keyb::Escape));

    // Tab layer (shift)
    chord!(TAB_SHIFT,     2, [On(TAB), Any], Shift(&Identity));


    // BCK layer (specials)
    chord!(BCK_AE,        2, [On(BCK), On(E)], Code(Keyb::Apostrophe));
    chord!(BCK_OE,        2, [On(BCK), On(O)], Code(Keyb::Semicolon));
    chord!(BCK_AA,        2, [On(BCK), On(A)], Code(Keyb::LeftBrace));


    // SPC layer (numeric)
    chord!(SPC_Q,         2, [On(SPC), On(Q)],         Code(Keyb::Keyboard1));
    chord!(SPC_W,         2, [On(SPC), On(W)],         Code(Keyb::Keyboard2));
    chord!(SPC_E,         2, [On(SPC), On(E)],         Code(Keyb::Keyboard3));
    chord!(SPC_R,         2, [On(SPC), On(R)],         Code(Keyb::Keyboard4));
    chord!(SPC_T,         2, [On(SPC), On(T)],         Code(Keyb::Keyboard5));
    chord!(SPC_Y,         2, [On(SPC), On(Y)],         Code(Keyb::Keyboard6));
    chord!(SPC_U,         2, [On(SPC), On(U)],         Code(Keyb::Keyboard7));
    chord!(SPC_I,         2, [On(SPC), On(I)],         Code(Keyb::Keyboard8));
    chord!(SPC_O,         2, [On(SPC), On(O)],         Code(Keyb::Keyboard9));
    chord!(SPC_P,         2, [On(SPC), On(P)],         Code(Keyb::Keyboard0));
    chord!(SPC_A,         2, [On(SPC), On(A)],         Code(Keyb::F1));
    chord!(SPC_S,         2, [On(SPC), On(S)],         Code(Keyb::F2));
    chord!(SPC_D,         2, [On(SPC), On(D)],         Code(Keyb::F3));
    chord!(SPC_F,         2, [On(SPC), On(F)],         Code(Keyb::F4));
    chord!(SPC_G,         2, [On(SPC), On(G)],         Code(Keyb::F5));
    chord!(SPC_H,         2, [On(SPC), On(H)],         Code(Keyb::F6));
    chord!(SPC_J,         2, [On(SPC), On(J)],         Code(Keyb::F7));
    chord!(SPC_K,         2, [On(SPC), On(K)],         Code(Keyb::F8));
    chord!(SPC_L,         2, [On(SPC), On(L)],         Code(Keyb::F9));
    chord!(SPC_SEMICOLON, 2, [On(SPC), On(SEMICOLON)], Code(Keyb::F10));
    chord!(SPC_B,         2, [On(SPC), On(B)],         Code(Keyb::F11));
    chord!(SPC_N,         2, [On(SPC), On(N)],         Code(Keyb::F12));


    // RET layer (symbols)
    chord!(RET_Q,         2, [On(RET), On(Q)],         Shift(&Code(Keyb::Keyboard1)));
    chord!(RET_W,         2, [On(RET), On(W)],         Shift(&Code(Keyb::Keyboard2))); 
    chord!(RET_E,         2, [On(RET), On(E)],         Shift(&Code(Keyb::Keyboard3))); 
    chord!(RET_R,         2, [On(RET), On(R)],         Shift(&Code(Keyb::Keyboard4))); 
    chord!(RET_T,         2, [On(RET), On(T)],         Shift(&Code(Keyb::Keyboard5))); 
    chord!(RET_Y,         2, [On(RET), On(Y)],         Shift(&Code(Keyb::Keyboard6))); 
    chord!(RET_U,         2, [On(RET), On(U)],         Shift(&Code(Keyb::Keyboard7))); 
    chord!(RET_I,         2, [On(RET), On(I)],         Shift(&Code(Keyb::Keyboard8))); 
    chord!(RET_O,         2, [On(RET), On(O)],         Shift(&Code(Keyb::Keyboard9))); 
    chord!(RET_P,         2, [On(RET), On(P)],         Shift(&Code(Keyb::Keyboard0))); 
    chord!(RET_A,         2, [On(RET), On(A)],         Shift(&Code(Keyb::LeftBrace))); 
    chord!(RET_S,         2, [On(RET), On(S)],         Shift(&Code(Keyb::Keyboard9))); 
    chord!(RET_D,         2, [On(RET), On(D)],         Code(Keyb::LeftBrace)); 
    chord!(RET_F,         2, [On(RET), On(F)],         Shift(&Code(Keyb::Comma))); 
    chord!(RET_G,         2, [On(RET), On(G)],         Shift(&Code(Keyb::Backslash))); 
    chord!(RET_H,         2, [On(RET), On(H)],         Code(Keyb::Backslash));
    chord!(RET_J,         2, [On(RET), On(J)],         Shift(&Code(Keyb::Dot))); 
    chord!(RET_K,         2, [On(RET), On(K)],         Code(Keyb::RightBrace));
    chord!(RET_L,         2, [On(RET), On(L)],         Shift(&Code(Keyb::Keyboard0))); 
    chord!(RET_SEMICOLON, 2, [On(RET), On(SEMICOLON)], Shift(&Code(Keyb::RightBrace))); 
    chord!(RET_Z,         2, [On(RET), On(Z)],         Shift(&Code(Keyb::Grave))); 
    chord!(RET_X,         2, [On(RET), On(X)],         Code(Keyb::Grave));
    chord!(RET_C,         2, [On(RET), On(C)],         Shift(&Code(Keyb::Equal))); 
    chord!(RET_V,         2, [On(RET), On(V)],         Code(Keyb::Apostrophe));
    chord!(RET_B,         2, [On(RET), On(B)],         Code(Keyb::Equal));
    chord!(RET_N,         2, [On(RET), On(N)],         Shift(&Code(Keyb::Apostrophe))); 
    chord!(RET_M,         2, [On(RET), On(M)],         Code(Keyb::Minus));
    chord!(RET_COMMA,     2, [On(RET), On(COMMA)],     Shift(&Code(Keyb::Minus))); 


    // Thumb keys
    chord!(ON_RET, 1, [On(RET)], Code(Keyb::ReturnEnter));
    chord!(ON_TAB, 1, [On(TAB)], Code(Keyb::Tab));
    chord!(ON_BCK, 1, [On(BCK)], Code(Keyb::DeleteBackspace));
    chord!(ON_SPC, 1, [On(SPC)], Code(Keyb::Space));


    // Base layer
    chord!(ON_Q,               1, [On(Q)],            Code(Keyb::Q));
    chord!(ON_W,               1, [On(W)],            Code(Keyb::W));
    chord!(ON_E,               1, [On(E)],            Code(Keyb::E));
    chord!(ON_R,               1, [On(R)],            Code(Keyb::R));
    chord!(ON_T,               1, [On(T)],            Code(Keyb::T));
    chord!(ON_Y,               1, [On(Y)],            Code(Keyb::Y));
    chord!(ON_U,               1, [On(U)],            Code(Keyb::U));
    chord!(ON_I,               1, [On(I)],            Code(Keyb::I));
    chord!(ON_O,               1, [On(O)],            Code(Keyb::O));
    chord!(ON_P,               1, [On(P)],            Code(Keyb::P));
    chord!(ON_A,               1, [On(A)],            Code(Keyb::A));
    chord!(ON_S,               1, [On(S)],            Code(Keyb::S));
    chord!(ON_D,               1, [On(D)],            Code(Keyb::D));
    chord!(ON_F,               1, [On(F)],            Code(Keyb::F));
    chord!(ON_G,               1, [On(G)],            Code(Keyb::G));
    chord!(ON_H,               1, [On(H)],            Code(Keyb::H));
    chord!(ON_J,               1, [On(J)],            Code(Keyb::J));
    chord!(ON_K,               1, [On(K)],            Code(Keyb::K));
    chord!(ON_L,               1, [On(L)],            Code(Keyb::L));
    chord!(ON_SEMICOLON,       1, [On(SEMICOLON)],    Code(Keyb::Semicolon));
    chord!(ON_Z,               1, [On(Z)],            Code(Keyb::Z));
    chord!(ON_X,               1, [On(X)],            Code(Keyb::X));
    chord!(ON_C,               1, [On(C)],            Code(Keyb::C));
    chord!(ON_V,               1, [On(V)],            Code(Keyb::V));
    chord!(ON_B,               1, [On(B)],            Code(Keyb::B));
    chord!(ON_N,               1, [On(N)],            Code(Keyb::N));
    chord!(ON_M,               1, [On(M)],            Code(Keyb::M));
    chord!(ON_COMMA,           1, [On(COMMA)],        Code(Keyb::Comma));
    chord!(ON_DOT,             1, [On(DOT)],          Code(Keyb::Dot));
    chord!(ON_FORWARDSLASH,    1, [On(FORWARDSLASH)], Code(Keyb::ForwardSlash));

    pub const RULES: [ChordEmit<Keyboard>; 111] = [
        R_GUI,
        R_ALT,
        R_SHIFT,
        R_CTRL,
        R_GUI_ALT,
        R_GUI_SHIFT,
        R_GUI_CTRL,
        R_ALT_SHIFT,
        R_CTRL_ALT,
        R_CTRL_SHIFT,
        R_ALLMOD,
        L_GUI,
        L_ALT,
        L_SHIFT,
        L_CTRL,
        L_GUI_ALT,
        L_GUI_SHIFT,
        L_GUI_CTRL,
        L_ALT_SHIFT,
        L_CTRL_ALT,
        L_CTRL_SHIFT,
        L_ALLMOD,
        TAB_SPC_ESC,
        TAB_SHIFT,
        BCK_AE,
        BCK_OE,
        BCK_AA,
        SPC_Q,
        SPC_W,
        SPC_E,
        SPC_R,
        SPC_T,
        SPC_Y,
        SPC_U,
        SPC_I,
        SPC_O,
        SPC_P,
        SPC_A,
        SPC_S,
        SPC_D,
        SPC_F,
        SPC_G,
        SPC_H,
        SPC_J,
        SPC_K,
        SPC_L,
        SPC_SEMICOLON,
        SPC_B,
        SPC_N,
        RET_Q,
        RET_W,
        RET_E,
        RET_R,
        RET_T,
        RET_Y,
        RET_U,
        RET_I,
        RET_O,
        RET_P,
        RET_A,
        RET_S,
        RET_D,
        RET_F,
        RET_G,
        RET_H,
        RET_J,
        RET_K,
        RET_L,
        RET_SEMICOLON,
        RET_Z,
        RET_X,
        RET_C,
        RET_V,
        RET_B,
        RET_N,
        RET_M,
        RET_COMMA,
        ON_RET,
        ON_TAB,
        ON_BCK,
        ON_SPC,
        ON_Q,
        ON_W,
        ON_E,
        ON_R,
        ON_T,
        ON_Y,
        ON_U,
        ON_I,
        ON_O,
        ON_P,
        ON_A,
        ON_S,
        ON_D,
        ON_F,
        ON_G,
        ON_H,
        ON_J,
        ON_K,
        ON_L,
        ON_SEMICOLON,
        ON_Z,
        ON_X,
        ON_C,
        ON_V,
        ON_B,
        ON_N,
        ON_M,
        ON_COMMA,
        ON_DOT,
        ON_FORWARDSLASH,
    ];
}
pub use unformatted::*;
