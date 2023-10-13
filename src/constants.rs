
use serde::{Serialize, Deserialize};
use serde::de::value::U64Deserializer;
use crate::constants::NetErr::ErrNetTx;
pub struct Pos2 {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CmdMouse {
    pub(crate) head: CmdHead,
    pub(crate) mouse: SoftMouse,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CmdKeyboard {
    head: CmdHead,
    keyboard: SoftKeyboard,
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CmdHead {
    pub mac: u32,           //盒子的mac地址（必须）
    pub rand: u32,          //随机值
    pub indexpts: u32,      //时间戳
    pub cmd: u32,           //指令码
}


#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SoftMouse {
    pub button: i32,
    pub x: i32,
    pub y: i32,
    pub wheel: i32,
    pub point: [i32; 10],  // 用于贝塞尔曲线控制（预留5阶导）
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SoftKeyboard {
    pub ctrl: i8,
    pub resvel: i8,
    pub button: [i8; 10],
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ClientTx {
    pub head: CmdHead,
}


#[derive(Debug, Serialize, Deserialize, Default)]
pub struct StandardMouseReport {
    pub report_id: u8,
    pub buttons: u8,  // 8个可用按钮
    pub x: i16,       // -32767 到 32767
    pub y: i16,       // -32767 到 32767
    pub wheel: i16,   // -32767 到 32767
}
#[derive(Debug, Serialize, Deserialize, Default)]
struct StandardKeyboardReport {
    pub report_id: u8,
    pub buttons: u8,      // 8个按钮控制键
    pub data: [u8; 10],    // 常规按键数据
}

pub fn str_to_hex(pb_src: &str, n_len: usize) -> u32 {
    let pb_src = pb_src.as_bytes();
    let mut pb_dest: [u32; 16] = [0; 16];
    for i in 0..n_len {
        let h1 = pb_src[2 * i];
        let h2 = pb_src[2 * i + 1];
        let mut s1 = h1.to_ascii_uppercase() - 0x30;
        if s1 > 9 { s1 -= 7 }
        let mut s2 = h2.to_ascii_uppercase() - 0x30;
        if s2 > 9 { s2 -= 7 }
        pb_dest[i] = (s1 * 16 + s2) as u32;
    }

    pb_dest[0] << 24 | pb_dest[1] << 16 | pb_dest[2] << 8 | pb_dest[3]
}

pub fn net_rx_return_handle(rx: &CmdHead, tx: &CmdHead) -> NetErr {
    if rx.cmd != tx.cmd {
        return  NetErr::ErrNetCmd;//命令码错误
    }

    if rx.indexpts != tx.indexpts {
        return  NetErr::ErrNetPts;//时间戳错误
    }
    return NetErr::Success
}
#[derive(Debug)]
pub enum NetErr {
    ErrCreatSocket,	        //创建socket失败
    ErrNetVersion,	    	//socket版本错误
    ErrNetTx,	        	//socket发送错误
    ErrNetRxTimeout,		//socket接收超时
    ErrNetCmd,			    //命令错误
    ErrNetPts,		    	//时间戳错误
    Success,				//正常执行
    UsbDevTxTimeout,		//USB device发送失败
}

pub mod cmd {
    // 命令码
    pub const CONNECT: u32 = 0xaf3c2828; // 连接盒子
    pub const MOUSE_MOVE: u32 = 0xaede7345; // 鼠标移动
    pub const MOUSE_LEFT: u32 = 0x9823AE8D; // 鼠标左键控制
    pub const MOUSE_MIDDLE: u32 = 0x97a3AE8D; // 鼠标中键控制
    pub const MOUSE_RIGHT: u32 = 0x238d8212; // 鼠标右键控制
    pub const MOUSE_WHEEL: u32 = 0xffeead38; // 鼠标滚轮控制
    pub const MOUSE_AUTOMOVE: u32 = 0xaede7346; // 鼠标自动模拟人工移动控制
    pub const KEYBOARD_ALL: u32 = 0x123c2c2f; // 键盘所有参数控制
    pub const REBOOT: u32 = 0xaa8855aa; // 盒子重启
    pub const BAZER_MOVE: u32 = 0xa238455a; // 鼠标贝塞尔移动
    pub const MONITOR: u32 = 0x27388020; // 监控盒子上的物理键鼠数据
    pub const DEBUG: u32 = 0x27382021; // 开启调试信息
    pub const MASK_MOUSE: u32 = 0x23234343; // 屏蔽物理键鼠
    pub const UNMASK_ALL: u32 = 0x23344343; // 解除屏蔽物理键鼠
    pub const SETCONFIG: u32 = 0x1d3d3323; // 设置IP配置信息
    pub const SHOWPIC: u32 = 0x12334883; // 显示图片
}

pub mod keyboard_table {
    pub const KEY_NONE: u32 = 0x00;
    pub const KEY_ERRORROLLOVER: u32 = 0x01;
    pub const KEY_POSTFAIL: u32 = 0x02;
    pub const KEY_ERRORUNDEFINED: u32 = 0x03;
    pub const KEY_A: u32 = 0x04;
    pub const KEY_B: u32 = 0x05;
    pub const KEY_C: u32 = 0x06;
    pub const KEY_D: u32 = 0x07;
    pub const KEY_E: u32 = 0x08;
    pub const KEY_F: u32 = 0x09;
    pub const KEY_G: u32 = 0x0A;
    pub const KEY_H: u32 = 0x0B;
    pub const KEY_I: u32 = 0x0C;
    pub const KEY_J: u32 = 0x0D;
    pub const KEY_K: u32 = 0x0E;
    pub const KEY_L: u32 = 0x0F;
    pub const KEY_M: u32 = 0x10;
    pub const KEY_N: u32 = 0x11;
    pub const KEY_O: u32 = 0x12;
    pub const KEY_P: u32 = 0x13;
    pub const KEY_Q: u32 = 0x14;
    pub const KEY_R: u32 = 0x15;
    pub const KEY_S: u32 = 0x16;
    pub const KEY_T: u32 = 0x17;
    pub const KEY_U: u32 = 0x18;
    pub const KEY_V: u32 = 0x19;
    pub const KEY_W: u32 = 0x1A;
    pub const KEY_X: u32 = 0x1B;
    pub const KEY_Y: u32 = 0x1C;
    pub const KEY_Z: u32 = 0x1D;
    pub const KEY_1_EXCLAMATION_MARK: u32 = 0x1E;
    pub const KEY_2_AT: u32 = 0x1F;
    pub const KEY_3_NUMBER_SIGN: u32 = 0x20;
    pub const KEY_4_DOLLAR: u32 = 0x21;
    pub const KEY_5_PERCENT: u32 = 0x22;
    pub const KEY_6_CARET: u32 = 0x23;
    pub const KEY_7_AMPERSAND: u32 = 0x24;
    pub const KEY_8_ASTERISK: u32 = 0x25;
    pub const KEY_9_OPARENTHESIS: u32 = 0x26;
    pub const KEY_0_CPARENTHESIS: u32 = 0x27;
    pub const KEY_ENTER: u32 = 0x28;
    pub const KEY_ESCAPE: u32 = 0x29;
    pub const KEY_BACKSPACE: u32 = 0x2A;
    pub const KEY_TAB: u32 = 0x2B;
    pub const KEY_SPACEBAR: u32 = 0x2C;
    pub const KEY_MINUS_UNDERSCORE: u32 = 0x2D;
    pub const KEY_EQUAL_PLUS: u32 = 0x2E;
    pub const KEY_OBRACKET_AND_OBRACE: u32 = 0x2F;
    pub const KEY_CBRACKET_AND_CBRACE: u32 = 0x30;
    pub const KEY_BACKSLASH_VERTICAL_BAR: u32 = 0x31;
    pub const KEY_NONUS_NUMBER_SIGN_TILDE: u32 = 0x32;
    pub const KEY_SEMICOLON_COLON: u32 = 0x33;
    pub const KEY_SINGLE_AND_DOUBLE_QUOTE: u32 = 0x34;
    pub const KEY_GRAVE_ACCENT_AND_TILDE: u32 = 0x35;
    pub const KEY_COMMA_AND_LESS: u32 = 0x36;
    pub const KEY_DOT_GREATER: u32 = 0x37;
    pub const KEY_SLASH_QUESTION: u32 = 0x38;
    pub const KEY_CAPS_LOCK: u32 = 0x39;
    pub const KEY_F1: u32 = 0x3A;
    pub const KEY_F2: u32 = 0x3B;
    pub const KEY_F3: u32 = 0x3C;
    pub const KEY_F4: u32 = 0x3D;
    pub const KEY_F5: u32 = 0x3E;
    pub const KEY_F6: u32 = 0x3F;
    pub const KEY_F7: u32 = 0x40;
    pub const KEY_F8: u32 = 0x41;
    pub const KEY_F9: u32 = 0x42;
    pub const KEY_F10: u32 = 0x43;
    pub const KEY_F11: u32 = 0x44;
    pub const KEY_F12: u32 = 0x45;
    pub const KEY_PRINTSCREEN: u32 = 0x46;
    pub const KEY_SCROLL_LOCK: u32 = 0x47;
    pub const KEY_PAUSE: u32 = 0x48;
    pub const KEY_INSERT: u32 = 0x49;
    pub const KEY_HOME: u32 = 0x4A;
    pub const KEY_PAGEUP: u32 = 0x4B;
    pub const KEY_DELETE: u32 = 0x4C;
    pub const KEY_END1: u32 = 0x4D;
    pub const KEY_PAGEDOWN: u32 = 0x4E;
    pub const KEY_RIGHTARROW: u32 = 0x4F;
    pub const KEY_LEFTARROW: u32 = 0x50;
    pub const KEY_DOWNARROW: u32 = 0x51;
    pub const KEY_UPARROW: u32 = 0x52;
    pub const KEY_KEYPAD_NUM_LOCK_AND_CLEAR: u32 = 0x53;
    pub const KEY_KEYPAD_SLASH: u32 = 0x54;
    pub const KEY_KEYPAD_ASTERIKS: u32 = 0x55;
    pub const KEY_KEYPAD_MINUS: u32 = 0x56;
    pub const KEY_KEYPAD_PLUS: u32 = 0x57;
    pub const KEY_KEYPAD_ENTER: u32 = 0x58;
    pub const KEY_KEYPAD_1_END: u32 = 0x59;
    pub const KEY_KEYPAD_2_DOWN_ARROW: u32 = 0x5A;
    pub const KEY_KEYPAD_3_PAGEDN: u32 = 0x5B;
    pub const KEY_KEYPAD_4_LEFT_ARROW: u32 = 0x5C;
    pub const KEY_KEYPAD_5: u32 = 0x5D;
    pub const KEY_KEYPAD_6_RIGHT_ARROW: u32 = 0x5E;
    pub const KEY_KEYPAD_7_HOME: u32 = 0x5F;
    pub const KEY_KEYPAD_8_UP_ARROW: u32 = 0x60;
    pub const KEY_KEYPAD_9_PAGEUP: u32 = 0x61;
    pub const KEY_KEYPAD_0_INSERT: u32 = 0x62;
    pub const KEY_KEYPAD_DECIMAL_SEPARATOR_DELETE: u32 = 0x63;
    pub const KEY_NONUS_BACK_SLASH_VERTICAL_BAR: u32 = 0x64;
    pub const KEY_APPLICATION: u32 = 0x65;
    pub const KEY_POWER: u32 = 0x66;
    pub const KEY_KEYPAD_EQUAL: u32 = 0x67;
    pub const KEY_F13: u32 = 0x68;
    pub const KEY_F14: u32 = 0x69;
    pub const KEY_F15: u32 = 0x6A;
    pub const KEY_F16: u32 = 0x6B;
    pub const KEY_F17: u32 = 0x6C;
    pub const KEY_F18: u32 = 0x6D;
    pub const KEY_F19: u32 = 0x6E;
    pub const KEY_F20: u32 = 0x6F;
    pub const KEY_F21: u32 = 0x70;
    pub const KEY_F22: u32 = 0x71;
    pub const KEY_F23: u32 = 0x72;
    pub const KEY_F24: u32 = 0x73;
    pub const KEY_EXECUTE: u32 = 0x74;
    pub const KEY_HELP: u32 = 0x75;
    pub const KEY_MENU: u32 = 0x76;
    pub const KEY_SELECT: u32 = 0x77;
    pub const KEY_STOP: u32 = 0x78;
    pub const KEY_AGAIN: u32 = 0x79;
    pub const KEY_UNDO: u32 = 0x7A;
    pub const KEY_CUT: u32 = 0x7B;
    pub const KEY_COPY: u32 = 0x7C;
    pub const KEY_PASTE: u32 = 0x7D;
    pub const KEY_FIND: u32 = 0x7E;
    pub const KEY_MUTE: u32 = 0x7F;
    pub const KEY_VOLUME_UP: u32 = 0x80;
    pub const KEY_VOLUME_DOWN: u32 = 0x81;
    pub const KEY_LOCKING_CAPS_LOCK: u32 = 0x82;
    pub const KEY_LOCKING_NUM_LOCK: u32 = 0x83;
    pub const KEY_LOCKING_SCROLL_LOCK: u32 = 0x84;
    pub const KEY_KEYPAD_COMMA: u32 = 0x85;
    pub const KEY_KEYPAD_EQUAL_SIGN: u32 = 0x86;
    pub const KEY_INTERNATIONAL1: u32 = 0x87;
    pub const KEY_INTERNATIONAL2: u32 = 0x88;
    pub const KEY_INTERNATIONAL3: u32 = 0x89;
    pub const KEY_INTERNATIONAL4: u32 = 0x8A;
    pub const KEY_INTERNATIONAL5: u32 = 0x8B;
    pub const KEY_INTERNATIONAL6: u32 = 0x8C;
    pub const KEY_INTERNATIONAL7: u32 = 0x8D;
    pub const KEY_INTERNATIONAL8: u32 = 0x8E;
    pub const KEY_INTERNATIONAL9: u32 = 0x8F;
    pub const KEY_LANG1: u32 = 0x90;
    pub const KEY_LANG2: u32 = 0x91;
    pub const KEY_LANG3: u32 = 0x92;
    pub const KEY_LANG4: u32 = 0x93;
    pub const KEY_LANG5: u32 = 0x94;
    pub const KEY_LANG6: u32 = 0x95;
    pub const KEY_LANG7: u32 = 0x96;
    pub const KEY_LANG8: u32 = 0x97;
    pub const KEY_LANG9: u32 = 0x98;
    pub const KEY_ALTERNATE_ERASE: u32 = 0x99;
    pub const KEY_SYSREQ: u32 = 0x9A;
    pub const KEY_CANCEL: u32 = 0x9B;
    pub const KEY_CLEAR: u32 = 0x9C;
    pub const KEY_PRIOR: u32 = 0x9D;
    pub const KEY_RETURN: u32 = 0x9E;
    pub const KEY_SEPARATOR: u32 = 0x9F;
    pub const KEY_OUT: u32 = 0xA0;
    pub const KEY_OPER: u32 = 0xA1;
    pub const KEY_CLEAR_AGAIN: u32 = 0xA2;
    pub const KEY_CRSEL: u32 = 0xA3;
    pub const KEY_EXSEL: u32 = 0xA4;
    pub const KEY_KEYPAD_00: u32 = 0xB0;
    pub const KEY_KEYPAD_000: u32 = 0xB1;
    pub const KEY_THOUSANDS_SEPARATOR: u32 = 0xB2;
    pub const KEY_DECIMAL_SEPARATOR: u32 = 0xB3;
    pub const KEY_CURRENCY_UNIT: u32 = 0xB4;
    pub const KEY_CURRENCY_SUB_UNIT: u32 = 0xB5;
    pub const KEY_KEYPAD_OPARENTHESIS: u32 = 0xB6;
    pub const KEY_KEYPAD_CPARENTHESIS: u32 = 0xB7;
    pub const KEY_KEYPAD_OBRACE: u32 = 0xB8;
    pub const KEY_KEYPAD_CBRACE: u32 = 0xB9;
    pub const KEY_KEYPAD_TAB: u32 = 0xBA;
    pub const KEY_KEYPAD_BACKSPACE: u32 = 0xBB;
    pub const KEY_KEYPAD_A: u32 = 0xBC;
    pub const KEY_KEYPAD_B: u32 = 0xBD;
    pub const KEY_KEYPAD_C: u32 = 0xBE;
    pub const KEY_KEYPAD_D: u32 = 0xBF;
    pub const KEY_KEYPAD_E: u32 = 0xC0;
    pub const KEY_KEYPAD_F: u32 = 0xC1;
    pub const KEY_KEYPAD_XOR: u32 = 0xC2;
    pub const KEY_KEYPAD_CARET: u32 = 0xC3;
    pub const KEY_KEYPAD_PERCENT: u32 = 0xC4;
    pub const KEY_KEYPAD_LESS: u32 = 0xC5;
    pub const KEY_KEYPAD_GREATER: u32 = 0xC6;
    pub const KEY_KEYPAD_AMPERSAND: u32 = 0xC7;
    pub const KEY_KEYPAD_LOGICAL_AND: u32 = 0xC8;
    pub const KEY_KEYPAD_VERTICAL_BAR: u32 = 0xC9;
    pub const KEY_KEYPAD_LOGIACL_OR: u32 = 0xCA;
    pub const KEY_KEYPAD_COLON: u32 = 0xCB;
    pub const KEY_KEYPAD_NUMBER_SIGN: u32 = 0xCC;
    pub const KEY_KEYPAD_SPACE: u32 = 0xCD;
    pub const KEY_KEYPAD_AT: u32 = 0xCE;
    pub const KEY_KEYPAD_EXCLAMATION_MARK: u32 = 0xCF;
    pub const KEY_KEYPAD_MEMORY_STORE: u32 = 0xD0;
    pub const KEY_KEYPAD_MEMORY_RECALL: u32 = 0xD1;
    pub const KEY_KEYPAD_MEMORY_CLEAR: u32 = 0xD2;
    pub const KEY_KEYPAD_MEMORY_ADD: u32 = 0xD3;
    pub const KEY_KEYPAD_MEMORY_SUBTRACT: u32 = 0xD4;
    pub const KEY_KEYPAD_MEMORY_MULTIPLY: u32 = 0xD5;
    pub const KEY_KEYPAD_MEMORY_DIVIDE: u32 = 0xD6;
    pub const KEY_KEYPAD_PLUSMINUS: u32 = 0xD7;
    pub const KEY_KEYPAD_CLEAR: u32 = 0xD8;
    pub const KEY_KEYPAD_CLEAR_ENTRY: u32 = 0xD9;
    pub const KEY_KEYPAD_BINARY: u32 = 0xDA;
    pub const KEY_KEYPAD_OCTAL: u32 = 0xDB;
    pub const KEY_KEYPAD_DECIMAL: u32 = 0xDC;
    pub const KEY_KEYPAD_HEXADECIMAL: u32 = 0xDD;
    pub const KEY_LEFTCONTROL: u32 = 0xE0;
    pub const KEY_LEFTSHIFT: u32 = 0xE1;
    pub const KEY_LEFTALT: u32 = 0xE2;
    pub const KEY_LEFT_GUI: u32 = 0xE3;
    pub const KEY_RIGHTCONTROL: u32 = 0xE4;
    pub const KEY_RIGHTSHIFT: u32 = 0xE5;
    pub const KEY_RIGHTALT: u32 = 0xE6;
    pub const KEY_RIGHT_GUI: u32 = 0xE7;
    pub const BIT0: u32 = 0x01;
    pub const BIT1: u32 = 0x02;
    pub const BIT2: u32 = 0x04;
    pub const BIT3: u32 = 0x08;
    pub const BIT4: u32 = 0x10;
    pub const BIT5: u32 = 0x20;
    pub const BIT6: u32 = 0x40;
    pub const BIT7: u32 = 0x80;
}