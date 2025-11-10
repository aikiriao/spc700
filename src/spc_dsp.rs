use crate::types::make_u16_from_u8;

/// DSPレジスタアドレス
const MVOLL_ADDRESS: u8 = 0x0C;
const MVOLR_ADDRESS: u8 = 0x1C;
const EVOLL_ADDRESS: u8 = 0x2C;
const EVOLR_ADDRESS: u8 = 0x3C;
const KON_ADDRESS: u8 = 0x4C;
const KOFF_ADDRESS: u8 = 0x5C;
const FLG_ADDRESS: u8 = 0x6C;
const ENDX_ADDRESS: u8 = 0x7C;
const EFB_ADDRESS: u8 = 0x0D;
const PMON_ADDRESS: u8 = 0x2D;
const NON_ADDRESS: u8 = 0x3D;
const EON_ADDRESS: u8 = 0x4D;
pub const DIR_ADDRESS: u8 = 0x5D;
const ESA_ADDRESS: u8 = 0x6D;
const EDL_ADDRESS: u8 = 0x7D;
const FIR0_ADDRESS: u8 = 0x0F;
const FIR1_ADDRESS: u8 = 0x1F;
const FIR2_ADDRESS: u8 = 0x2F;
const FIR3_ADDRESS: u8 = 0x3F;
const FIR4_ADDRESS: u8 = 0x4F;
const FIR5_ADDRESS: u8 = 0x5F;
const FIR6_ADDRESS: u8 = 0x6F;
const FIR7_ADDRESS: u8 = 0x7F;
const V0VOLL_ADDRESS: u8 = 0x00;
const V0VOLR_ADDRESS: u8 = 0x01;
const V0PITCHL_ADDRESS: u8 = 0x02;
const V0PITCHH_ADDRESS: u8 = 0x03;
const V0SRCN_ADDRESS: u8 = 0x04;
const V0ADSR1_ADDRESS: u8 = 0x05;
const V0ADSR2_ADDRESS: u8 = 0x06;
const V0GAIN_ADDRESS: u8 = 0x07;
const V0ENVX_ADDRESS: u8 = 0x08;
const V0OUTX_ADDRESS: u8 = 0x09;

