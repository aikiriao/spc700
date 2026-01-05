use crate::eg::*;
use crate::types::*;
use core::f32::consts::PI;
use libm;
use log::trace;

/// パーカッションパートのチャンネル
const MIDI_PERCUSSION_CHANNEL: u8 = 0x09;

/// MIDIメッセージ：ノートオン
const MIDIMSG_NOTE_ON: u8 = 0x90;
/// MIDIメッセージ：ノートオフ
const MIDIMSG_NOTE_OFF: u8 = 0x80;
/// MIDIメッセージ：コントロールチェンジ
const MIDIMSG_CONTROL_CHANGE: u8 = 0xB0;
/// MIDIメッセージ：プログラムチェンジ
const MIDIMSG_PROGRAM_CHANGE: u8 = 0xC0;
/// MIDIメッセージ：ピッチベンド
const MIDIMSG_PITCH_BEND: u8 = 0xE0;

/// MIDIコントロールチェンジ：チャンネルボリューム
const MIDICC_CHANNEL_VOLUME: u8 = 0x07;
/// MIDIコントロールチェンジ：パンポット
const MIDICC_PANPOT: u8 = 0x0A;
/// MIDIコントロールチェンジ：エクスプレッション
const MIDICC_EXPRESSION: u8 = 0x0B;
/// MIDIコントロールチェンジ：RPN LSB
const MIDICC_RPN_LSB: u8 = 0x64;
/// MIDIコントロールチェンジ：RPN MSB
const MIDICC_RPN_MSB: u8 = 0x65;
/// MIDIコントロールチェンジ：RPN データエントリーLSB
const MIDICC_RPN_DATA_ENTRY_LSB: u8 = 0x06;
/// MIDIコントロールチェンジ：RPN データエントリーMSB
const MIDICC_RPN_DATA_ENTRY_MSB: u8 = 0x26;
/// MIDIコントロールチェンジ：エフェクト1デプス
const MIDICC_EFFECT1_DEPTH: u8 = 0x5B;

/// MIDI出力のための独自追加アドレス

/// 設定・取得対象のサンプル番号(SRN)
pub const DSP_ADDRESS_SRN_TARGET: u8 = 0x0A;
/// SRNのフラグ EVP00000
/// E: エンベロープをエクスプレッションとして出力
/// V: ボリュームの再生中更新
/// P: ピッチベンド
pub const DSP_ADDRESS_SRN_FLAG: u8 = 0x1A;
/// SRNのプログラム番号 0x00 - 0x7FはGMと同等、0x80-0xFFはドラムキット音色+0x80
pub const DSP_ADDRESS_SRN_PROGRAM: u8 = 0x2A;
/// SRNの中央に該当するノート（基準ピッチ）の整数部8bit
pub const DSP_ADDRESS_SRN_CENTER_NOTE: u8 = 0x3A;
/// SRNの中央に該当するノート（基準ピッチ）の小数部8bit
pub const DSP_ADDRESS_SRN_CENTER_NOTE_FRACTION: u8 = 0x4A;
/// SRNのノートオンのベロシティ値
pub const DSP_ADDRESS_SRN_NOTEON_VELOCITY: u8 = 0x5A;
/// SRNのピッチベンドセンシティビティ
pub const DSP_ADDRESS_SRN_PITCHBEND_SENSITIVITY: u8 = 0x6A;
/// ノートオンフラグ
pub const DSP_ADDRESS_NOTEON: u8 = 0x0B;
/// エンベロープ・ボリューム・ピッチベンド更新間隔(ms)
pub const DSP_ADDRESS_PLAYBACK_PARAMETER_UPDATE_PERIOD: u8 = 0x1B;

/// ボイス
#[derive(Copy, Clone, Debug)]
struct MIDIVoiceRegister {
    /// チャンネル番号(0-7)
    channel: u8,
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
    /// キーオンされているか
    keyon: bool,
    /// キーオフされているか
    keyoff: bool,
    /// ノートオンされているか
    noteon: bool,
    /// ノートオンした音はドラムか
    noteon_drum: bool,
    /// 前ボイス出力のピッチモジュレーションをするか
    pitch_mod: bool,
    /// ノイズ有効か
    noise: bool,
    /// エコー有効か
    echo: bool,
    /// エンベロープが更新されたか
    envelope_updated: bool,
    /// ボリュームが更新されたか
    volume_updated: bool,
    /// 最後に発声した音のノート番号
    last_note: u8,
    /// ピッチベンドの基準ピッチ（最後に発声した音のピッチ）
    pitch_bend_base: u16,
    /// 最後に設定したピッチベンド値
    last_pitch: u16,
    /// 最後に設定したプログラム番号
    last_program: u8,
}

