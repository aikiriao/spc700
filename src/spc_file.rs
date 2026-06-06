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

/// Extended ID666 フォーマット
#[derive(Debug, Clone)]
pub struct ExtendedID666Format {
    /// 曲名
    pub song_name: Option<[u8; 256]>,
    /// ゲーム名
    pub game_name: Option<[u8; 256]>,
    /// アーティスト名
    pub artist_name: Option<[u8; 256]>,
    /// ダンパー名
    pub dumper_name: Option<[u8; 256]>,
    /// ダンプした日yyyymmdd
    pub dump_date: Option<u32>,
    /// エミュレータ種別
    pub emulator_used: Option<u8>,
    /// コメント
    pub comments: Option<[u8; 256]>,
    /// オフィシャルサウンドトラック(OST)タイトル
    pub ost_title: Option<[u8; 256]>,
    /// OSTディスク
    pub ost_disc: Option<u8>,
    /// OSTトラック
    pub ost_track: Option<u16>,
    /// パブリッシャー名
    pub publisher_name: Option<[u8; 256]>,
    /// コピーライト年
    pub copyright_year: Option<u16>,
    /// イントロの長さ（1/64000秒単位）
    pub introduction_length: Option<u32>,
    /// ループの長さ（1/64000秒単位）
    pub loop_length: Option<u32>,
    /// ループの長さ（1/64000秒単位）負数のときがある
    pub end_length: Option<i32>,
    /// ループの長さ（1/64000秒単位）
    pub fade_length: Option<u32>,
    /// ミュートチャンネル（各チャンネルのミュートフラグ）
    pub mute_channels: Option<u8>,
    /// ループを何回繰り返すか
    pub number_of_loops: Option<u8>,
    /// 出力に適用するゲイン（65536で通常再生）
    pub amplification_value: Option<u32>,
}

/// SPCファイル
#[derive(Debug, Clone)]
pub struct SPCFile {
    /// SPCファイルヘッダ
    pub header: SPCFileHeader,
    /// 拡張ID666
    pub extended_id666: Option<ExtendedID666Format>,
    /// 64KB RAM
    pub ram: [u8; 65536],
    /// DSPレジスタ
    pub dsp_register: [u8; 128],
    /// XRAMバッファ
    pub xram_buffer: [u8; 64],
}

