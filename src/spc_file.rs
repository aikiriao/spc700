use crate::types::*;

/// SPCファイルタグ
#[derive(Debug, Clone)]
pub enum SPCFileTag {
    /// ID666フォーマット
    ID666 = 0x1A,
    /// その他
    Other = 0x1B,
}

/// エミュレータの種類
#[derive(Debug, Clone)]
pub enum EmuratorType {
    /// 不明
    Unknown = 0x00,
    /// ZSNES
    ZSNES = 0x01,
    /// Snes9x
    Snes9x = 0x02,
    /// ZST2SPC
    ZST2SPC = 0x03,
    /// その他
    Other = 0x04,
    /// SNEShout
    SNEShout = 0x05,
    /// ZSNES/W
    ZSNESW = 0x06,
    /// Snes9xpp
    Snes9xpp = 0x07,
    /// SNESGT
    SNESGT = 0x08,
}

/// SPCファイルヘッダ
#[derive(Debug, Clone)]
pub struct SPCFileHeader {
    /// ヘッダ情報
    pub info: [u8; 33],
    /// タグの種類
    pub tag: SPCFileTag,
    /// タグのバージョン
    pub tag_version: u8,
    /// SPCを保存した直前のレジスタ
    pub spc_register: SPCRegister,
    /// 曲のタイトル
    pub music_title: [u8; 32],
    /// ゲームのタイトル
    pub game_title: [u8; 32],
    /// SPCファイルの作成者
    pub creator: [u8; 16],
    /// コメント
    pub comment: [u8; 32],
    /// SPCファイル生成日(1-31)
    pub generate_date: u8,
    /// SPCファイル生成日(1-12)
    pub generate_month: u8,
    /// SPCファイル生成年(1-9999)
    pub generate_year: u16,
    /// 曲の演奏時間（秒）
    pub duration: u16,
    /// フェードアウト時間（ミリ秒）
    pub fadeout_time: u32,
    /// 作曲者
    pub composer: [u8; 32],
    /// 初期チャンネル無効
    pub initial_channel_invalid: u8,
    /// 生成したエミュレータの種類
    pub emurator_type: EmuratorType,
}

/// SPCファイル
#[derive(Debug, Clone)]
pub struct SPCFile {
    /// SPCファイルヘッダ
    pub header: SPCFileHeader,
    /// 64KB RAM
    pub ram: [u8; 65536],
    /// DSPレジスタ
    pub dsp_register: [u8; 128],
    /// XRAMバッファ
    pub xram_buffer: [u8; 64],
}

// 10進文字列からu64を生成
fn u8array_to_numeric(data: &[u8]) -> Option<u64> {
    // 末尾のヌル文字を読み飛ばし
    let mut i = 0;
    while data[data.len() - 1 - i] == 0 {
        i += 1;
        if i == (data.len() - 1) {
            return None;
        }
    }

    // 数字文字列の1桁目から読み取り
    let mut ret = 0;
    let mut base = 1;
    while i < data.len() {
        let chr = data[data.len() - 1 - i];
        if chr >= b'0' && chr <= b'9' {
            ret += base * ((chr - b'0') as u64);
        } else {
            return None;
        }
        base *= 10;
        i += 1;
    }

    Some(ret)
}