/// 各サンプルに対応するマップ
struct SampleSourceMap {
    /// プログラム番号（音色）
    program: [u8; 256],
    /// 基準ノート（ピッチ） 整数部8bit, 小数部8bit
    center_note: [u16; 256],
    /// ノートオンベロシティ
    noteon_velocity: [u8; 256],
    /// ピッチベンドセンシティビティ
    pitchbend_sensitibity: [u8; 256],
    /// エンベロープ出力有効か
    output_envelope: [bool; 256],
    /// 再生中のボリューム・パン出力有効か
    output_volume_pan: [bool; 256],
    /// ピッチベンド出力有効か
    output_pitch_bend: [bool; 256],
    /// エコーをエフェクト1デプス（リバーブ）として出力するか
    echo_as_effect1_depth: [bool; 256],
}

/// MIDI-DSP
pub struct MIDIDSP {
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
    /// BRRのディレクトリのページ
    brr_dir_page: u8,
    /// ゲイン更新用のカウンタ
    global_counter: u16,
    /// 各チャンネルのボイス
    voice: [MIDIVoiceRegister; 8],
    /// 各サンプル番号に対応するマップ
    sample_source_map: SampleSourceMap,
    /// 設定対象のサンプル番号
    sample_source_target: usize,
    /// エンベロープ・ボリューム・ピッチベンド更新間隔サンプル
    playback_parameter_update_period: u16,
}

/// ピッチをMIDIノート番号に変換
fn pitch_to_note(center_note: u16, pitch: u16) -> u8 {
    // pitch(2^12を基準とする再生速度)から半音単位でのずれを計算
    // 例1）pitch = 2048 -> semitone = -12(-1 octave)
    // 例2）pitch = 4096 -> semitone =   0
    // 例3）pitch = 8192 -> semitone =  12(+1 octave)
    // 12 * log2(pitch / 4096) = 12 * (log2(pitch) - 12)
    let mut semitone = 12.0 * (libm::log2f(pitch as f32) - 12.0);
    // 基準ノート値（固定小数 8bit整数/8bit小数）を加算
    const NOTE_FRACTION_FACTOR: f32 = 1.0 / 256.0;
    semitone += center_note as f32 * NOTE_FRACTION_FACTOR;
    libm::roundf(semitone).clamp(0.0, 127.0) as u8
}

/// LRボリュームをボリュームとパンの組に変換
fn lrvolume_to_volume_and_pan(lrvolume: &[i8; 2]) -> (u8, u8) {
    // 振幅（パワー）比を維持するように設定
    let volume = lrvolume[0].max(lrvolume[1]) as u8;
    let pan = if lrvolume[0] == 0 && lrvolume[1] == 0 {
        64
    } else if lrvolume[0] == 0 {
        127
    } else if lrvolume[1] == 0 {
        0
    } else {
        const FACTOR: f32 = 256.0 / PI;
        libm::roundf(FACTOR * libm::atanf(lrvolume[0] as f32 / lrvolume[1] as f32)) as u8
    };
    (volume, pan)
}

/// エコーボリュームをエフェクト1デプスに変換
fn echovolume_to_effect1_depth(echo_volume: &[i8; 2]) -> u8 {
    (echo_volume[0].abs() + echo_volume[1].abs()) as u8 / 2
}

impl MIDIOutput {
    /// MIDIメッセージを追加
    fn push_message(&mut self, data: &[u8]) {
        assert!(data.len() <= 3);
        assert!(self.num_messages < MAX_NUM_MIDI_OUTPUT_MESSAGES);
        for i in 0..data.len() {
            self.messages[self.num_messages].data[i] = data[i];
        }
        self.messages[self.num_messages].length = data.len();
        self.num_messages += 1;
    }
}

