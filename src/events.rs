
#[deriving(Clone,Show)]
pub enum Event {
    /// The position of the window has changed.
    PositionChanged(uint, uint),

    /// The size of the window has changed.
    SizeChanged(uint, uint),

    /// The position of the window has changed.
    Moved(uint, uint),

    /// The window has been closed.
    Closed,

    /// The window received a unicode character.
    ReceivedCharacter(char),

    /// The cursor has moved on the window.
    /// 
    /// The parameter are the (x,y) coords in pixels relative to the top-left corner of the window.
    CursorPositionChanged(uint, uint),

    /// The window gained or lost focus.
    /// 
    /// The parameter is true if the window has gained focus, and false if it has lost focus.
    Focused(bool),

    /// The window has been turned into an icon or restored.
    /// 
    /// The parameter is true if the window has been iconified, and false if it has been restored.
    Iconified(bool),

    /// The system asked that the content of this window must be redrawn.
    NeedRefresh,

    /// An element has been pressed.
    Pressed(Element),

    /// An element has been released.
    Released(Element),
}

#[deriving(Show, Clone)]
pub enum Element {
    Slider0,
    Slider1,
    Slider2,
    Slider3,
    Button0,
    Button1,
    Button2,
    Button3,
    Button4,
    Button5,
    Button6,
    Button7,
    Button8,
    Button9,
    Button10,
    Button11,
    Button12,
    Button13,
    Button14,
    Button15,
    Key0,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    A,
    AbntC1,
    AbntC2,
    Add,
    Apostrophe,
    Apps,
    At,
    Ax,
    B,
    Back,
    Backslash,
    C,
    Calculator,
    Capital,
    Colon,
    Comma,
    Convert,
    D,
    Decimal,
    Delete,
    Divide,
    Down,
    E,
    End,
    Equals,
    Escape,
    F,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    G,
    Grave,
    H,
    Home,
    I,
    Insert,
    J,
    K,
    Kana,
    Kanji,
    L,
    LCracket,
    LControl,
    Left,
    LMenu,
    LShift,
    LWin,
    M,
    Mail,
    MediaSelect,
    MediaStop,
    Minus,
    Multiply,
    Mute,
    MyComputer,
    N,
    Next,
    NextTrack,
    NoConvert,
    Numlock,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    NumpadComma,
    NumpadEnter,
    NumpadEquals,
    O,
    OEM102,
    P,
    Pause,
    Period,
    Playpause,
    Power,
    Prevtrack,
    Prior,
    Q,
    R,
    RBracket,
    RControl,
    Return,
    Right,
    RMenu,
    RShift,
    RWin,
    S,
    Scroll,
    Semicolon,
    Slash,
    Sleep,
    Snapshot,
    Space,
    Stop,
    Subtract,
    Sysrq,
    T,
    Tab,
    U,
    Underline,
    Unlabeled,
    Up,
    V,
    VolumeDown,
    VolumeUp,
    W,
    Wake,
    Webback,
    WebFavorites,
    WebForward,
    WebHome,
    WebRefresh,
    WebSearch,
    WebStop,
    X,
    Y,
    Yen,
    Z
}