/// SPCファイルヘッダのパース
fn parse_spc_header(data: &[u8]) -> Option<SPCFileHeader> {
    // サイズチェック
    if data.len() < 256 {
        return None;
    }

    // テキストかバイナリかを日付の表記とバージョン文字列で判定
    let binary = data[0x9E + 2] != b'/' || data[0x9E + 5] != b'/';

    if binary {
        Some(SPCFileHeader {
            info: data[0..33].try_into().unwrap(),
            tag: if data[0x23] == 0x1A {
                SPCFileTag::ID666
            } else {
                SPCFileTag::Other
            },
            tag_version: data[0x24],
            spc_register: SPCRegister {
                pc: make_u16_from_u8(&data[0x25..0x27]),
                a: data[0x27],
                x: data[0x28],
                y: data[0x29],
                psw: data[0x2A],
                sp: data[0x2B],
            },
            music_title: data[0x2E..0x2E + 32].try_into().unwrap(),
            game_title: data[0x4E..0x4E + 32].try_into().unwrap(),
            creator: data[0x6E..0x6E + 16].try_into().unwrap(),
            comment: data[0x7E..0x7E + 32].try_into().unwrap(),
            generate_date: data[0x9E],
            generate_month: data[0x9F],
            generate_year: make_u16_from_u8(&data[0xA0..0xA2]),
            duration: make_u16_from_u8(&data[0xA9..0xAB]),
            fadeout_time: ((data[0xAC] as u32) << 16)
                | ((data[0xAD] as u32) << 8)
                | (data[0xAE] as u32), // TODO: エンディアンは？
            composer: data[0xB0..0xB0 + 32].try_into().unwrap(),
            initial_channel_invalid: data[0xD0],
            emurator_type: match data[0xD1] {
                0x00 => EmuratorType::Unknown,
                0x01 => EmuratorType::ZSNES,
                0x02 => EmuratorType::Snes9x,
                0x03 => EmuratorType::ZST2SPC,
                0x04 => EmuratorType::Other,
                0x05 => EmuratorType::SNEShout,
                0x06 => EmuratorType::ZSNESW,
                0x07 => EmuratorType::Snes9xpp,
                0x08 => EmuratorType::SNESGT,
                _ => {
                    return None;
                }
            },
        })
    } else {
        Some(SPCFileHeader {
            info: data[0..33].try_into().unwrap(),
            tag: if data[0x23] == 0x1A {
                SPCFileTag::ID666
            } else {
                SPCFileTag::Other
            },
            tag_version: data[0x24],
            spc_register: SPCRegister {
                pc: make_u16_from_u8(&data[0x25..0x27]),
                a: data[0x27],
                x: data[0x28],
                y: data[0x29],
                psw: data[0x2A],
                sp: data[0x2B],
            },
            music_title: data[0x2E..0x2E + 32].try_into().unwrap(),
            game_title: data[0x4E..0x4E + 32].try_into().unwrap(),
            creator: data[0x6E..0x6E + 16].try_into().unwrap(),
            comment: data[0x7E..0x7E + 32].try_into().unwrap(),
            generate_date: if let Some(d) = u8array_to_numeric(&data[0x9E + 3..0x9E + 5]) {
                d as u8
            } else {
                0
            },
            generate_month: if let Some(m) = u8array_to_numeric(&data[0x9E..0x9E + 2]) {
                m as u8
            } else {
                0
            },
            generate_year: if let Some(y) = u8array_to_numeric(&data[0x9E + 6..0x9E + 11]) {
                y as u16
            } else {
                0
            },
            duration: if let Some(d) = u8array_to_numeric(&data[0xA9..0xA9 + 3]) {
                d as u16
            } else {
                0
            },
            fadeout_time: if let Some(f) = u8array_to_numeric(&data[0xAC..0xAC + 5]) {
                f as u32
            } else {
                0
            },
            composer: data[0xB1..0xB1 + 32].try_into().unwrap(),
            initial_channel_invalid: data[0xD1],
            emurator_type: match data[0xD2] - b'0' {
                0x00 => EmuratorType::Unknown,
                0x01 => EmuratorType::ZSNES,
                0x02 => EmuratorType::Snes9x,
                0x03 => EmuratorType::ZST2SPC,
                0x04 => EmuratorType::Other,
                0x05 => EmuratorType::SNEShout,
                0x06 => EmuratorType::ZSNESW,
                0x07 => EmuratorType::Snes9xpp,
                0x08 => EmuratorType::SNESGT,
                _ => {
                    return None;
                }
            },
        })
    }
}

/// SPCファイルのパース
pub fn parse_spc_file(data: &[u8]) -> Option<SPCFile> {
    // サイズチェック
    if data.len() < 66048 {
        return None;
    }

    if let Some(header) = parse_spc_header(data) {
        return Some(SPCFile {
            header: header,
            ram: data[0x100..0x100 + 65536].try_into().unwrap(),
            dsp_register: data[0x10100..0x10100 + 128].try_into().unwrap(),
            xram_buffer: data[0x101C0..0x101C0 + 64].try_into().unwrap(),
        });
    }

    None
}