impl MIDIVoiceRegister {
    fn new(ch: u8) -> Self {
        Self {
            channel: ch,
            volume: [0; 2],
            pitch: 0,
            brr_dir_address_base: 0,
            sample_source: 0,
            eg: EnvelopeGenerator::new(),
            keyon: false,
            keyoff: false,
            noteon: false,
            noteon_drum: false,
            pitch_mod: false,
            noise: false,
            echo: false,
            envelope_updated: false,
            volume_updated: false,
            last_note: 0,
            pitch_bend_base: 0,
            last_pitch: 0,
            last_program: 0,
        }
    }

    /// 32kHz定期処理
    fn tick(
        &mut self,
        echo_volume: u8,
        global_counter: u16,
        playback_parameter_update: bool,
        srn_map: &SampleSourceMap,
        out: &mut MIDIOutput,
    ) {
        // キーオンが入ったとき
        if self.keyon {
            self.keyon = false;
            // キーオフが漏れていた場合はノートオフを送信
            if self.noteon {
                let first_byte = if self.noteon_drum {
                    MIDIMSG_NOTE_OFF | MIDI_PERCUSSION_CHANNEL
                } else {
                    MIDIMSG_NOTE_OFF | self.channel
                };
                out.push_message(&[first_byte, self.last_note, 0]);
            }
            // エンベロープ設定
            self.eg.keyon();
            // ノートオン
            let program = srn_map.program[self.sample_source as usize];
            let (volume, pan) = lrvolume_to_volume_and_pan(&self.volume);
            let effect1_depth =
                if self.echo && srn_map.echo_as_effect1_depth[self.sample_source as usize] {
                    echo_volume
                } else {
                    0
                };
            let channel = if program <= 0x7F {
                self.channel
            } else {
                MIDI_PERCUSSION_CHANNEL
            };
            let note = if program <= 0x7F {
                pitch_to_note(srn_map.center_note[self.sample_source as usize], self.pitch)
            } else {
                program - 0x80
            };
            if program <= 0x7F {
                // 音色が変わっていたらプログラムチェンジを送信
                if program != self.last_program {
                    out.push_message(&[MIDIMSG_PROGRAM_CHANGE | channel, program]);
                    // ピッチベンドセンシティビティ設定
                    let first_byte = MIDIMSG_CONTROL_CHANGE | channel;
                    out.push_message(&[first_byte, MIDICC_RPN_MSB, 0x00]);
                    out.push_message(&[first_byte, MIDICC_RPN_LSB, 0x00]);
                    out.push_message(&[
                        first_byte,
                        MIDICC_RPN_DATA_ENTRY_LSB,
                        srn_map.pitchbend_sensitibity[self.sample_source as usize],
                    ]);
                    out.push_message(&[first_byte, MIDICC_RPN_DATA_ENTRY_MSB, 0]);
                    self.last_program = program;
                }
                self.pitch_bend_base = self.pitch;
                self.last_pitch = self.pitch;
            }
            // ボリューム・パン
            out.push_message(&[
                MIDIMSG_CONTROL_CHANGE | channel,
                MIDICC_CHANNEL_VOLUME,
                volume,
            ]);
            out.push_message(&[MIDIMSG_CONTROL_CHANGE | channel, MIDICC_PANPOT, pan]);
            // エフェクト1デプス
            out.push_message(&[
                MIDIMSG_CONTROL_CHANGE | channel,
                MIDICC_EFFECT1_DEPTH,
                effect1_depth,
            ]);
            // エクスプレッション
            let initial_expression = if srn_map.output_envelope[self.sample_source as usize] {
                ((self.eg.gain >> 4) & 0x7F) as u8
            } else {
                0x7F
            };
            out.push_message(&[
                MIDIMSG_CONTROL_CHANGE | channel,
                MIDICC_EXPRESSION,
                initial_expression,
            ]);
            // ピッチベンドの設定値を中心(8192)に戻す
            out.push_message(&[MIDIMSG_PITCH_BEND | channel, 0, 0x40]);
            // ノートオン発行
            out.push_message(&[
                MIDIMSG_NOTE_ON | channel,
                note,
                srn_map.noteon_velocity[self.sample_source as usize],
            ]);
            self.noteon_drum = program > 0x7F;
            self.last_note = note;
            self.envelope_updated = false;
            self.noteon = true;
        }

        // キーオフが入ったとき
        if self.keyoff {
            self.keyoff = false;
            // ノートオフ
            if self.noteon {
                let first_byte = if self.noteon_drum {
                    MIDIMSG_NOTE_OFF | MIDI_PERCUSSION_CHANNEL
                } else {
                    MIDIMSG_NOTE_OFF | self.channel
                };
                out.push_message(&[first_byte, self.last_note, 0]);
                self.noteon = false;
            }
        }

        // エンベロープ内部状態更新
        if self.eg.update(global_counter) && !self.envelope_updated {
            self.envelope_updated = true;
        }

        // 再生パラメータ更新（過剰に送ると遅延するので間引く）
        if self.noteon && !self.noteon_drum && playback_parameter_update {
            // エクスプレッション（エンベロープ）
            if self.envelope_updated && srn_map.output_envelope[self.sample_source as usize] {
                out.push_message(&[
                    MIDIMSG_CONTROL_CHANGE | self.channel,
                    MIDICC_EXPRESSION,
                    ((self.eg.gain >> 4) & 0x7F) as u8,
                ]);
                self.envelope_updated = false;
            }
            // ボリューム・パン
            if self.volume_updated && srn_map.output_volume_pan[self.sample_source as usize] {
                let (volume, pan) = lrvolume_to_volume_and_pan(&self.volume);
                out.push_message(&[
                    MIDIMSG_CONTROL_CHANGE | self.channel,
                    MIDICC_CHANNEL_VOLUME,
                    volume,
                ]);
                out.push_message(&[MIDIMSG_CONTROL_CHANGE | self.channel, MIDICC_PANPOT, pan]);
                self.volume_updated = false;
            }
            // ピッチベンド
            if self.noteon
                && self.last_pitch != self.pitch
                && srn_map.output_pitch_bend[self.sample_source as usize]
            {
                let max_semitone =
                    srn_map.pitchbend_sensitibity[self.sample_source as usize] as f32;
                let pitchbend_ratio =
                    libm::log2f((self.pitch as f32) / (self.pitch_bend_base as f32)) * max_semitone
                        / 12.0;
                // [-max_semitone,max_semitone]半音を[-8192,8192]に対応付ける
                let pitch_bend =
                    libm::roundf((pitchbend_ratio * 8192.0).clamp(-8192.0, 8191.0)) as i16 + 8192;
                // 7bitを2分割
                out.push_message(&[
                    MIDIMSG_PITCH_BEND | self.channel,
                    (pitch_bend & 0x7F) as u8,        // LSB
                    ((pitch_bend >> 7) & 0x7F) as u8, // MSB
                ]);
                self.last_pitch = self.pitch;
            }
        }
    }
}

