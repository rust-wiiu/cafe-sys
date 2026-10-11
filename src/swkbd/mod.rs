//! Software Keyboard (swkbd)
//!
//! On-screen keyboard with an optional input form, drawn by the application each frame.

use crate::ffi;

/// cUnfixedStringLengthMax
///
/// Unfixed strings are finalized when they exceed this length.
pub const UNFIXED_STRING_LENGTH_MAX: usize = 24;
/// cProportionalStringTextMax
pub const PROPORTIONAL_STRING_TEXT_MAX: usize = 9999;
/// cProportionalStringLengthMax
pub const PROPORTIONAL_STRING_LENGTH_MAX: usize = 65000;
/// cMonospaceStringLengthMax
pub const MONOSPACE_STRING_LENGTH_MAX: usize = 40;
/// cGuideStringLengthMax
pub const GUIDE_STRING_LENGTH_MAX: usize = 256;
/// cOkStringLengthMax
pub const OK_STRING_LENGTH_MAX: usize = 8;
/// cUserWordMax
pub const USER_WORD_MAX: usize = 500;
/// cLearnDicSize
///
/// Size of a learning dictionary buffer, see [init_learn_dic].
pub const LEARN_DIC_SIZE: usize = 80 + 42 * 1000;

/// nn::swkbd::RegionType
#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RegionType {
    #[default]
    Japan = 0,
    Usa = 1,
    Europe = 2,
    China = 3,
    Korea = 4,
    Taiwan = 5,
}

/// nn::swkbd::LanguageType
///
/// Must be supported by the [RegionType], otherwise English (Japanese for [RegionType::Japan]) is used.
#[doc(alias = "LangType")]
#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LanguageType {
    #[default]
    Japanese = 0,
    English = 1,
    French = 2,
    German = 3,
    Italian = 4,
    Spanish = 5,
    SimplifiedChinese = 6,
    Korean = 7,
    Dutch = 8,
    Portuguese = 9,
    Russian = 10,
    TraditionalChinese = 11,
}

/// nn::swkbd::ControllerType
///
/// The keyboard is shown on the TV for Wii Remotes and on the GamePad for the DRC.
#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ControllerType {
    WiiRemote0 = 0,
    WiiRemote1 = 1,
    WiiRemote2 = 2,
    WiiRemote3 = 3,
    #[default]
    DrcGamepad = 4,
}

/// nn::swkbd::KeyboardMode
#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyboardMode {
    /// All layouts of the region, limited by [ConfigArg::keyboard_flags].
    #[default]
    Full = 0,
    Numpad = 1,
    /// Letters, numbers and ASCII symbols only.
    Password = 2,
    /// Letters, numbers, `_` and `-`.
    Nnid = 3,
}

/// nn::swkbd::KeyboardLayout
#[doc(alias = "KeyboardType")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KeyboardLayout(pub u32);

impl KeyboardLayout {
    pub const JPN_JP_QWERTY: Self = Self(0);
    pub const USA_EN_QWERTY: Self = Self(1);
    pub const USA_FR_QWERTY: Self = Self(2);
    pub const USA_ES_QWERTY: Self = Self(3);
    pub const USA_PT_QWERTY: Self = Self(4);
    pub const EUR_EN_QWERTY: Self = Self(5);
    pub const EUR_FR_AZERTY: Self = Self(6);
    pub const EUR_DE_QWERTZ: Self = Self(7);
    pub const EUR_IT_QWERTY: Self = Self(8);
    pub const EUR_ES_QWERTY: Self = Self(9);
    pub const EUR_NL_QWERTY: Self = Self(10);
    pub const EUR_PT_QWERTY: Self = Self(11);
    pub const EUR_RU_JCUKEN: Self = Self(12);
    pub const JPN_JP_KANA: Self = Self(13);
    pub const JPN_JP_PHONE: Self = Self(14);
    pub const JPN_JP_HANDWRITING: Self = Self(15);
    pub const JPN_JP_SYMBOLS: Self = Self(16);
    pub const LATIN_SYMBOLS: Self = Self(17);
    pub const USA_EUR_SYMBOLS: Self = Self(18);
    /// Default layout of the language.
    pub const LANGUAGE_DEFAULT: Self = Self(19);
}

