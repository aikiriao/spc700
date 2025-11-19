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
const DIR_ADDRESS: u8 = 0x5D;
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

/// グローバルカウンタイベントが発生するまでのサンプル数
const COUNTER_RATES: [u16; 32] = [
    0, /* Inf */
    2048, 1536, 1280, 1024, 768, 640, 512, 384, 320, 256, 192, 160, 128, 96, 80, 64, 48, 40, 32,
    24, 20, 16, 12, 10, 8, 6, 5, 4, 3, 2, 1,
];

/// グローバルカウンタのオフセット
const COUNTER_OFFSETS: [u16; 32] = [
    0, /* N/A */
    0, 1040, 536, 0, 1040, 536, 0, 1040, 536, 0, 1040, 536, 0, 1040, 536, 0, 1040, 536, 0, 1040,
    536, 0, 1040, 536, 0, 1040, 536, 0, 1040, 0, 0,
];

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
    decode_buffer: [i16; 20],
    decode_history: [i32; 2],
    sample_index_fixed: u16,
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
    brr_dir_address_base: usize,
    sample_source: u8,
    adsr_enable: bool,
    attack_rate: u8,
    decay_rate: u8,
    sustain_rate: u8,
    sustain_level: u8,
    gain_mode: SPCVoiceGainMode,
    gain_value: u8,
    envelope_state: SPCEnvelopeState,
    envelope_gain: i32,
    envelope_rate: u8,
    output_sample: i16,
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
    mute: bool,
    noise_clock: u8,
    echo_feedback: i8,
    echo_buffer_write_enable: bool,
    echo: [bool; 8],
    brr_dir_page: u8,
    echo_buffer_address: usize,
    echo_buffer_size: usize,
    echo_buffer_pos: usize,
    fir_coef: [i8; 8],
    fir_buffer: [[i16; 8]; 2],
    fir_buffer_pos: usize,
    global_counter: u16,
    voice: [SPCVoiceRegister; 8],
}

impl SPCDecoder {
    fn new() -> Self {
        Self {
            decode_buffer: [0; 20],
            decode_history: [0; 2],
            decode_start_address: 0,
            decode_loop_address: 0,
            decode_read_pos: 0,
            sample_index_fixed: 0,
            loop_flag: false,
            end: false,
        }
    }

    /// 1サンプルをテーブルを使用して補間
    fn interpolate_sample(decode_buffer: &[i16], interp_index: usize) -> i16 {
        // 前のサンプルを使用し補間
        let mut output: i32 = 0;
        output += (GAUSS_INTERPOLATION_TABLE[0x0FF - interp_index] * decode_buffer[0] as i32) >> 10;
        output += (GAUSS_INTERPOLATION_TABLE[0x1FF - interp_index] * decode_buffer[1] as i32) >> 10;
        output += (GAUSS_INTERPOLATION_TABLE[0x100 + interp_index] * decode_buffer[2] as i32) >> 10;
        output += (GAUSS_INTERPOLATION_TABLE[0x000 + interp_index] * decode_buffer[3] as i32) >> 10;
        output >>= 1;

        output as i16
    }