/// ガウス補間テーブル
const GAUSS_INTERPOLATION_TABLE: [i32; 512] = [
    0x000, 0x000, 0x000, 0x000, 0x000, 0x000, 0x000, 0x000, 0x000, 0x000, 0x000, 0x000, 0x000,
    0x000, 0x000, 0x000, 0x001, 0x001, 0x001, 0x001, 0x001, 0x001, 0x001, 0x001, 0x001, 0x001,
    0x001, 0x002, 0x002, 0x002, 0x002, 0x002, 0x002, 0x002, 0x003, 0x003, 0x003, 0x003, 0x003,
    0x004, 0x004, 0x004, 0x004, 0x004, 0x005, 0x005, 0x005, 0x005, 0x006, 0x006, 0x006, 0x006,
    0x007, 0x007, 0x007, 0x008, 0x008, 0x008, 0x009, 0x009, 0x009, 0x00A, 0x00A, 0x00A, 0x00B,
    0x00B, 0x00B, 0x00C, 0x00C, 0x00D, 0x00D, 0x00E, 0x00E, 0x00F, 0x00F, 0x00F, 0x010, 0x010,
    0x011, 0x011, 0x012, 0x013, 0x013, 0x014, 0x014, 0x015, 0x015, 0x016, 0x017, 0x017, 0x018,
    0x018, 0x019, 0x01A, 0x01B, 0x01B, 0x01C, 0x01D, 0x01D, 0x01E, 0x01F, 0x020, 0x020, 0x021,
    0x022, 0x023, 0x024, 0x024, 0x025, 0x026, 0x027, 0x028, 0x029, 0x02A, 0x02B, 0x02C, 0x02D,
    0x02E, 0x02F, 0x030, 0x031, 0x032, 0x033, 0x034, 0x035, 0x036, 0x037, 0x038, 0x03A, 0x03B,
    0x03C, 0x03D, 0x03E, 0x040, 0x041, 0x042, 0x043, 0x045, 0x046, 0x047, 0x049, 0x04A, 0x04C,
    0x04D, 0x04E, 0x050, 0x051, 0x053, 0x054, 0x056, 0x057, 0x059, 0x05A, 0x05C, 0x05E, 0x05F,
    0x061, 0x063, 0x064, 0x066, 0x068, 0x06A, 0x06B, 0x06D, 0x06F, 0x071, 0x073, 0x075, 0x076,
    0x078, 0x07A, 0x07C, 0x07E, 0x080, 0x082, 0x084, 0x086, 0x089, 0x08B, 0x08D, 0x08F, 0x091,
    0x093, 0x096, 0x098, 0x09A, 0x09C, 0x09F, 0x0A1, 0x0A3, 0x0A6, 0x0A8, 0x0AB, 0x0AD, 0x0AF,
    0x0B2, 0x0B4, 0x0B7, 0x0BA, 0x0BC, 0x0BF, 0x0C1, 0x0C4, 0x0C7, 0x0C9, 0x0CC, 0x0CF, 0x0D2,
    0x0D4, 0x0D7, 0x0DA, 0x0DD, 0x0E0, 0x0E3, 0x0E6, 0x0E9, 0x0EC, 0x0EF, 0x0F2, 0x0F5, 0x0F8,
    0x0FB, 0x0FE, 0x101, 0x104, 0x107, 0x10B, 0x10E, 0x111, 0x114, 0x118, 0x11B, 0x11E, 0x122,
    0x125, 0x129, 0x12C, 0x130, 0x133, 0x137, 0x13A, 0x13E, 0x141, 0x145, 0x148, 0x14C, 0x150,
    0x153, 0x157, 0x15B, 0x15F, 0x162, 0x166, 0x16A, 0x16E, 0x172, 0x176, 0x17A, 0x17D, 0x181,
    0x185, 0x189, 0x18D, 0x191, 0x195, 0x19A, 0x19E, 0x1A2, 0x1A6, 0x1AA, 0x1AE, 0x1B2, 0x1B7,
    0x1BB, 0x1BF, 0x1C3, 0x1C8, 0x1CC, 0x1D0, 0x1D5, 0x1D9, 0x1DD, 0x1E2, 0x1E6, 0x1EB, 0x1EF,
    0x1F3, 0x1F8, 0x1FC, 0x201, 0x205, 0x20A, 0x20F, 0x213, 0x218, 0x21C, 0x221, 0x226, 0x22A,
    0x22F, 0x233, 0x238, 0x23D, 0x241, 0x246, 0x24B, 0x250, 0x254, 0x259, 0x25E, 0x263, 0x267,
    0x26C, 0x271, 0x276, 0x27B, 0x280, 0x284, 0x289, 0x28E, 0x293, 0x298, 0x29D, 0x2A2, 0x2A6,
    0x2AB, 0x2B0, 0x2B5, 0x2BA, 0x2BF, 0x2C4, 0x2C9, 0x2CE, 0x2D3, 0x2D8, 0x2DC, 0x2E1, 0x2E6,
    0x2EB, 0x2F0, 0x2F5, 0x2FA, 0x2FF, 0x304, 0x309, 0x30E, 0x313, 0x318, 0x31D, 0x322, 0x326,
    0x32B, 0x330, 0x335, 0x33A, 0x33F, 0x344, 0x349, 0x34E, 0x353, 0x357, 0x35C, 0x361, 0x366,
    0x36B, 0x370, 0x374, 0x379, 0x37E, 0x383, 0x388, 0x38C, 0x391, 0x396, 0x39B, 0x39F, 0x3A4,
    0x3A9, 0x3AD, 0x3B2, 0x3B7, 0x3BB, 0x3C0, 0x3C5, 0x3C9, 0x3CE, 0x3D2, 0x3D7, 0x3DC, 0x3E0,
    0x3E5, 0x3E9, 0x3ED, 0x3F2, 0x3F6, 0x3FB, 0x3FF, 0x403, 0x408, 0x40C, 0x410, 0x415, 0x419,
    0x41D, 0x421, 0x425, 0x42A, 0x42E, 0x432, 0x436, 0x43A, 0x43E, 0x442, 0x446, 0x44A, 0x44E,
    0x452, 0x455, 0x459, 0x45D, 0x461, 0x465, 0x468, 0x46C, 0x470, 0x473, 0x477, 0x47A, 0x47E,
    0x481, 0x485, 0x488, 0x48C, 0x48F, 0x492, 0x496, 0x499, 0x49C, 0x49F, 0x4A2, 0x4A6, 0x4A9,
    0x4AC, 0x4AF, 0x4B2, 0x4B5, 0x4B7, 0x4BA, 0x4BD, 0x4C0, 0x4C3, 0x4C5, 0x4C8, 0x4CB, 0x4CD,
    0x4D0, 0x4D2, 0x4D5, 0x4D7, 0x4D9, 0x4DC, 0x4DE, 0x4E0, 0x4E3, 0x4E5, 0x4E7, 0x4E9, 0x4EB,
    0x4ED, 0x4EF, 0x4F1, 0x4F3, 0x4F5, 0x4F6, 0x4F8, 0x4FA, 0x4FB, 0x4FD, 0x4FF, 0x500, 0x502,
    0x503, 0x504, 0x506, 0x507, 0x508, 0x50A, 0x50B, 0x50C, 0x50D, 0x50E, 0x50F, 0x510, 0x511,
    0x511, 0x512, 0x513, 0x514, 0x514, 0x515, 0x516, 0x516, 0x517, 0x517, 0x517, 0x518, 0x518,
    0x518, 0x518, 0x518, 0x519, 0x519,
];

