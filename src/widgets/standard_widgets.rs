/* automatically generated by rust-bindgen */

pub const xpWidgetClass_MainWindow: ::std::os::raw::c_int = 1;

#[derive(Copy, Clone)]
#[repr(u32)]
pub enum Enum_Unnamed1 {
    xpMainWindowStyle_MainWindow = 0,
    xpMainWindowStyle_Translucent = 1,
}
#[derive(Copy, Clone)]
#[repr(u32)]
pub enum Enum_Unnamed2 {
    xpProperty_MainWindowType = 1100,
    xpProperty_MainWindowHasCloseBoxes = 1200,
}
#[derive(Copy, Clone)]
#[repr(u32)]
pub enum Enum_Unnamed3 { xpMessage_CloseButtonPushed = 1200, }


pub const xpWidgetClass_SubWindow: ::std::os::raw::c_int = 2;

#[derive(Copy, Clone)]
#[repr(u32)]
pub enum Enum_Unnamed4 {
    xpSubWindowStyle_SubWindow = 0,
    xpSubWindowStyle_Screen = 2,
    xpSubWindowStyle_ListView = 3,
}
#[derive(Copy, Clone)]
#[repr(u32)]
pub enum Enum_Unnamed5 { xpProperty_SubWindowType = 1200, }

pub const xpWidgetClass_Button: ::std::os::raw::c_int = 3;

#[derive(Copy, Clone)]
#[repr(u32)]
pub enum Enum_Unnamed6 {
    xpPushButton = 0,
    xpRadioButton = 1,
    xpWindowCloseBox = 3,
    xpLittleDownArrow = 5,
    xpLittleUpArrow = 6,
}
#[derive(Copy, Clone)]
#[repr(u32)]
pub enum Enum_Unnamed7 {
    xpButtonBehaviorPushButton = 0,
    xpButtonBehaviorCheckBox = 1,
    xpButtonBehaviorRadioButton = 2,
}
#[derive(Copy, Clone)]
#[repr(u32)]
pub enum Enum_Unnamed8 {
    xpProperty_ButtonType = 1300,
    xpProperty_ButtonBehavior = 1301,
    xpProperty_ButtonState = 1302,
}
#[derive(Copy, Clone)]
#[repr(u32)]
pub enum Enum_Unnamed9 {
    xpMsg_PushButtonPressed = 1300,
    xpMsg_ButtonStateChanged = 1301,
}


pub const xpWidgetClass_TextField: ::std::os::raw::c_int = 4;

#[derive(Copy, Clone)]
#[repr(u32)]
pub enum Enum_Unnamed10 {
    xpTextEntryField = 0,
    xpTextTransparent = 3,
    xpTextTranslucent = 4,
}
#[derive(Copy, Clone)]
#[repr(u32)]
pub enum Enum_Unnamed11 {
    xpProperty_EditFieldSelStart = 1400,
    xpProperty_EditFieldSelEnd = 1401,
    xpProperty_EditFieldSelDragStart = 1402,
    xpProperty_TextFieldType = 1403,
    xpProperty_PasswordMode = 1404,
    xpProperty_MaxCharacters = 1405,
    xpProperty_ScrollPosition = 1406,
    xpProperty_Font = 1407,
    xpProperty_ActiveEditSide = 1408,
}
#[derive(Copy, Clone)]
#[repr(u32)]
pub enum Enum_Unnamed12 { xpMsg_TextFieldChanged = 1400, }


pub const xpWidgetClass_ScrollBar: ::std::os::raw::c_int = 5;

#[derive(Copy, Clone)]
#[repr(u32)]
pub enum Enum_Unnamed13 {
    xpScrollBarTypeScrollBar = 0,
    xpScrollBarTypeSlider = 1,
}
#[derive(Copy, Clone)]
#[repr(u32)]
pub enum Enum_Unnamed14 {
    xpProperty_ScrollBarSliderPosition = 1500,
    xpProperty_ScrollBarMin = 1501,
    xpProperty_ScrollBarMax = 1502,
    xpProperty_ScrollBarPageAmount = 1503,
    xpProperty_ScrollBarType = 1504,
    xpProperty_ScrollBarSlop = 1505,
}
#[derive(Copy, Clone)]
#[repr(u32)]
pub enum Enum_Unnamed15 { xpMsg_ScrollBarSliderPositionChanged = 1500, }
#[derive(Copy, Clone)]
#[repr(u32)]
pub enum Enum_Unnamed16 { xpProperty_CaptionLit = 1600, }
#[derive(Copy, Clone)]
#[repr(u32)]
pub enum Enum_Unnamed17 {
    xpShip = 4,
    xpILSGlideScope = 5,
    xpMarkerLeft = 6,
    xp_Airport = 7,
    xpNDB = 8,
    xpVOR = 9,
    xpRadioTower = 10,
    xpAircraftCarrier = 11,
    xpFire = 12,
    xpMarkerRight = 13,
    xpCustomObject = 14,
    xpCoolingTower = 15,
    xpSmokeStack = 16,
    xpBuilding = 17,
    xpPowerLine = 18,
    xpVORWithCompassRose = 19,
    xpOilPlatform = 21,
    xpOilPlatformSmall = 22,
    xpWayPoint = 23,
}
#[derive(Copy, Clone)]
#[repr(u32)]
pub enum Enum_Unnamed18 { xpProperty_GeneralGraphicsType = 1700, }
#[derive(Copy, Clone)]
#[repr(u32)]
pub enum Enum_Unnamed19 {
    xpProperty_ProgressPosition = 1800,
    xpProperty_ProgressMin = 1801,
    xpProperty_ProgressMax = 1802,
}