    /// 1サンプルデコード
    fn decode_brr_sample(history: &mut [i32], filter: u8, granularity: u8, nibble: u8) -> i16 {
        assert!(nibble <= 0xF);

        // 符号付き4bit値の読み取り
        let mut sample = if nibble >= 8 {
            (nibble as i32) | !0xFi32
        } else {
            nibble as i32
        };

        let scale = if granularity <= 12 {
            granularity
        } else {
            sample >>= 3;
            12
        };

        // デコード処理
        let mut output = (sample << (scale as i32)) >> 1;
        let p1 = history[1];
        let p2 = history[0];
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
                output += (-3 * p1) >> 5;
                output -= p2;
                output += p2 >> 4;
            }
            3 => {
                // output + (115 / 64) * p1 - (13 / 16) * p2
                output += p1 << 1;
                output += (-13 * p1) >> 6;
                output -= p2;
                output += (3 * p2) >> 4;
            }
            _ => panic!("Invalid BRR filter!"),
        }

        // 出力を15bit幅にクリップ（[-3FFA, 3FF8]）
        output = output.clamp(-16378, 16376);

        // デコード履歴更新
        history[0] = history[1];
        history[1] = output;

        output as i16
    }

    /// BRRブロックヘッダ（RFレジスタ）のデコード
    fn decode_brr_block_header(rfreg: u8) -> (u8, u8, bool, bool) {
        let granularity = rfreg >> 4;
        let filter = (rfreg >> 2) & 0x3;
        let loop_flag = (rfreg & 0x2) != 0;
        let end_flag = (rfreg & 0x1) != 0;
        (granularity, filter, loop_flag, end_flag)
    }

    /// 1ブロックデコード
    fn decode_brr_block_signal(
        history: &mut [i32],
        granularity: u8,
        filter: u8,
        ram: &[u8],
        out: &mut [i16],
    ) {
        assert!(ram.len() >= 8);
        // 16サンプル復号
        for i in 0..8 {
            let byte = ram[i];
            out[2 * i + 0] =
                Self::decode_brr_sample(history, filter, granularity, (byte >> 4) & 0xF);
            out[2 * i + 1] =
                Self::decode_brr_sample(history, filter, granularity, (byte >> 0) & 0xF);
        }
    }

    /// 1ブロックデコード
    fn decode_brr_block(&mut self, ram: &[u8]) {
        let granularity;
        let filter;
        assert!(ram.len() >= 9);

        // ブロックヘッダデコード
        (granularity, filter, self.loop_flag, self.end) = Self::decode_brr_block_header(ram[0]);

        // 末尾4サンプルを先頭に移動（補間のため）
        for i in 0..4 {
            self.decode_buffer[i] = self.decode_buffer[16 + i];
        }

        // 1ブロックデコード
        Self::decode_brr_block_signal(
            &mut self.decode_history,
            granularity,
            filter,
            &ram[1..],
            &mut self.decode_buffer[4..],
        );
    }

    /// 1サンプル出力
    fn process(&mut self, ram: &[u8], pitch: u16) -> i16 {
        let next_block;

        // サンプルを進める
        (self.sample_index_fixed, next_block) = self.sample_index_fixed.overflowing_add(pitch);

        // バッファが尽きたら次のブロックをデコード
        if next_block {
            // 1ブロックデコード
            self.decode_brr_block(&ram[self.decode_read_pos..]);
            if self.end {
                // 末尾に達していたらループ開始アドレスに戻る
                self.decode_read_pos = self.decode_loop_address;
            } else {
                // 次のブロックに進む
                self.decode_read_pos += 9;
            }
        }

        // 補間して出力
        let index = (self.sample_index_fixed >> 12) as usize;
        Self::interpolate_sample(
            &self.decode_buffer[index..(index + 4)],
            ((self.sample_index_fixed >> 4) & 0xFF) as usize,
        )
    }
}

impl SPCVoiceRegister {
    fn new() -> Self {
        Self {
            volume: [0; 2],
            pitch: 0,
            brr_dir_address_base: 0,
            sample_source: 0,
            adsr_enable: false,
            attack_rate: 0,
            decay_rate: 0,
            sustain_rate: 0,
            sustain_level: 0,
            gain_mode: SPCVoiceGainMode::Fixed { gain: 0 },
            gain_value: 0,
            envelope_gain: 0,
            output_sample: 0,
            keyon: false,
            keyoff: false,
            pitch_mod: false,
            noise: false,
            envelope_state: SPCEnvelopeState::Release,
            envelope_rate: 0,
            decoder: SPCDecoder::new(),
        }
    }