#[derive(Copy, Clone, Debug)]
enum SPCVoiceGainMode {
    Fixed { gain: u8 },
    LinearDecrease { rate: u8 },
    ExponentialDecrease { rate: u8 },
    LinearIncrease { rate: u8 },
    BentIncrease { rate: u8 },
}

#[derive(Copy, Clone, Debug)]
enum SPCEnvelopeState {
    Attack,
    Decay,
    Sustain,
    Release,
}

#[derive(Copy, Clone, Debug)]
struct SPCDecoder {
    decode_buffer: [i16; 16],
    decode_history: [i32; 4],
    decode_buffer_pos: usize,
    sample_count: usize,
    decode_start_address: usize,
    decode_loop_address: usize,
    decode_read_pos: usize,
    pub loop_flag: bool,
    pub end: bool,
}

#[derive(Copy, Clone, Debug)]
struct SPCVoiceRegister {
    volume: [i8; 2],
    pitch: u16,
    sample_source: u8,
    adsr_enable: bool,
    attack_rate: u8,
    decay_rate: u8,
    sustain_rate: u8,
    sustain_level: u8,
    gain_mode: SPCVoiceGainMode,
    envelope_state: SPCEnvelopeState,
    envelope_value: u8,
    output_sample: i8,
    keyon: bool,
    keyoff: bool,
    pitch_mod: bool,
    noise: bool,
    decoder: SPCDecoder,
}

/// S-DSP
#[derive(Copy, Clone, Debug)]
pub struct SPCDSP {
    volume: [i8; 2],
    echo_volume: [i8; 2],
    flag: u8,
    echo_feedback: i8,
    echo: [bool; 8],
    brr_dir_page: u8,
    echo_start_page: u8,
    echo_delay: u8,
    fir_coef: [i8; 8],
    voice: [SPCVoiceRegister; 8],
}

impl SPCDecoder {
    fn new() -> Self {
        Self {
            decode_buffer: [0; 16],
            decode_history: [0; 4],
            decode_buffer_pos: 16,
            decode_start_address: 0,
            decode_loop_address: 0,
            decode_read_pos: 0,
            sample_count: 0,
            loop_flag: false,
            end: false,
        }
    }

    /// 1サンプルをテーブルを使用して補間
    fn interpolate_sample(decode_history: &[i32], sample_count: usize) -> i16 {
        let interp_index = (sample_count >> 4) & 0xFF;

        // 前のサンプルを使用し補間
        let mut output: i32 = 0;
        output += (GAUSS_INTERPOLATION_TABLE[0x0FF - interp_index] * decode_history[0]) >> 10;
        output += (GAUSS_INTERPOLATION_TABLE[0x1FF - interp_index] * decode_history[1]) >> 10;
        output += (GAUSS_INTERPOLATION_TABLE[0x100 + interp_index] * decode_history[2]) >> 10;
        output += (GAUSS_INTERPOLATION_TABLE[0x000 + interp_index] * decode_history[3]) >> 10;
        output >>= 1;

        output as i16
    }