/// KeyboardFlag
///
/// Layouts usable in [KeyboardMode::Full], one bit per [KeyboardLayout].
#[doc(alias = "KeyboardFlag")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KeyboardFlags(pub u32);

impl KeyboardFlags {
    pub const NONE: Self = Self(0);
    pub const ALL: Self = Self(0x7FFFF);

    #[inline]
    pub const fn from_layout(layout: KeyboardLayout) -> Self {
        Self(1 << layout.0)
    }
}

impl core::ops::BitOr for KeyboardFlags {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

/// InvalidChar
///
/// Characters that cannot be entered.
#[doc(alias = "InvalidChar")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InvalidChars(pub u32);

impl InvalidChars {
    pub const NONE: Self = Self(0);
    pub const LINEFEED: Self = Self(1 << 0);
    pub const SPACE: Self = Self(1 << 1);
    pub const AT_MARK: Self = Self(1 << 2);
    pub const PERCENT: Self = Self(1 << 3);
    pub const SLASH: Self = Self(1 << 4);
    pub const BACKSLASH: Self = Self(1 << 5);
    pub const NUMERIC: Self = Self(1 << 6);
    pub const HEART: Self = Self(1 << 7);
    pub const EURO_MARK: Self = Self(1 << 8);
    pub const OUTSIDE_OF_SJIS: Self = Self(1 << 9);
    /// Everything not used in download codes. Use with [KeyboardMode::Nnid].
    pub const DOWNLOAD_CODE: Self = Self(1 << 10);
    /// Everything not used in NNIDs. Use with [KeyboardMode::Nnid].
    pub const NNID: Self = Self(1 << 11);
}

impl core::ops::BitOr for InvalidChars {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

/// LoadOffFlag
///
/// Features not to load, to reduce the work memory.
#[doc(alias = "LoadOffFlag")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LoadOffFlags(pub u32);

impl LoadOffFlags {
    pub const NONE: Self = Self(0);
    /// [InputFormType::Monospace].
    pub const MONOSPACE: Self = Self(1 << 0);
    /// [InputFormType::Proportional].
    pub const PROPORTIONAL: Self = Self(1 << 1);
    pub const ALL: Self = Self(0b11);
}

/// nn::swkbd::GetWorkMemorySize
///
/// Size of the work memory passed to [create]. Unlike wut, this excludes the memory for loading the RPL, which is already loaded as an import.
#[doc(alias = "GetWorkMemorySize")]
#[inline]
pub const fn get_work_memory_size(load_off_flags: LoadOffFlags) -> u32 {
    let mut size = 0x1600000;
    if load_off_flags.0 & LoadOffFlags::MONOSPACE.0 != 0 {
        size -= 0xA0000;
    }
    if load_off_flags.0 & LoadOffFlags::PROPORTIONAL.0 != 0 {
        size -= 0x120000;
    }
    size
}

/// nn::swkbd::InputFormType
#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputFormType {
    /// Individual boxes, up to [MONOSPACE_STRING_LENGTH_MAX] characters.
    Monospace = 0,
    /// Page design, up to [PROPORTIONAL_STRING_TEXT_MAX] characters.
    #[default]
    Proportional = 1,
}

/// nn::swkbd::PasswordMode
#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PasswordMode {
    /// Shows the text.
    #[default]
    Clear = 0,
    /// Masks characters immediately.
    Hide = 1,
    /// Masks characters after one second or the next input.
    Fade = 2,
}

/// nn::swkbd::State
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct State(pub u32);

impl State {
    pub const HIDDEN: Self = Self(0);
    pub const FADE_IN: Self = Self(1);
    pub const VISIBLE: Self = Self(2);
    pub const FADE_OUT: Self = Self(3);
}