    /// 1ステレオサンプル計算
    fn compute_sample(&mut self, ram: &[u8], global_counter: u16, prev_voice_out: i16) -> [i16; 2] {
        // キーオンが入ったとき
        if self.keyon {
            self.keyon = false;
            // エンベロープ設定
            self.envelope_state = SPCEnvelopeState::Attack;
            if self.adsr_enable {
                self.envelope_gain = 0;
                self.envelope_rate = self.attack_rate;
            } else {
                match self.gain_mode {
                    SPCVoiceGainMode::Fixed { gain } => {
                        self.envelope_gain = (gain as i32) << 4;
                        self.envelope_rate = 0;
                    }
                    SPCVoiceGainMode::LinearDecrease { rate } |
                    SPCVoiceGainMode::ExponentialDecrease { rate } | 
                    SPCVoiceGainMode::LinearIncrease { rate } |
                    SPCVoiceGainMode::BentIncrease { rate } => {
                        self.envelope_rate = rate;
                    }
                }
            }
            // デコーダのアドレス設定
            self.decoder.end = false;
            let dir_address = self.brr_dir_address_base + 4 * (self.sample_source as usize);
            self.decoder.decode_start_address =
                make_u16_from_u8(&ram[dir_address..(dir_address + 2)]) as usize;
            self.decoder.decode_loop_address =
                make_u16_from_u8(&ram[(dir_address + 2)..(dir_address + 4)]) as usize;
            self.decoder.decode_read_pos = self.decoder.decode_start_address;
            self.decoder.sample_index_fixed = 0;
            self.decoder.decode_buffer.fill(0);
            self.decoder.decode_history.fill(0);
        }

        // キーオフが入ったとき
        if self.keyoff {
            // フラグクリア（レジスタ設定時にReleaseには移行済み）
            self.keyoff = false;
        }

        // ピッチ（+モジュレーション）
        let mut pitch = self.pitch as i32;
        if self.pitch_mod && !self.noise {
            let factor = (prev_voice_out >> 4) as i32 + 0x400;
            pitch = (factor * pitch) >> 10;
        };

        // デコード
        let mut out = self.decoder.process(ram, pitch as u16);

        // ENDフラグがセットかつループフラグが立っていなければ即時ミュート
        if self.decoder.end {
            if !self.decoder.loop_flag {
                self.envelope_state = SPCEnvelopeState::Release;
                self.envelope_gain = 0;
            }
        }

        // デコード後の出力サンプル更新
        self.output_sample = out;

        // TODO: NON

        // アクション発生判定
        if (self.envelope_rate > 0)
            && ((global_counter + COUNTER_OFFSETS[self.envelope_rate as usize])
                % COUNTER_RATES[self.envelope_rate as usize]
                == 0)
        {
            // エンベロープゲイン更新
            if self.adsr_enable {
                match self.envelope_state {
                    SPCEnvelopeState::Attack => {
                        if self.attack_rate == 31 {
                            self.envelope_gain += 1024;
                        } else {
                            // rate = aaaa1のLinear increaseと同じ
                            self.envelope_gain += 32;
                        }
                    }
                    SPCEnvelopeState::Decay => {
                        // rate = 1ddd0のExp. decreaseと同じ
                        self.envelope_gain -= 1;
                        self.envelope_gain -= self.envelope_gain >> 8;
                    }
                    SPCEnvelopeState::Sustain => {
                        // rate = rrrrrのExp. decreaseと同じ
                        self.envelope_gain -= 1;
                        self.envelope_gain -= self.envelope_gain >> 8;
                    }
                    SPCEnvelopeState::Release => {
                        self.envelope_gain -= 8;
                    }
                }
            } else {
                match self.gain_mode {
                    SPCVoiceGainMode::Fixed { gain } => {
                        self.envelope_gain = (gain as i32) << 4;
                    }
                    SPCVoiceGainMode::LinearDecrease { .. } => {
                        self.envelope_gain -= 32;
                    }
                    SPCVoiceGainMode::ExponentialDecrease { .. } => {
                        self.envelope_gain -= 1;
                        self.envelope_gain -= self.envelope_gain >> 8;
                    }
                    SPCVoiceGainMode::LinearIncrease { .. } => {
                        self.envelope_gain += 32;
                    }
                    SPCVoiceGainMode::BentIncrease { .. } => {
                        self.envelope_gain += if self.envelope_gain < 0x600 { 32 } else { 8 };
                    }
                }
            }

            // エンベロープ状態更新（これはエンベロープの有効無効に関係なく実行）
            // ゲインは範囲制限前の値を使用
            match self.envelope_state {
                SPCEnvelopeState::Attack => {
                    if self.envelope_gain >= 0x7E0 {
                        self.envelope_state = SPCEnvelopeState::Decay;
                        self.envelope_rate = self.decay_rate;
                    }
                }
                SPCEnvelopeState::Decay => {
                    if ((self.envelope_gain >> 8) & 0x7) <= (self.sustain_level as i32) {
                        self.envelope_state = SPCEnvelopeState::Sustain;
                        self.envelope_rate = self.sustain_rate;
                    }
                }
                SPCEnvelopeState::Sustain | SPCEnvelopeState::Release => {}
            }

            // ゲインの範囲制限
            self.envelope_gain = self.envelope_gain.clamp(0, 0x7FF);
        }
        out = (((out as i32) * self.envelope_gain) >> 11) as i16;

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
            mute: false,
            echo_buffer_write_enable: false,
            noise_clock: 0,
            echo_feedback: 0,
            echo: [false; 8],
            brr_dir_page: 0,
            echo_buffer_address: 0,
            echo_buffer_size: 0,
            echo_buffer_pos: 0,
            fir_coef: [0; 8],
            fir_buffer: [[0; 8]; 2],
            fir_buffer_pos: 0,
            voice: [SPCVoiceRegister::new(); 8],
            global_counter: 0,
        }
    }

    /// 128バイトメモリから初期化
    pub fn initialize_dsp_register(&mut self, ram: &[u8], dsp_register: &[u8; 128]) {
        // DIRは先に設定（初期状態でKONがある場合にアドレスを正しくするため）
        self.write_dsp_register(ram, DIR_ADDRESS, dsp_register[DIR_ADDRESS as usize]);

        // すべてのレジスタを設定
        for i in 0..128 {
            self.write_dsp_register(ram, i, dsp_register[i as usize]);
        }

        // ENDXは最後に直接設定（通常の設定処理ではすべてクリアされるため）
        let endx = dsp_register[ENDX_ADDRESS as usize];
        for ch in 0..8 {
            self.voice[ch].decoder.end = ((endx >> ch) & 0x1) != 0;
        }
    }

    /// DSPレジスタの書き込み処理
    pub fn write_dsp_register(&mut self, ram: &[u8], address: u8, value: u8) {
        println!("DSPW: {:02X} <- {:02X}", address, value);
        match address & 0x7F {
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
                    self.voice[ch].keyon = ((value >> ch) & 0x1) != 0;
                }
            }
            KOFF_ADDRESS => {
                for ch in 0..8 {
                    let keyoff = ((value >> ch) & 0x1) != 0;
                    self.voice[ch].keyoff = keyoff;
                    // Releaseに移行
                    // サンプル処理する前にKOFFがクリアされることがあるため、即時に反映
                    if keyoff {
                        self.voice[ch].envelope_state = SPCEnvelopeState::Release;
                        self.voice[ch].envelope_rate = 31; // 毎サンプル更新
                    }
                }
            }
            FLG_ADDRESS => {
                // FIXME: RESETは無視
                self.mute = (value & 0x40) != 0;
                self.echo_buffer_write_enable = (value & 0x20) == 0; // ~ECEN
                self.noise_clock = value & 0x1F;
                // 読まれる可能性があるので、値としては保持しておく
                self.flag = value;
            }
            ENDX_ADDRESS => {
                // 注意：書かれた値に関係なくすべてのフラグをクリア
                for ch in 0..8 {
                    self.voice[ch].decoder.end = false;
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
                for ch in 0..8 {
                    self.voice[ch].brr_dir_address_base = (value as usize) << 8;
                }
            }
            ESA_ADDRESS => {
                self.echo_buffer_address = (value as usize) << 8;
            }
            EDL_ADDRESS => {
                self.echo_buffer_size = if (value & 0x0F) == 0 {
                    4
                } else {
                    ((value & 0x0F) as usize) << 11
                };
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
                        // デコードアドレスを更新
                        let dir_address =
                            self.voice[ch].brr_dir_address_base + 4 * (value as usize);
                        self.voice[ch].decoder.decode_start_address =
                            make_u16_from_u8(&ram[dir_address..(dir_address + 2)]) as usize;
                        self.voice[ch].decoder.decode_loop_address =
                            make_u16_from_u8(&ram[(dir_address + 2)..(dir_address + 4)]) as usize;
                        self.voice[ch].sample_source = value;
                    }
                    V0ADSR1_ADDRESS => {
                        self.voice[ch].adsr_enable = (value & 0x80) != 0;
                        self.voice[ch].attack_rate = 2 * (value & 0xF) + 1;
                        self.voice[ch].decay_rate = 2 * ((value >> 4) & 0x7) + 16;
                    }
                    V0ADSR2_ADDRESS => {
                        self.voice[ch].sustain_rate = value & 0x1F;
                        if self.voice[ch].adsr_enable {
                            self.voice[ch].sustain_level = (value >> 5) & 0x7;
                        } else {
                            // ADSRが無効のときは V0GAIN_ADDRESS の上位3bit
                            self.voice[ch].sustain_level = (self.voice[ch].gain_value >> 5) & 0x7;
                        }
                    }
                    V0GAIN_ADDRESS => {
                        if (value & 0x80) == 0 {
                            self.voice[ch].gain_mode =
                                SPCVoiceGainMode::Fixed { gain: value & 0x7F };
                        } else {
                            self.voice[ch].gain_mode = match (value >> 5) & 0x3 {
                                0 => SPCVoiceGainMode::LinearDecrease { rate: value & 0x1F },
                                1 => SPCVoiceGainMode::ExponentialDecrease { rate: value & 0x1F },
                                2 => SPCVoiceGainMode::LinearIncrease { rate: value & 0x1F },
                                3 => SPCVoiceGainMode::BentIncrease { rate: value & 0x1F },
                                _ => unreachable!("Unsupported Gain Type!"),
                            };
                        }
                        // ADSRが無効であれば即時反映
                        if !self.voice[ch].adsr_enable {
                            match self.voice[ch].gain_mode {
                                SPCVoiceGainMode::Fixed { gain } => {
                                    self.voice[ch].envelope_gain = (gain as i32) << 4;
                                }
                                SPCVoiceGainMode::LinearDecrease { rate }
                                | SPCVoiceGainMode::ExponentialDecrease { rate }
                                | SPCVoiceGainMode::LinearIncrease { rate }
                                | SPCVoiceGainMode::BentIncrease { rate } => {
                                    self.voice[ch].envelope_rate = rate;
                                }
                            }
                        }
                        // sustain_levelの設定で参照するため設定値を保持
                        self.voice[ch].gain_value = value;
                    }
                    V0ENVX_ADDRESS => {
                        // 書き込みは無視される（読み取り用レジスタ）
                        // 実際は書き込んで操作できるが、そのような使い方は考慮外とする
                    }
                    V0OUTX_ADDRESS => {
                        // 書き込めるけど意味はない（読み取り用レジスタ）
                        self.voice[ch].output_sample = (value as i16) << 8;
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
            ESA_ADDRESS => ((self.echo_buffer_address >> 8) & 0xFF) as u8,
            EDL_ADDRESS => ((self.echo_buffer_size >> 11) & 0xFF) as u8,
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
                    V0GAIN_ADDRESS => self.voice[ch].gain_value,
                    V0ENVX_ADDRESS => ((self.voice[ch].envelope_gain >> 4) & 0xFF) as u8,
                    V0OUTX_ADDRESS => ((self.voice[ch].output_sample >> 8) & 0xFF) as u8,
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

    /// FIRフィルタ出力計算
    fn compute_fir(&mut self, ram: &[u8]) -> [i16; 2] {
        // エコーバッファのアドレス
        let echo_buffer_addr = self.echo_buffer_address + self.echo_buffer_pos;

        // FIRバッファ更新
        for ch in 0..2 {
            let buf = (((ram[echo_buffer_addr + 2 * ch + 1] as i8) as i16) << 8)
                | (ram[echo_buffer_addr + 2 * ch + 0] as i16);
            self.fir_buffer[ch][self.fir_buffer_pos] = buf >> 1; // 下位1bitは捨てられる
        }

        // FIRフィルタ計算
        let mut out = [0; 2];
        for ch in 0..2 {
            for i in 0..8 {
                let buf =
                    self.fir_buffer[ch][(self.fir_buffer_pos.wrapping_sub(7 - i)) & 0x7] as i32;
                out[ch] += (buf * (self.fir_coef[i] as i32)) >> 6;
            }
        }
        self.fir_buffer_pos = (self.fir_buffer_pos + 1) & 0x7;

        [out[0] as i16, out[1] as i16]
    }

    /// エコーバッファの更新
    fn put_echo_buffer(&mut self, ram: &mut [u8], echo_in: &[i32; 2]) {
        // リングバッファ書き込み
        if self.echo_buffer_write_enable {
            let echo_buffer_addr = self.echo_buffer_address + self.echo_buffer_pos;
            ram[echo_buffer_addr + 0] = ((echo_in[0] >> 0) & 0xFF) as u8;
            ram[echo_buffer_addr + 1] = ((echo_in[0] >> 8) & 0xFF) as u8;
            ram[echo_buffer_addr + 2] = ((echo_in[1] >> 0) & 0xFF) as u8;
            ram[echo_buffer_addr + 3] = ((echo_in[1] >> 8) & 0xFF) as u8;
        }
        self.echo_buffer_pos = (self.echo_buffer_pos + 4) % self.echo_buffer_size;
    }

    /// ステレオサンプル計算処理
    pub fn compute_sample(&mut self, ram: &mut [u8]) -> [i16; 2] {
        let mut out = [0i32; 2];
        let mut echo_in = [0i32; 2];
        // 全チャンネルの出力をミックス
        let mut prev_voice_out = 0;
        for ch in 0..8 {
            let vout = self.voice[ch].compute_sample(ram, self.global_counter, prev_voice_out);
            out[0] += vout[0] as i32;
            out[1] += vout[1] as i32;
            if self.echo[ch] {
                echo_in[0] += vout[0] as i32;
                echo_in[1] += vout[1] as i32;
            }
            prev_voice_out = self.voice[ch].output_sample;
        }
        // エコー成分計算
        let fir_out = self.compute_fir(ram);
        // マスターボリューム適用・エコー成分加算
        for ch in 0..2 {
            out[ch] = (out[ch] * (self.volume[ch] as i32)) >> 7;
            out[ch] += ((fir_out[ch] as i32) * (self.echo_volume[ch] as i32)) >> 7;
        }
        // フィードバック成分加算・エコーバッファ更新
        for ch in 0..2 {
            echo_in[ch] += ((fir_out[ch] as i32) * (self.echo_feedback as i32)) >> 7;
        }
        self.put_echo_buffer(ram, &echo_in);
        // ミュートならば無音
        if self.mute {
            for ch in 0..2 {
                out[ch] = 0;
            }
        }
        // グローバルカウンタの更新
        if self.global_counter == 0 {
            self.global_counter = 0x77FF;
        }
        self.global_counter -= 1;

        [out[0] as i16, out[1] as i16]
    }
}