    /// 1サンプルデコード
    fn decode_sample(&mut self, filter: u8, granularity: u8, pitch: u16, nibble: u8) -> i16 {
        assert!(nibble <= 0xF);

        // 符号付き4bit値の読み取り
        let sample = if nibble >= 8 {
            (nibble as i32) | !0xFi32
        } else {
            nibble as i32
        };

        // デコード処理
        let mut output = sample << (granularity as i32);
        let p1 = self.decode_history[3];
        let p2 = self.decode_history[2];
        match filter {
            0 => {}
            1 => {
                // output + (15 / 16) * p1
                output += p1;
                output += (-p1) >> 4;
            }
            2 => {
                // output + (61 / 32) * p1 - (15 / 16) * p2
                output += p1 << 1;
                output += (-(p1 << 1) + p1) >> 5;
                output -= p2;
                output += p2 >> 4;
            }
            3 => {
                // output + (115 / 64) * p1 - (13 / 16) * p2
                output += p1 << 1;
                output += (-(p1 + (p1 << 2) + (p1 << 3))) >> 6;
                output -= p2;
                output += ((p2 << 1) + p2) >> 4;
            }
            _ => panic!("Invalid BRR filter!"),
        }

        // 出力を15bit幅にクリップ（[-3FFA, 3FF8]）
        output = output.clamp(-16378, 16376);

        // デコード履歴更新
        self.decode_history[0] = self.decode_history[1];
        self.decode_history[1] = self.decode_history[2];
        self.decode_history[2] = self.decode_history[3];
        self.decode_history[3] = output;

        // ガウス補間
        let out = Self::interpolate_sample(&self.decode_history, self.sample_count);

        // サンプルインデックス更新
        self.sample_count = self.sample_count.wrapping_add(pitch as usize);

        out
    }

    /// 1ブロックデコード
    fn decode_block(&mut self, ram: &[u8], pitch: u16) {
        assert!(ram.len() >= 9);

        // RFレジスタの復号
        let rfreg = ram[0];
        let granularity = rfreg >> 4;
        let filter = (rfreg >> 2) & 0x3;

        // 16サンプル復号
        for i in 0..8 {
            let byte = ram[i + 1];
            self.decode_buffer[2 * i + 0] =
                self.decode_sample(filter, granularity, pitch, (byte >> 4) & 0xF);
            self.decode_buffer[2 * i + 1] =
                self.decode_sample(filter, granularity, pitch, (byte >> 0) & 0xF);
        }

        // フラグ更新
        self.loop_flag = ((rfreg >> 1) & 0x1) != 0;
        self.end = ((rfreg >> 0) & 0x1) != 0;
    }

    /// 1サンプルデコード
    fn process(&mut self, ram: &[u8], pitch: u16) -> i16 {
        // バッファが尽きたら次のブロックをデコード
        if self.decode_buffer_pos >= 16 {
            // 1ブロックデコード
            self.decode_block(&ram[self.decode_read_pos..], pitch);
            self.decode_buffer_pos = 0;
            // デコードアドレスの更新
            self.decode_read_pos = if self.end {
                // ループ開始アドレスに戻る
                self.decode_loop_address
            } else {
                // 次のブロックに進む
                self.decode_read_pos + 9
            };
        }

        // バッファからデータを取り出し
        let out = self.decode_buffer[self.decode_buffer_pos];
        self.decode_buffer_pos += 1;

        out
    }
}

impl SPCVoiceRegister {
    fn new() -> Self {
        Self {
            volume: [0; 2],
            pitch: 0,
            sample_source: 0,
            adsr_enable: false,
            attack_rate: 0,
            decay_rate: 0,
            sustain_rate: 0,
            sustain_level: 0,
            gain_mode: SPCVoiceGainMode::Fixed { gain: 0 },
            envelope_value: 0,
            output_sample: 0,
            keyon: false,
            keyoff: false,
            pitch_mod: false,
            noise: false,
            envelope_state: SPCEnvelopeState::Release,
            decoder: SPCDecoder::new(),
        }
    }