/// nn::swkbd::CursorMove
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CursorMove(pub u32);

impl CursorMove {
    pub const LEFT: Self = Self(0);
    pub const RIGHT: Self = Self(1);
    pub const UP: Self = Self(2);
    pub const DOWN: Self = Self(3);
}

/// nn::swkbd::KeyboardCondition
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KeyboardCondition {
    pub layout: KeyboardLayout,
    /// Tab of the layout, -1 for the default.
    pub tab: i32,
}

impl Default for KeyboardCondition {
    fn default() -> Self {
        Self {
            layout: KeyboardLayout::LANGUAGE_DEFAULT,
            tab: -1,
        }
    }
}

/// nn::swkbd::UserWord
///
/// User dictionary entry. Both strings are null-terminated; for Japanese, `reading` is in hiragana.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UserWord {
    pub reading: [u16; UNFIXED_STRING_LENGTH_MAX + 1],
    pub word: [u16; UNFIXED_STRING_LENGTH_MAX + 1],
}

/// Number of predictive text dictionary languages.
pub const DIC_LANGUAGE_COUNT: usize = 13;

/// nn::swkbd::ConfigArg
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConfigArg {
    pub language: LanguageType,
    pub controller: ControllerType,
    pub keyboard_mode: KeyboardMode,
    pub keyboard_flags: KeyboardFlags,
    pub first_keyboard_condition: KeyboardCondition,
    /// Shows the selection cursor from the start.
    pub first_select_cursor: bool,
    /// Label of the OK button, up to [OK_STRING_LENGTH_MAX] characters.
    pub ok_string: *const u16,
    /// Key label left of 0 on the numpad, 0 for none.
    pub numpad_char_left: u16,
    /// Key label right of 0 on the numpad, 0 for none.
    pub numpad_char_right: u16,
    /// Predictive text input.
    pub predict: bool,
    pub invalid_chars: InvalidChars,
    /// Must stay valid until the keyboard is closed.
    pub user_words: *mut UserWord,
    /// Up to [USER_WORD_MAX].
    pub num_user_words: i32,
    /// Learning dictionaries per language, see [init_learn_dic].
    pub learn_dic: [*mut ffi::c_void; DIC_LANGUAGE_COUNT],
    /// Customized dictionaries per language.
    pub customize_dic: [*const ffi::c_void; DIC_LANGUAGE_COUNT],
    /// 1 for 60 fps, 2 for 30 fps.
    pub frame_rate: i32,
    /// Draws the system Wii Remote pointer.
    pub draw_dpd_cursor: bool,
    /// Initial lock in [KeyboardMode::Password]: 0 for caps lock, 1 for shift, -1 for none.
    pub first_lock: i32,
}

impl Default for ConfigArg {
    fn default() -> Self {
        Self {
            language: LanguageType::English,
            controller: ControllerType::DrcGamepad,
            keyboard_mode: KeyboardMode::Full,
            keyboard_flags: KeyboardFlags::ALL,
            first_keyboard_condition: KeyboardCondition::default(),
            first_select_cursor: false,
            ok_string: core::ptr::null(),
            numpad_char_left: 0,
            numpad_char_right: 0,
            predict: false,
            invalid_chars: InvalidChars::NONE,
            user_words: core::ptr::null_mut(),
            num_user_words: 0,
            learn_dic: [core::ptr::null_mut(); DIC_LANGUAGE_COUNT],
            customize_dic: [core::ptr::null(); DIC_LANGUAGE_COUNT],
            frame_rate: 1,
            draw_dpd_cursor: false,
            first_lock: -1,
        }
    }
}

/// nn::swkbd::ReceiverArg
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ReceiverArg {
    pub event_receiver: *mut EventReceiver,
    pub string_buf: *mut u16,
    pub string_buf_size: i32,
    /// Finalized strings are clipped to this many characters, -1 for none.
    pub fixed_char_num_limit: i32,
    pub cursor_pos: i32,
    /// Selection start, -1 for none.
    pub select_from: i32,
}

