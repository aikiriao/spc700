use crate::decoder::*;
use crate::eg::*;
use crate::types::*;
use log::trace;

/// ボイス
#[derive(Copy, Clone, Debug)]
struct VoiceRegister {
    /// LRチャンネルのボリューム
    volume: [i8; 2],
    /// 再生ピッチ（サンプル参照位置の増加幅）
    pitch: u16,
    /// デコードアドレスが入っているアドレス
    brr_dir_address_base: usize,
    /// 再生対象の音源サンプル
    sample_source: u8,
    /// エンベロープジェネレータ
    eg: EnvelopeGenerator,
    /// LRゲイン適用前の、最後に出力したサンプル値
    output_sample: i16,
    /// キーオンされているか
    keyon: bool,
    /// キーオフされているか
    keyoff: bool,
    /// 前ボイス出力のピッチモジュレーションをするか
    pitch_mod: bool,
    /// ノイズ有効か
    noise: bool,
    /// デコーダ
    decoder: Decoder,
}

/// S-DSP
#[derive(Copy, Clone, Debug)]
pub struct SDSP {
    /// マスターボリューム
    volume: [i8; 2],
    /// エコーボリューム
    echo_volume: [i8; 2],
    /// フラグ
    flag: u8,
    /// ミュートするか
    mute: bool,
    /// ノイズ周波数
    noise_clock: u8,
    /// エコーフィードバック係数
    echo_feedback: i8,
    /// エコーバッファに書き込むか
    echo_buffer_write_enable: bool,
    /// 各チャンネルのエコー有効フラグ
    echo: [bool; 8],
    /// BRRのディレクトリのページ
    brr_dir_page: u8,
    /// エコーバッファの開始アドレス
    echo_buffer_address: usize,
    /// エコーバッファサイズ
    echo_buffer_size: usize,
    /// エコーバッファ参照位置
    echo_buffer_pos: usize,
    /// FIRフィルタ係数
    fir_coef: [i8; 8],
    /// LRチャンネルのFIRフィルタバッファ
    fir_buffer: [[i16; 8]; 2],
    /// FIRフィルタバッファ参照位置
    fir_buffer_pos: usize,
    /// ゲイン更新用のカウンタ
    global_counter: u16,
    /// 各チャンネルのボイス
    voice: [VoiceRegister; 8],
}

impl VoiceRegister {
    fn new() -> Self {
        Self {
            volume: [0; 2],
            pitch: 0,
            brr_dir_address_base: 0,
            sample_source: 0,
            eg: EnvelopeGenerator::new(),
            output_sample: 0,
            keyon: false,
            keyoff: false,
            pitch_mod: false,
            noise: false,
            decoder: Decoder::new(),
        }
    }

    /// 1ステレオサンプル計算
    fn tick(&mut self, ram: &[u8], global_counter: u16, prev_voice_out: i16) -> [i16; 2] {
        // キーオンが入ったとき
        if self.keyon {
            self.keyon = false;
            // エンベロープ設定
            self.eg.keyon();
            // デコーダ設定
            self.decoder.keyon(
                ram,
                self.brr_dir_address_base + 4 * (self.sample_source as usize),
            );
        }

        // キーオフが入ったとき
        if self.keyoff {
            // フラグクリア
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
                self.eg.mute();
            }
        }

        // デコード後の出力サンプル更新
        self.output_sample = out;

        // TODO: NON

        // エンベロープ内部状態更新
        self.eg.update(global_counter);

        // エンベロープ適用
        out = (((out as i32) * self.eg.gain) >> 11) as i16;

        // 左右ボリューム適用
        let lout = ((out as i32) * (self.volume[0] as i32)) >> 7;
        let rout = ((out as i32) * (self.volume[1] as i32)) >> 7;

        [lout as i16, rout as i16]
    }
}

impl SDSP {
    pub fn new() -> SDSP {
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
            voice: [VoiceRegister::new(); 8],
            global_counter: 0,
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
}

impl SPCDSP for SDSP {
    /// 128バイトメモリから初期化
    fn initialize(&mut self, ram: &mut [u8], dsp_register: &[u8; 128]) {
        // DIRは先に設定（初期状態でKONがある場合にアドレスを正しくするため）
        self.write_register(ram, DIR_ADDRESS, dsp_register[DIR_ADDRESS as usize]);

        // すべてのレジスタを設定
        for i in 0..128 {
            self.write_register(ram, i, dsp_register[i as usize]);
        }

        // ENDXは最後に直接設定（通常の設定処理ではすべてクリアされるため）
        let endx = dsp_register[ENDX_ADDRESS as usize];
        for ch in 0..8 {
            self.voice[ch].decoder.end = ((endx >> ch) & 0x1) != 0;
        }

        // エコーバッファの内容をクリア（初期のRAMに信号が残っている場合がある）
        if self.echo_buffer_address != 0 && self.echo_buffer_size != 0 {
            for i in 0..self.echo_buffer_size {
                ram[self.echo_buffer_address + i] = 0;
            }
        }
    }

    /// DSPレジスタの書き込み処理
    fn write_register(&mut self, ram: &[u8], address: u8, value: u8) {
        trace!("DSPW: {:02X} <- {:02X}", address, value);
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
                    // サンプル処理する前にKOFFがクリアされることがあるため、即時に反映
                    if keyoff {
                        self.voice[ch].eg.keyoff();
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
                        self.voice[ch].decoder.set_address(
                            ram,
                            self.voice[ch].brr_dir_address_base + 4 * (value as usize),
                        );
                        self.voice[ch].sample_source = value;
                    }
                    V0ADSR1_ADDRESS => {
                        self.voice[ch].eg.set_adsr1(value);
                    }
                    V0ADSR2_ADDRESS => {
                        self.voice[ch].eg.set_adsr2(value);
                    }
                    V0GAIN_ADDRESS => {
                        self.voice[ch].eg.set_gain(value);
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
    fn read_register(&self, _ram: &[u8], address: u8) -> u8 {
        trace!("DSPR: {:02X}", address);
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
                    V0ADSR1_ADDRESS => self.voice[ch].eg.get_adsr1(),
                    V0ADSR2_ADDRESS => self.voice[ch].eg.get_adsr2(),
                    V0GAIN_ADDRESS => self.voice[ch].eg.get_gain(),
                    V0ENVX_ADDRESS => ((self.voice[ch].eg.gain >> 4) & 0xFF) as u8,
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

    /// ステレオサンプル計算処理
    fn tick(&mut self, ram: &mut [u8]) -> [i16; 2] {
        let mut out = [0i32; 2];
        let mut echo_in = [0i32; 2];
        let mut prev_voice_out = 0;
        // 全チャンネルの出力をミックス
        for ch in 0..8 {
            let vout = self.voice[ch].tick(ram, self.global_counter, prev_voice_out);
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
        // フィードバック成分加算
        for ch in 0..2 {
            echo_in[ch] += ((fir_out[ch] as i32) * (self.echo_feedback as i32)) >> 7;
        }
        // エコーバッファ更新
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