    fn compute_sample(&mut self, ram: &[u8]) -> [i16; 2] {
        // デコード
        let mut out = self.decoder.process(ram, self.pitch);
        // 最後の出力サンプル更新
        self.output_sample = ((out >> 8) & 0xFF) as i8;
        // ENDフラグがセットかつループフラグが立っていなければ即時ミュート
        if self.decoder.end {
            if !self.decoder.loop_flag {
                self.envelope_state = SPCEnvelopeState::Release;
                self.envelope_value = 0;
                return [0, 0];
            }
        }

        // TODO: PMON
        // TODO: NON
        // TODO: ADSR
        // 左右ボリューム適用
        let lout = ((out as i32) * (self.volume[0] as i32)) >> 7;
        let rout = ((out as i32) * (self.volume[1] as i32)) >> 7;
        [lout as i16, rout as i16]
    }
}

impl SPCDSP {
    pub fn new() -> SPCDSP {
        Self {
            volume: [0; 2],
            echo_volume: [0; 2],
            flag: 0,
            echo_feedback: 0,
            echo: [false; 8],
            brr_dir_page: 0,
            echo_start_page: 0,
            echo_delay: 0,
            fir_coef: [0; 8],
            voice: [SPCVoiceRegister::new(); 8],
        }
    }

    /// DSPレジスタの書き込み処理
    pub fn write_dsp_register(&mut self, ram: &[u8], address: u8, value: u8) {
        match address {
            MVOLL_ADDRESS => {
                self.volume[0] = value as i8;
            }
            MVOLR_ADDRESS => {
                self.volume[1] = value as i8;
            }
            EVOLL_ADDRESS => {
                self.echo_volume[0] = value as i8;
            }
            EVOLR_ADDRESS => {
                self.echo_volume[1] = value as i8;
            }
            KON_ADDRESS => {
                for ch in 0..8 {
                    let keyon = ((value >> ch) & 0x1) != 0;
                    self.voice[ch].keyon = keyon;
                    // キーオンが入ったとき
                    if keyon {
                        let dir_address = ((self.brr_dir_page as usize) << 8)
                            + 4 * (self.voice[ch].sample_source as usize);
                        self.voice[ch].envelope_state = SPCEnvelopeState::Attack;
                        self.voice[ch].envelope_value = 0;
                        self.voice[ch].decoder.end = false;
                        self.voice[ch].decoder.decode_start_address =
                            make_u16_from_u8(&ram[dir_address..(dir_address + 2)]) as usize;
                        self.voice[ch].decoder.decode_loop_address =
                            make_u16_from_u8(&ram[(dir_address + 2)..(dir_address + 4)]) as usize;
                        self.voice[ch].decoder.decode_read_pos =
                            self.voice[ch].decoder.decode_start_address;
                    }
                }
            }
            KOFF_ADDRESS => {
                for ch in 0..8 {
                    let keyoff = ((value >> ch) & 0x1) != 0;
                    self.voice[ch].keyoff = keyoff;
                    // キーオフが入ったとき
                    if keyoff {
                        self.voice[ch].envelope_state = SPCEnvelopeState::Release;
                    }
                }
            }
            FLG_ADDRESS => {
                self.flag = value;
            }
            ENDX_ADDRESS => {
                for ch in 0..8 {
                    self.voice[ch].decoder.end = ((value >> ch) & 0x1) != 0;
                }
            }
            EFB_ADDRESS => {
                self.echo_feedback = value as i8;
            }
            PMON_ADDRESS => {
                for ch in 1..8 {
                    /* NOTE! 0は無効 */
                    self.voice[ch].pitch_mod = ((value >> ch) & 0x1) != 0;
                }
            }
            NON_ADDRESS => {
                for ch in 0..8 {
                    self.voice[ch].noise = ((value >> ch) & 0x1) != 0;
                }
            }
            EON_ADDRESS => {
                for ch in 0..8 {
                    self.echo[ch] = ((value >> ch) & 0x1) != 0;
                }
            }
            DIR_ADDRESS => {
                self.brr_dir_page = value;
            }
            ESA_ADDRESS => {
                self.echo_start_page = value;
            }
            EDL_ADDRESS => {
                self.echo_delay = value & 0x0F;
            }
            FIR0_ADDRESS | FIR1_ADDRESS | FIR2_ADDRESS | FIR3_ADDRESS | FIR4_ADDRESS
            | FIR5_ADDRESS | FIR6_ADDRESS | FIR7_ADDRESS => {
                let index = address >> 4;
                self.fir_coef[index as usize] = value as i8;
            }
            address if ((address & 0xF) <= 0x9) => {
                let ch = (address >> 4) as usize;
                match address & 0xF {
                    V0VOLL_ADDRESS => {
                        self.voice[ch].volume[0] = value as i8;
                    }
                    V0VOLR_ADDRESS => {
                        self.voice[ch].volume[1] = value as i8;
                    }
                    V0PITCHL_ADDRESS => {
                        self.voice[ch].pitch = (self.voice[ch].pitch & 0xFF00) | (value as u16);
                    }
                    V0PITCHH_ADDRESS => {
                        self.voice[ch].pitch =
                            ((value as u16) << 8) | (self.voice[ch].pitch & 0x00FF);
                    }
                    V0SRCN_ADDRESS => {
                        self.voice[ch].sample_source = value;
                    }
                    V0ADSR1_ADDRESS => {
                        self.voice[ch].adsr_enable = (value >> 7) != 0;
                        self.voice[ch].attack_rate = value & 0xF;
                        self.voice[ch].decay_rate = (value >> 4) & 0x7;
                    }
                    V0ADSR2_ADDRESS => {
                        self.voice[ch].sustain_rate = value & 0x1F;
                        self.voice[ch].sustain_level = (value >> 5) & 0x7;
                    }
                    V0GAIN_ADDRESS => {
                        if (value >> 7) == 0 {
                            self.voice[ch].gain_mode =
                                SPCVoiceGainMode::Fixed { gain: value & 0x7F };
                        } else {
                            let rate = value & 0x1F;
                            self.voice[ch].gain_mode = match (value >> 5) & 0x3 {
                                0 => SPCVoiceGainMode::LinearDecrease { rate: rate },
                                1 => SPCVoiceGainMode::ExponentialDecrease { rate: rate },
                                2 => SPCVoiceGainMode::LinearIncrease { rate: rate },
                                3 => SPCVoiceGainMode::BentIncrease { rate: rate },
                                _ => panic!("Unsupported Gain Type!"),
                            };
                        }
                    }
                    V0ENVX_ADDRESS => {
                        // 書き込めるけど意味はない（読み取り用レジスタ）
                        self.voice[ch].envelope_value = value;
                    }
                    V0OUTX_ADDRESS => {
                        // 書き込めるけど意味はない（読み取り用レジスタ）
                        self.voice[ch].output_sample = value as i8;
                    }
                    _ => {
                        // 他のアドレスへの書き込みは効果なし
                    }
                }
            }
            _ => {
                // 他のアドレスへの書き込みは効果なし
            }
        }
    }