impl Default for ReceiverArg {
    fn default() -> Self {
        Self {
            event_receiver: core::ptr::null_mut(),
            string_buf: core::ptr::null_mut(),
            string_buf_size: 0,
            fixed_char_num_limit: -1,
            cursor_pos: 0,
            select_from: -1,
        }
    }
}

/// nn::swkbd::KeyboardArg
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KeyboardArg {
    pub config: ConfigArg,
    pub receiver: ReceiverArg,
}

/// nn::swkbd::InputFormArg
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InputFormArg {
    pub kind: InputFormType,
    /// Initial cursor position, -1 for the end.
    pub cursor_pos: i32,
    /// Initial text.
    pub initial_text: *const u16,
    /// Hint text, up to [GUIDE_STRING_LENGTH_MAX] characters.
    pub hint_text: *const u16,
    /// Maximum number of characters, -1 for unlimited.
    pub max_text_length: i32,
    pub password_mode: PasswordMode,
    /// Separator bits between the boxes of [InputFormType::Monospace], e.g. `0x88` for groups of 4, 4, 4.
    pub separate_flags: u32,
    /// Shows the cursor in [InputFormType::Monospace].
    pub draw_cursor: bool,
    /// Selects the initial text in [InputFormType::Proportional].
    pub select_all_text: bool,
    /// Shows copy and paste buttons in [InputFormType::Proportional].
    pub copy_paste: bool,
}

impl Default for InputFormArg {
    fn default() -> Self {
        Self {
            kind: InputFormType::Proportional,
            cursor_pos: -1,
            initial_text: core::ptr::null(),
            hint_text: core::ptr::null(),
            max_text_length: -1,
            password_mode: PasswordMode::Clear,
            separate_flags: 0,
            draw_cursor: false,
            select_all_text: false,
            copy_paste: false,
        }
    }
}

/// nn::swkbd::AppearArg
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AppearArg {
    pub keyboard: KeyboardArg,
    pub input_form: InputFormArg,
}

/// nn::swkbd::ControllerInfo
///
/// The statuses are copied.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ControllerInfo {
    /// Touch positions must already be calibrated.
    pub vpad: *const crate::vpad::Status,
    pub kpad: [*const crate::padscore::kpad::Status; 4],
}

impl Default for ControllerInfo {
    fn default() -> Self {
        Self {
            vpad: core::ptr::null(),
            kpad: [core::ptr::null(); 4],
        }
    }
}

/// nn::swkbd::DrawStringInfo
///
/// Ranges of unfixed text, -1 if unused.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DrawStringInfo {
    pub unfixed_from: i32,
    pub unfixed_to: i32,
    pub kana_from: i32,
    pub kana_to: i32,
    pub dead_key_pos: i32,
    pub cell_phone_pos: i32,
    /// Whether the unfixed text is Japanese.
    pub unfixed_jp: bool,
}

impl Default for DrawStringInfo {
    fn default() -> Self {
        Self {
            unfixed_from: -1,
            unfixed_to: -1,
            kana_from: -1,
            kana_to: -1,
            dead_key_pos: -1,
            cell_phone_pos: -1,
            unfixed_jp: true,
        }
    }
}

/// nn::swkbd::IEventReceiver
///
/// C++ interface receiving text change and cursor move events.
#[doc(alias = "IEventReceiver")]
#[repr(C)]
pub struct EventReceiver {
    _private: [u8; 0],
}

/// nn::swkbd::IControllerEventObj
///
/// C++ interface for custom controller event handling.
#[doc(alias = "IControllerEventObj")]
#[repr(C)]
pub struct ControllerEventObj {
    _private: [u8; 0],
}

/// nn::swkbd::ISoundObj
///
/// C++ interface for custom keyboard sounds.
#[doc(alias = "ISoundObj")]
#[repr(C)]
pub struct SoundObj {
    _private: [u8; 0],
}