/// メモリ上にあるデータから32bitデータを読みだす
fn make_u32_from_u8(data: &[u8]) -> u32 {
    assert_eq!(data.len(), 4);
    ((data[3] as u32) << 24)
        | ((data[2] as u32) << 16)
        | ((data[1] as u32) << 8)
        | ((data[0] as u32) << 0)
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

// ヘッダがバイナリフォーマットか否（テキスト）か判定
fn determine_data_format_is_binary(data: &[u8]) -> bool {
    // 日付情報が存在するか
    let date_exist = {
        let mut flag = false;
        for i in 0..11 {
            if data[0x9E + i] != 0 {
                flag = true;
                break;
            }
        }
        flag
    };

    if date_exist {
        // 日付を区切るスラッシュがあればテキスト
        if data[0x9E + 2] == b'/' && data[0x9E + 5] == b'/' {
            return false;
        }
        // 日付が文字としてパースできればテキスト
        if let Some(_) = u8array_to_numeric(&data[0x9E..0x9E + 2]) {
            return false;
        }
        if let Some(_) = u8array_to_numeric(&data[0x9E + 3..0x9E + 5]) {
            return false;
        }
        if let Some(_) = u8array_to_numeric(&data[0x9E + 6..0x9E + 11]) {
            return false;
        }
    }

    // 演奏時間が存在するか
    let duration_exist = {
        let mut flag = false;
        for i in 0..3 {
            if data[0xA9 + i] != 0 {
                flag = true;
                break;
            }
        }
        flag
    };

    if duration_exist {
        // 演奏時間が文字としてパースできればテキスト
        if let Some(_) = u8array_to_numeric(&data[0xA9..0xA9 + 3]) {
            return false;
        }
    }

    // フェードアウト時間が存在するか
    let fadeout_exist = {
        let mut flag = false;
        for i in 0..5 {
            if data[0xAC + i] != 0 {
                flag = true;
                break;
            }
        }
        flag
    };

    if fadeout_exist {
        // フェードアウト時間が文字としてパースできればテキスト
        if let Some(_) = u8array_to_numeric(&data[0xAC..0xAC + 5]) {
            return false;
        }
    }

    // 判定不可能な場合はバイナリとする
    true
}

/// SPCファイルヘッダのパース
pub fn parse_spc_header(data: &[u8]) -> Option<SPCFileHeader> {
    // サイズチェック
    if data.len() < 256 {
        return None;
    }

    // バイナリ/テキストフォーマット判定してからパース
    if determine_data_format_is_binary(data) {
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
            fadeout_time: ((data[0xAC] as u32) << 0)
                | ((data[0xAD] as u32) << 8)
                | ((data[0xAE] as u32) << 16),
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

// 最大長さ256の文字列を生成
fn make_fixedstring_from_slice(data: &[u8]) -> Option<[u8; 256]> {
    if data.len() > 256 {
        return None;
    }

    let mut buf = [0u8; 256];
    buf[..data.len()].copy_from_slice(data);

    Some(buf)
}

/// Extended ID666 Formatのパース
fn parse_xid6_format(data: &[u8]) -> Option<ExtendedID666Format> {
    // サイズチェック
    if data.len() <= 8 {
        return None;
    }

    // チャンクタイプチェック
    if data[0] != b'x' || data[1] != b'i' || data[2] != b'd' || data[3] != b'6' {
        return None;
    }

    // 初期化
    let mut xid6 = ExtendedID666Format {
        song_name: None,
        game_name: None,
        artist_name: None,
        dumper_name: None,
        dump_date: None,
        emulator_used: None,
        comments: None,
        ost_title: None,
        ost_disc: None,
        ost_track: None,
        publisher_name: None,
        copyright_year: None,
        introduction_length: None,
        loop_length: None,
        end_length: None,
        fade_length: None,
        mute_channels: None,
        number_of_loops: None,
        amplification_value: None,
    };

    // チャンクの大きさ
    let chunk_size = make_u32_from_u8(&data[4..8]) as usize;
    let chunk_data = &data[8..];
    let mut read_pos = 0usize;
    while read_pos < chunk_size {
        let chunk_id = chunk_data[read_pos + 0];
        let chunk_type = chunk_data[read_pos + 1];
        let data_length = if chunk_type != 0 {
            make_u16_from_u8(&chunk_data[read_pos + 2..read_pos + 4]) as usize
        } else {
            0
        };
        read_pos += 4;
        match chunk_id {
            0x01 => {
                xid6.song_name =
                    make_fixedstring_from_slice(&chunk_data[read_pos..read_pos + data_length]);
            }
            0x02 => {
                xid6.game_name =
                    make_fixedstring_from_slice(&chunk_data[read_pos..read_pos + data_length]);
            }
            0x03 => {
                xid6.artist_name =
                    make_fixedstring_from_slice(&chunk_data[read_pos..read_pos + data_length]);
            }
            0x04 => {
                xid6.dumper_name =
                    make_fixedstring_from_slice(&chunk_data[read_pos..read_pos + data_length]);
            }
            0x05 => {
                xid6.dump_date = Some(make_u32_from_u8(&chunk_data[read_pos..read_pos + 4]));
            }
            0x06 => {
                // ヘッダ内のデータのため前方参照
                xid6.emulator_used = Some(chunk_data[read_pos - 2]);
            }
            0x07 => {
                xid6.comments =
                    make_fixedstring_from_slice(&chunk_data[read_pos..read_pos + data_length]);
            }
            0x10 => {
                xid6.ost_title =
                    make_fixedstring_from_slice(&chunk_data[read_pos..read_pos + data_length]);
            }
            0x11 => {
                xid6.ost_disc = Some(chunk_data[read_pos]);
            }
            0x12 => {
                xid6.ost_track = Some(make_u16_from_u8(&chunk_data[read_pos..read_pos + 2]));
            }
            0x13 => {
                xid6.publisher_name =
                    make_fixedstring_from_slice(&chunk_data[read_pos..read_pos + data_length]);
            }
            0x14 => {
                // ヘッダ内のデータのため前方参照
                xid6.copyright_year = Some(make_u16_from_u8(&chunk_data[read_pos - 2..read_pos]));
            }
            0x30 => {
                xid6.introduction_length =
                    Some(make_u32_from_u8(&chunk_data[read_pos..read_pos + 4]));
            }
            0x31 => {
                xid6.loop_length = Some(make_u32_from_u8(&chunk_data[read_pos..read_pos + 4]));
            }
            0x32 => {
                xid6.end_length =
                    Some(make_u32_from_u8(&chunk_data[read_pos..read_pos + 4]) as i32);
            }
            0x33 => {
                xid6.fade_length = Some(make_u32_from_u8(&chunk_data[read_pos..read_pos + 4]));
            }
            0x34 => {
                // ヘッダ内のデータのため前方参照
                xid6.mute_channels = Some(chunk_data[read_pos - 2]);
            }
            0x35 => {
                // ヘッダ内のデータのため前方参照
                xid6.number_of_loops = Some(chunk_data[read_pos - 2]);
            }
            0x36 => {
                xid6.amplification_value =
                    Some(make_u32_from_u8(&chunk_data[read_pos..read_pos + 4]));
            }
            _ => {
                return None;
            }
        }
        // 32bit境界に合わせて移動
        read_pos += ((data_length as usize + 3) / 4) * 4;
    }

    Some(xid6)
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
            extended_id666: parse_xid6_format(&data[0x10200..]),
        });
    }

    None
}