    /// DSPレジスタの読み込み処理
    pub fn read_dsp_register(&self, _ram: &[u8], address: u8) -> u8 {
        match address {
        // 80-FFの読み込みは00-7Fと同等に扱われる
        match address & 0x7F {
            MVOLL_ADDRESS => self.volume[0] as u8,
            MVOLR_ADDRESS => self.volume[1] as u8,
            EVOLL_ADDRESS => self.echo_volume[0] as u8,
            EVOLR_ADDRESS => self.echo_volume[1] as u8,
            KON_ADDRESS => {
                let mut ret = 0;
                let mut bit = 1;
                for ch in 0..8 {
                    if self.voice[ch].keyon {
                        ret |= bit;
                    }
                    bit <<= 1;
                }
                ret
            }
            KOFF_ADDRESS => {
                let mut ret = 0;
                let mut bit = 1;
                for ch in 0..8 {
                    if self.voice[ch].keyoff {
                        ret |= bit;
                    }
                    bit <<= 1;
                }
                ret
            }
            FLG_ADDRESS => self.flag,
            ENDX_ADDRESS => {
                let mut ret = 0;
                let mut bit = 1;
                for ch in 0..8 {
                    if self.voice[ch].decoder.end {
                        ret |= bit;
                    }
                    bit <<= 1;
                }
                ret
            }
            EFB_ADDRESS => self.echo_feedback as u8,
            PMON_ADDRESS => {
                let mut ret = 0;
                let mut bit = 1;
                for ch in 1..8 {
                    /* NOTE! ch==0は常に無効 */
                    if self.voice[ch].pitch_mod {
                        ret |= bit;
                    }
                    bit <<= 1;
                }
                ret
            }
            NON_ADDRESS => {
                let mut ret = 0;
                let mut bit = 1;
                for ch in 0..8 {
                    if self.voice[ch].noise {
                        ret |= bit;
                    }
                    bit <<= 1;
                }
                ret
            }
            EON_ADDRESS => {
                let mut ret = 0;
                let mut bit = 1;
                for ch in 0..8 {
                    if self.echo[ch] {
                        ret |= bit;
                    }
                    bit <<= 1;
                }
                ret
            }
            DIR_ADDRESS => self.brr_dir_page,
            ESA_ADDRESS => self.echo_start_page,
            EDL_ADDRESS => self.echo_delay,
            FIR0_ADDRESS | FIR1_ADDRESS | FIR2_ADDRESS | FIR3_ADDRESS | FIR4_ADDRESS
            | FIR5_ADDRESS | FIR6_ADDRESS | FIR7_ADDRESS => {
                let index = address >> 4;
                self.fir_coef[index as usize] as u8
            }
            address if ((address & 0xF) <= 0x9) => {
                let ch = (address >> 4) as usize;
                match address & 0xF {
                    V0VOLL_ADDRESS => self.voice[ch].volume[0] as u8,
                    V0VOLR_ADDRESS => self.voice[ch].volume[1] as u8,
                    V0PITCHL_ADDRESS => (self.voice[ch].pitch & 0xFF) as u8,
                    V0PITCHH_ADDRESS => ((self.voice[ch].pitch >> 8) & 0xFF) as u8,
                    V0SRCN_ADDRESS => self.voice[ch].sample_source,
                    V0ADSR1_ADDRESS => {
                        let adsr_flag = if self.voice[ch].adsr_enable {
                            0x80
                        } else {
                            0x00
                        };
                        adsr_flag | (self.voice[ch].decay_rate << 4) | self.voice[ch].attack_rate
                    }
                    V0ADSR2_ADDRESS => {
                        (self.voice[ch].sustain_level << 5) | self.voice[ch].sustain_rate
                    }
                    V0GAIN_ADDRESS => match self.voice[ch].gain_mode {
                        SPCVoiceGainMode::Fixed { gain } => gain & 0x7F,
                        SPCVoiceGainMode::LinearDecrease { rate } => {
                            0x80 | (0 << 5) | (rate & 0x1F)
                        }
                        SPCVoiceGainMode::ExponentialDecrease { rate } => {
                            0x80 | (1 << 5) | (rate & 0x1F)
                        }
                        SPCVoiceGainMode::LinearIncrease { rate } => {
                            0x80 | (2 << 5) | (rate & 0x1F)
                        }
                        SPCVoiceGainMode::BentIncrease { rate } => 0x80 | (3 << 5) | (rate & 0x1F),
                    },
                    V0ENVX_ADDRESS => self.voice[ch].envelope_value,
                    V0OUTX_ADDRESS => self.voice[ch].output_sample as u8,
                    _ => {
                        panic!("Unsupported DSP address!");
                    }
                }
            }
            _ => {
                panic!("Unsupported DSP address!");
            }
        }
    }

    /// ステレオサンプル計算処理
    pub fn compute_sample(&mut self, ram: &[u8]) -> [i16; 2] {
        let mut out = [0i32; 2];
        // 全チャンネルの出力をミックス
        for ch in 0..8 {
            let vout = self.voice[ch].compute_sample(ram);
            out[0] += vout[0] as i32;
            out[1] += vout[1] as i32;
        }
        // マスターボリューム適用
        for ch in 0..2 {
            out[ch] = (out[ch] * (self.volume[ch] as i32)) >> 7;
        }
        [out[0] as i16, out[1] as i16]
    }
}