// TODO: replace with FSClient from coreinit::fs
pub use crate::nn::save::FsClient;

const _: () = {
    assert!(size_of::<KeyboardCondition>() == 0x8);
    assert!(size_of::<UserWord>() == 0x64);
    assert!(size_of::<DrawStringInfo>() == 0x1C);
};

unsafe extern "C" {
    /// SwkbdCreate
    ///
    /// `work_memory` must be [get_work_memory_size] bytes. Call [set_version] first.
    ///
    /// Call from a thread with lower priority than the main thread, after `GX2Init`, and keep that thread alive until [destroy]. Overwrites the `OSDynLoad` allocator.
    #[doc(alias = "SwkbdCreate", alias = "Create")]
    #[link_name = "SwkbdCreate__3RplFPUcQ3_2nn5swkbd10RegionTypeUiP8FSClient"]
    pub unsafe fn create(
        work_memory: *mut u8,
        region: RegionType,
        load_off_flags: LoadOffFlags,
        fs_client: *mut FsClient,
    );

    /// SwkbdDestroy
    #[doc(alias = "SwkbdDestroy", alias = "Destroy")]
    #[link_name = "SwkbdDestroy__3RplFv"]
    pub unsafe fn destroy();

    /// SwkbdSetVersion
    ///
    /// Call with 3 before [create].
    #[doc(alias = "SwkbdSetVersion")]
    #[link_name = "SwkbdSetVersion__3RplFi"]
    pub unsafe fn set_version(version: ffi::c_int);

    /// SwkbdCalc
    ///
    /// Call every frame.
    #[doc(alias = "SwkbdCalc", alias = "Calc")]
    #[link_name = "SwkbdCalc__3RplFRCQ3_2nn5swkbd14ControllerInfo"]
    pub unsafe fn calc(info: *const ControllerInfo);

    /// SwkbdDrawTV
    #[doc(alias = "SwkbdDrawTV", alias = "DrawTV")]
    #[link_name = "SwkbdDrawTV__3RplFv"]
    pub unsafe fn draw_tv();

    /// SwkbdDrawDRC
    #[doc(alias = "SwkbdDrawDRC", alias = "DrawDRC")]
    #[link_name = "SwkbdDrawDRC__3RplFv"]
    pub unsafe fn draw_drc();

    /// SwkbdAppearInputForm
    ///
    /// Returns false if already appearing or visible.
    #[doc(alias = "SwkbdAppearInputForm", alias = "AppearInputForm")]
    #[link_name = "SwkbdAppearInputForm__3RplFRCQ3_2nn5swkbd9AppearArg"]
    pub unsafe fn appear_input_form(arg: *const AppearArg) -> bool;

    /// SwkbdDisappearInputForm
    ///
    /// Returns false if already disappearing or hidden.
    #[doc(alias = "SwkbdDisappearInputForm", alias = "DisappearInputForm")]
    #[link_name = "SwkbdDisappearInputForm__3RplFv"]
    pub unsafe fn disappear_input_form() -> bool;

    /// SwkbdGetStateInputForm
    #[doc(alias = "SwkbdGetStateInputForm", alias = "GetStateInputForm")]
    #[link_name = "SwkbdGetStateInputForm__3RplFv"]
    pub unsafe fn get_state_input_form() -> State;

    /// SwkbdGetInputFormString
    ///
    /// Null-terminated.
    #[doc(alias = "SwkbdGetInputFormString", alias = "GetInputFormString")]
    #[link_name = "SwkbdGetInputFormString__3RplFv"]
    pub unsafe fn get_input_form_string() -> *const u16;

    /// SwkbdSetInputFormString
    ///
    /// Replaces the text and moves the cursor to the end. Null is an empty string.
    #[doc(alias = "SwkbdSetInputFormString", alias = "SetInputFormString")]
    #[link_name = "SwkbdSetInputFormString__3RplFPCw"]
    pub unsafe fn set_input_form_string(string: *const u16);

    /// SwkbdAppearKeyboard
    ///
    /// Shows the keyboard without an input form.
    #[doc(alias = "SwkbdAppearKeyboard", alias = "AppearKeyboard")]
    #[link_name = "SwkbdAppearKeyboard__3RplFRCQ3_2nn5swkbd11KeyboardArg"]
    pub unsafe fn appear_keyboard(arg: *const KeyboardArg) -> bool;

    /// SwkbdDisappearKeyboard
    #[doc(alias = "SwkbdDisappearKeyboard", alias = "DisappearKeyboard")]
    #[link_name = "SwkbdDisappearKeyboard__3RplFv"]
    pub unsafe fn disappear_keyboard() -> bool;

    /// SwkbdGetStateKeyboard
    #[doc(alias = "SwkbdGetStateKeyboard", alias = "GetStateKeyboard")]
    #[link_name = "SwkbdGetStateKeyboard__3RplFv"]
    pub unsafe fn get_state_keyboard() -> State;

    /// SwkbdIsDecideOkButton
    ///
    /// `by_select_cursor` (nullable) is set if pressed with the selection cursor.
    #[doc(alias = "SwkbdIsDecideOkButton", alias = "IsDecideOkButton")]
    #[link_name = "SwkbdIsDecideOkButton__3RplFPb"]
    pub unsafe fn is_decide_ok_button(by_select_cursor: *mut bool) -> bool;

    /// SwkbdIsDecideCancelButton
    ///
    /// `by_select_cursor` (nullable) is set if pressed with the selection cursor.
    #[doc(alias = "SwkbdIsDecideCancelButton", alias = "IsDecideCancelButton")]
    #[link_name = "SwkbdIsDecideCancelButton__3RplFPb"]
    pub unsafe fn is_decide_cancel_button(by_select_cursor: *mut bool) -> bool;

    /// SwkbdSetEnableOkButton
    #[doc(alias = "SwkbdSetEnableOkButton", alias = "SetEnableOkButton")]
    #[link_name = "SwkbdSetEnableOkButton__3RplFb"]
    pub unsafe fn set_enable_ok_button(enable: bool);

    /// SwkbdSetControllerRemo
    ///
    /// Switches the Wii Remote operating a keyboard shown on the TV.
    #[doc(alias = "SwkbdSetControllerRemo", alias = "SetControllerRemo")]
    #[link_name = "SwkbdSetControllerRemo__3RplFQ3_2nn5swkbd14ControllerType"]
    pub unsafe fn set_controller_remo(controller: ControllerType);

    /// SwkbdInitLearnDic
    ///
    /// Initializes a new learning dictionary buffer of [LEARN_DIC_SIZE] bytes.
    #[doc(alias = "SwkbdInitLearnDic", alias = "InitLearnDic")]
    #[link_name = "SwkbdInitLearnDic__3RplFPv"]
    pub unsafe fn init_learn_dic(buffer: *mut ffi::c_void) -> bool;

    /// SwkbdIsNeedCalcSubThreadFont
    #[doc(
        alias = "SwkbdIsNeedCalcSubThreadFont",
        alias = "IsNeedCalcSubThreadFont"
    )]
    #[link_name = "SwkbdIsNeedCalcSubThreadFont__3RplFv"]
    pub unsafe fn is_need_calc_sub_thread_font() -> bool;

    /// SwkbdCalcSubThreadFont
    ///
    /// Generates fonts. Call from a thread with higher priority than the one calling [calc_sub_thread_predict].
    #[doc(alias = "SwkbdCalcSubThreadFont", alias = "CalcSubThreadFont")]
    #[link_name = "SwkbdCalcSubThreadFont__3RplFv"]
    pub unsafe fn calc_sub_thread_font();

    /// SwkbdIsNeedCalcSubThreadPredict
    #[doc(
        alias = "SwkbdIsNeedCalcSubThreadPredict",
        alias = "IsNeedCalcSubThreadPredict"
    )]
    #[link_name = "SwkbdIsNeedCalcSubThreadPredict__3RplFv"]
    pub unsafe fn is_need_calc_sub_thread_predict() -> bool;

    /// SwkbdCalcSubThreadPredict
    ///
    /// Runs predictive text input. Call from a thread with lower priority than the main thread.
    #[doc(alias = "SwkbdCalcSubThreadPredict", alias = "CalcSubThreadPredict")]
    #[link_name = "SwkbdCalcSubThreadPredict__3RplFv"]
    pub unsafe fn calc_sub_thread_predict();

    /// SwkbdSetUserControllerEventObj
    #[doc(
        alias = "SwkbdSetUserControllerEventObj",
        alias = "SetUserControllerEventObj"
    )]
    #[link_name = "SwkbdSetUserControllerEventObj__3RplFPQ3_2nn5swkbd19IControllerEventObj"]
    pub unsafe fn set_user_controller_event_obj(obj: *mut ControllerEventObj);

    /// SwkbdSetUserSoundObj
    ///
    /// Null uses the default sounds.
    #[doc(alias = "SwkbdSetUserSoundObj", alias = "SetUserSoundObj")]
    #[link_name = "SwkbdSetUserSoundObj__3RplFPQ3_2nn5swkbd9ISoundObj"]
    pub unsafe fn set_user_sound_obj(obj: *mut SoundObj);

    /// SwkbdMuteAllSound
    #[doc(alias = "SwkbdMuteAllSound", alias = "MuteAllSound")]
    #[link_name = "SwkbdMuteAllSound__3RplFb"]
    pub unsafe fn mute_all_sound(mute: bool);

    /// SwkbdSetReceiver
    ///
    /// Set again whenever the application edits the text.
    #[doc(alias = "SwkbdSetReceiver", alias = "SetReceiver")]
    #[link_name = "SwkbdSetReceiver__3RplFRCQ3_2nn5swkbd11ReceiverArg"]
    pub unsafe fn set_receiver(arg: *const ReceiverArg);

    /// SwkbdIsKeyboardTarget
    #[doc(alias = "SwkbdIsKeyboardTarget", alias = "IsKeyboardTarget")]
    #[link_name = "SwkbdIsKeyboardTarget__3RplFPQ3_2nn5swkbd14IEventReceiver"]
    pub unsafe fn is_keyboard_target(receiver: *mut EventReceiver) -> bool;

    /// SwkbdGetKeyboardCondition
    #[doc(alias = "SwkbdGetKeyboardCondition", alias = "GetKeyboardCondition")]
    #[link_name = "SwkbdGetKeyboardCondition__3RplFPQ3_2nn5swkbd17KeyboardCondition"]
    pub unsafe fn get_keyboard_condition(condition: *mut KeyboardCondition);

    /// SwkbdIsCoveredWithSubWindow
    ///
    /// Ignore input form operations while true.
    #[doc(
        alias = "SwkbdIsCoveredWithSubWindow",
        alias = "IsCoveredWithSubWindow"
    )]
    #[link_name = "SwkbdIsCoveredWithSubWindow__3RplFv"]
    pub unsafe fn is_covered_with_sub_window() -> bool;

    /// SwkbdIsSelectCursorActive
    ///
    /// Ignore input form operations while true.
    #[doc(alias = "SwkbdIsSelectCursorActive", alias = "IsSelectCursorActive")]
    #[link_name = "SwkbdIsSelectCursorActive__3RplFv"]
    pub unsafe fn is_select_cursor_active() -> bool;

    /// SwkbdInactivateSelectCursor
    #[doc(
        alias = "SwkbdInactivateSelectCursor",
        alias = "InactivateSelectCursor"
    )]
    #[link_name = "SwkbdInactivateSelectCursor__3RplFv"]
    pub unsafe fn inactivate_select_cursor();

    /// SwkbdConfirmUnfixAll
    ///
    /// Finalizes the unfixed text.
    #[doc(alias = "SwkbdConfirmUnfixAll", alias = "ConfirmUnfixAll")]
    #[link_name = "SwkbdConfirmUnfixAll__3RplFv"]
    pub unsafe fn confirm_unfix_all();

    /// SwkbdGetDrawStringInfo
    #[doc(alias = "SwkbdGetDrawStringInfo", alias = "GetDrawStringInfo")]
    #[link_name = "SwkbdGetDrawStringInfo__3RplFPQ3_2nn5swkbd14DrawStringInfo"]
    pub unsafe fn get_draw_string_info(info: *mut DrawStringInfo);

    /// SwkbdSetCursorPos
    #[doc(alias = "SwkbdSetCursorPos", alias = "SetCursorPos")]
    #[link_name = "SwkbdSetCursorPos__3RplFi"]
    pub unsafe fn set_cursor_pos(pos: ffi::c_int);

    /// SwkbdSetSelectFrom
    ///
    /// -1 if no text is selected.
    #[doc(alias = "SwkbdSetSelectFrom", alias = "SetSelectFrom")]
    #[link_name = "SwkbdSetSelectFrom__3RplFi"]
    pub unsafe fn set_select_from(from: ffi::c_int);
}