impl SPCDSP for MIDIDSP {
    type Output = MIDIOutput;

    /// コンストラクタ
    fn new() -> Self {
        Self {
            volume: [0; 2],
            echo_volume: [0; 2],
            flag: 0,
            mute: false,
            noise_clock: 0,
            brr_dir_page: 0,
            voice: [
                // 0(1ch)は特別な意味を持つ場合があるので空けておく
                MIDIVoiceRegister::new(1),
                MIDIVoiceRegister::new(2),
                MIDIVoiceRegister::new(3),
                MIDIVoiceRegister::new(4),
                MIDIVoiceRegister::new(5),
                MIDIVoiceRegister::new(6),
                MIDIVoiceRegister::new(7),
                MIDIVoiceRegister::new(8),
            ],
            global_counter: 0,
            sample_source_map: SampleSourceMap {
                program: [0; 256],
                center_note: [64 << 8; 256], // 中心ノートは64で仮置き
                noteon_velocity: [0x7F; 256],
                pitchbend_sensitibity: [12; 256],
                output_envelope: [true; 256],
                output_volume_pan: [true; 256],
                output_pitch_bend: [true; 256],
                echo_as_effect1_depth: [true; 256],
            },
            sample_source_target: 0,
            playback_parameter_update_period: 160,
        }
    }