imports_section!(
    "swkbd",
    [
        "SwkbdAppearInputForm__3RplFRCQ3_2nn5swkbd9AppearArg",
        "SwkbdAppearKeyboard__3RplFRCQ3_2nn5swkbd11KeyboardArg",
        "SwkbdCalcSubThreadFont__3RplFv",
        "SwkbdCalcSubThreadPredict__3RplFv",
        "SwkbdCalc__3RplFRCQ3_2nn5swkbd14ControllerInfo",
        "SwkbdConfirmUnfixAll__3RplFv",
        "SwkbdCreate__3RplFPUcQ3_2nn5swkbd10RegionTypeUiP8FSClient",
        "SwkbdDestroy__3RplFv",
        "SwkbdDisappearInputForm__3RplFv",
        "SwkbdDisappearKeyboard__3RplFv",
        "SwkbdDrawDRC__3RplFv",
        "SwkbdDrawTV__3RplFv",
        "SwkbdGetDrawStringInfo__3RplFPQ3_2nn5swkbd14DrawStringInfo",
        "SwkbdGetInputFormString__3RplFv",
        "SwkbdGetKeyboardCondition__3RplFPQ3_2nn5swkbd17KeyboardCondition",
        "SwkbdGetStateInputForm__3RplFv",
        "SwkbdGetStateKeyboard__3RplFv",
        "SwkbdInactivateSelectCursor__3RplFv",
        "SwkbdInitLearnDic__3RplFPv",
        "SwkbdIsCoveredWithSubWindow__3RplFv",
        "SwkbdIsDecideCancelButton__3RplFPb",
        "SwkbdIsDecideOkButton__3RplFPb",
        "SwkbdIsKeyboardTarget__3RplFPQ3_2nn5swkbd14IEventReceiver",
        "SwkbdIsNeedCalcSubThreadFont__3RplFv",
        "SwkbdIsNeedCalcSubThreadPredict__3RplFv",
        "SwkbdIsSelectCursorActive__3RplFv",
        "SwkbdMuteAllSound__3RplFb",
        "SwkbdSetControllerRemo__3RplFQ3_2nn5swkbd14ControllerType",
        "SwkbdSetCursorPos__3RplFi",
        "SwkbdSetEnableOkButton__3RplFb",
        "SwkbdSetInputFormString__3RplFPCw",
        "SwkbdSetReceiver__3RplFRCQ3_2nn5swkbd11ReceiverArg",
        "SwkbdSetSelectFrom__3RplFi",
        "SwkbdSetUserControllerEventObj__3RplFPQ3_2nn5swkbd19IControllerEventObj",
        "SwkbdSetUserSoundObj__3RplFPQ3_2nn5swkbd9ISoundObj",
        "SwkbdSetVersion__3RplFi"
    ],
    []
);