    /// 128バイトメモリから初期化
    fn initialize(&mut self, ram: &mut [u8], dsp_register: &[u8; 128]) {
        // DIRは先に設定（初期状態でKONがある場合にアドレスを正しくするため）
        self.write_register(ram, DSP_ADDRESS_DIR, dsp_register[DSP_ADDRESS_DIR as usize]);

        // すべてのレジスタを設定
        for i in 0..128 {
            self.write_register(ram, i, dsp_register[i as usize]);
        }
    }

    /// DSPレジスタの書き込み処理
    fn write_register(&mut self, _ram: &[u8], address: u8, value: u8) {
        trace!("DSPW: {:02X} <- {:02X}", address, value);
        match address & 0x7F {
            DSP_ADDRESS_MVOLL => {
                self.volume[0] = value as i8;
            }
            DSP_ADDRESS_MVOLR => {
                self.volume[1] = value as i8;
            }
            DSP_ADDRESS_EVOLL => {
                self.echo_volume[0] = value as i8;
            }
            DSP_ADDRESS_EVOLR => {
                self.echo_volume[1] = value as i8;
            }
            DSP_ADDRESS_KON => {
                for ch in 0..8 {
                    self.voice[ch].keyon = ((value >> ch) & 0x1) != 0;
                }
            }
            DSP_ADDRESS_KOFF => {
                for ch in 0..8 {
                    let keyoff = ((value >> ch) & 0x1) != 0;
                    self.voice[ch].keyoff = keyoff;
                    // サンプル処理する前にKOFFがクリアされることがあるため、即時に反映
                    if keyoff {
                        self.voice[ch].eg.keyoff();
                    }
                }
            }
            DSP_ADDRESS_FLG => {
                // RESETは無視
                self.mute = (value & 0x40) != 0;
                self.noise_clock = value & 0x1F;
                // 読まれる可能性があるので、値としては保持しておく
                self.flag = value;
            }
            DSP_ADDRESS_ENDX => {
                // 何もしない
            }
            DSP_ADDRESS_EFB => {
                // 何もしない
            }
            DSP_ADDRESS_PMON => {
                for ch in 1..8 {
                    /* NOTE! 0は無効 */
                    self.voice[ch].pitch_mod = ((value >> ch) & 0x1) != 0;
                }
            }
            DSP_ADDRESS_NON => {
                for ch in 0..8 {
                    self.voice[ch].noise = ((value >> ch) & 0x1) != 0;
                }
            }
            DSP_ADDRESS_EON => {
                for ch in 0..8 {
                    self.voice[ch].echo = ((value >> ch) & 0x1) != 0;
                }
            }
            DSP_ADDRESS_DIR => {
                self.brr_dir_page = value;
                for ch in 0..8 {
                    self.voice[ch].brr_dir_address_base = (value as usize) << 8;
                }
            }
            DSP_ADDRESS_ESA => {
                // 何もしない
            }
            DSP_ADDRESS_EDL => {
                // 何もしない
            }
            DSP_ADDRESS_FIR0 | DSP_ADDRESS_FIR1 | DSP_ADDRESS_FIR2 | DSP_ADDRESS_FIR3
            | DSP_ADDRESS_FIR4 | DSP_ADDRESS_FIR5 | DSP_ADDRESS_FIR6 | DSP_ADDRESS_FIR7 => {
                // 何もしない
            }
            DSP_ADDRESS_SRN_TARGET => {
                self.sample_source_target = value as usize;
            }
            DSP_ADDRESS_SRN_FLAG => {
                self.sample_source_map.output_envelope[self.sample_source_target] =
                    (value & 0x80) != 0;
                self.sample_source_map.output_volume_pan[self.sample_source_target] =
                    (value & 0x40) != 0;
                self.sample_source_map.output_pitch_bend[self.sample_source_target] =
                    (value & 0x20) != 0;
                self.sample_source_map.echo_as_effect1_depth[self.sample_source_target] =
                    (value & 0x10) != 0;
            }
            DSP_ADDRESS_SRN_PROGRAM => {
                self.sample_source_map.program[self.sample_source_target] = value;
            }
            DSP_ADDRESS_SRN_CENTER_NOTE => {
                let note = self.sample_source_map.center_note[self.sample_source_target];
                self.sample_source_map.center_note[self.sample_source_target] =
                    ((value as u16) << 8) | (note & 0x00FF);
            }
            DSP_ADDRESS_SRN_CENTER_NOTE_FRACTION => {
                let note = self.sample_source_map.center_note[self.sample_source_target];
                self.sample_source_map.center_note[self.sample_source_target] =
                    ((value as u16) << 0) | (note & 0xFF00);
            }
            DSP_ADDRESS_SRN_NOTEON_VELOCITY => {
                self.sample_source_map.noteon_velocity[self.sample_source_target] = value;
            }
            DSP_ADDRESS_SRN_PITCHBEND_SENSITIVITY => {
                self.sample_source_map.pitchbend_sensitibity[self.sample_source_target] = value;
            }
            DSP_ADDRESS_NOTEON => {
                for ch in 0..8 {
                    self.voice[ch].noteon = ((value >> ch) & 0x1) != 0;
                }
            }
            DSP_ADDRESS_PLAYBACK_PARAMETER_UPDATE_PERIOD => {
                self.playback_parameter_update_period = (value as u16) << 5;
            }
            address if ((address & 0xF) <= 0x9) => {
                let ch = (address >> 4) as usize;
                match address & 0xF {
                    DSP_ADDRESS_V0VOLL => {
                        self.voice[ch].volume[0] = value as i8;
                        self.voice[ch].volume_updated = true;
                    }
                    DSP_ADDRESS_V0VOLR => {
                        self.voice[ch].volume[1] = value as i8;
                        self.voice[ch].volume_updated = true;
                    }
                    DSP_ADDRESS_V0PITCHL => {
                        self.voice[ch].pitch = (self.voice[ch].pitch & 0xFF00) | (value as u16);
                    }
                    DSP_ADDRESS_V0PITCHH => {
                        self.voice[ch].pitch =
                            ((value as u16) << 8) | (self.voice[ch].pitch & 0x00FF);
                    }
                    DSP_ADDRESS_V0SRCN => {
                        self.voice[ch].sample_source = value;
                    }
                    DSP_ADDRESS_V0ADSR1 => {
                        self.voice[ch].eg.set_adsr1(value);
                        self.voice[ch].envelope_updated = true;
                    }
                    DSP_ADDRESS_V0ADSR2 => {
                        self.voice[ch].eg.set_adsr2(value);
                        self.voice[ch].envelope_updated = true;
                    }
                    DSP_ADDRESS_V0GAIN => {
                        self.voice[ch].eg.set_gain(value);
                        self.voice[ch].envelope_updated = true;
                    }
                    DSP_ADDRESS_V0ENVX => {
                        // 書き込みは無視される（読み取り用レジスタ）
                        // 実際は書き込んで操作できるが、そのような使い方は考慮外とする
                    }
                    DSP_ADDRESS_V0OUTX => {
                        // 何もしない
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
            DSP_ADDRESS_MVOLL => self.volume[0] as u8,
            DSP_ADDRESS_MVOLR => self.volume[1] as u8,
            DSP_ADDRESS_EVOLL => self.echo_volume[0] as u8,
            DSP_ADDRESS_EVOLR => self.echo_volume[1] as u8,
            DSP_ADDRESS_KON => {
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
            DSP_ADDRESS_KOFF => {
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
            DSP_ADDRESS_FLG => self.flag,
            DSP_ADDRESS_ENDX => 0, // 0を返す
            DSP_ADDRESS_EFB => 0,  // 0を返す
            DSP_ADDRESS_PMON => {
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
            DSP_ADDRESS_NON => {
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
            DSP_ADDRESS_EON => {
                let mut ret = 0;
                let mut bit = 1;
                for ch in 0..8 {
                    if self.voice[ch].echo {
                        ret |= bit;
                    }
                    bit <<= 1;
                }
                ret
            }
            DSP_ADDRESS_DIR => self.brr_dir_page,
            DSP_ADDRESS_ESA => 0, // 0を返す
            DSP_ADDRESS_EDL => 0, // 0を返す
            DSP_ADDRESS_FIR0 | DSP_ADDRESS_FIR1 | DSP_ADDRESS_FIR2 | DSP_ADDRESS_FIR3
            | DSP_ADDRESS_FIR4 | DSP_ADDRESS_FIR5 | DSP_ADDRESS_FIR6 | DSP_ADDRESS_FIR7 => 0, // 0を返す
            DSP_ADDRESS_SRN_TARGET => self.sample_source_target as u8,
            DSP_ADDRESS_SRN_FLAG => {
                let mut value = 0;
                if self.sample_source_map.output_envelope[self.sample_source_target] {
                    value |= 0x80;
                }
                if self.sample_source_map.output_volume_pan[self.sample_source_target] {
                    value |= 0x40;
                }
                if self.sample_source_map.output_pitch_bend[self.sample_source_target] {
                    value |= 0x20;
                }
                if self.sample_source_map.echo_as_effect1_depth[self.sample_source_target] {
                    value |= 0x10;
                }
                value
            }
            DSP_ADDRESS_SRN_PROGRAM => self.sample_source_map.program[self.sample_source_target],
            DSP_ADDRESS_SRN_CENTER_NOTE => {
                ((self.sample_source_map.center_note[self.sample_source_target] >> 8) & 0xFF) as u8
            }
            DSP_ADDRESS_SRN_CENTER_NOTE_FRACTION => {
                ((self.sample_source_map.center_note[self.sample_source_target] >> 0) & 0xFF) as u8
            }
            DSP_ADDRESS_SRN_NOTEON_VELOCITY => {
                self.sample_source_map.noteon_velocity[self.sample_source_target]
            }
            DSP_ADDRESS_SRN_PITCHBEND_SENSITIVITY => {
                self.sample_source_map.pitchbend_sensitibity[self.sample_source_target]
            }
            DSP_ADDRESS_NOTEON => {
                let mut ret = 0;
                let mut bit = 1;
                for ch in 0..8 {
                    if self.voice[ch].noteon {
                        ret |= bit;
                    }
                    bit <<= 1;
                }
                ret
            }
            DSP_ADDRESS_PLAYBACK_PARAMETER_UPDATE_PERIOD => {
                (self.playback_parameter_update_period >> 5) as u8
            }
            address if ((address & 0xF) <= 0x9) => {
                let ch = (address >> 4) as usize;
                match address & 0xF {
                    DSP_ADDRESS_V0VOLL => self.voice[ch].volume[0] as u8,
                    DSP_ADDRESS_V0VOLR => self.voice[ch].volume[1] as u8,
                    DSP_ADDRESS_V0PITCHL => (self.voice[ch].pitch & 0xFF) as u8,
                    DSP_ADDRESS_V0PITCHH => ((self.voice[ch].pitch >> 8) & 0xFF) as u8,
                    DSP_ADDRESS_V0SRCN => self.voice[ch].sample_source,
                    DSP_ADDRESS_V0ADSR1 => self.voice[ch].eg.get_adsr1(),
                    DSP_ADDRESS_V0ADSR2 => self.voice[ch].eg.get_adsr2(),
                    DSP_ADDRESS_V0GAIN => self.voice[ch].eg.get_gain(),
                    DSP_ADDRESS_V0ENVX => ((self.voice[ch].eg.gain >> 4) & 0xFF) as u8,
                    DSP_ADDRESS_V0OUTX => 0, // 0を返す
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

    /// 32kHz周期処理
    fn tick(&mut self, _ram: &mut [u8]) -> Option<MIDIOutput> {
        let mut out = MIDIOutput {
            messages: [MIDIMessage {
                data: [0; 3],
                length: 0,
            }; MAX_NUM_MIDI_OUTPUT_MESSAGES],
            num_messages: 0,
        };
        // 全チャンネルの周期処理を実行
        let playback_parameter_update = if self.playback_parameter_update_period == 0 {
            false
        } else {
            (self.global_counter % self.playback_parameter_update_period) == 1
        };
        for ch in 0..8 {
            self.voice[ch].tick(
                echovolume_to_effect1_depth(&self.echo_volume),
                self.global_counter,
                playback_parameter_update,
                &self.sample_source_map,
                &mut out,
            );
        }
        // グローバルカウンタの更新
        if self.global_counter == 0 {
            self.global_counter = 0x77FF;
        }
        self.global_counter -= 1;

        // ミュートならば強制的に無音
        if self.mute || out.num_messages == 0 {
            None
        } else {
            Some(out)
        }
    }
}
