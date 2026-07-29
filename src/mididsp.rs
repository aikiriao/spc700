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
/// MIDIコントロールチェンジ：リバーブデプス
const MIDICC_REVERB_DEPTH: u8 = 0x5B;
/// MIDIコントロールチェンジ：コーラスデプス
const MIDICC_CHORUS_DEPTH: u8 = 0x5D;

/// MIDI出力のための独自追加アドレス

/// 設定・取得対象のサンプル番号(SRCN)
pub const DSP_ADDRESS_SRCN_TARGET: u8 = 0x0A;
/// SRCNのフラグ MEU00000
/// M: ミュートフラグ（1ならばメッセージを出力しない）
/// E: エンベロープをエクスプレッションとして出力
/// U: ノートオンの後にパン・ボリューム・エクスプレッション・ピッチベンドを変えるか
pub const DSP_ADDRESS_SRCN_FLAG: u8 = 0x0B;
/// SRCNのプログラム番号 0x00 - 0x7FはGMと同等、0x80-0xFFはドラムキット音色+0x80
pub const DSP_ADDRESS_SRCN_PROGRAM: u8 = 0x1A;
/// SRCNのノートオンのベロシティ値
pub const DSP_ADDRESS_SRCN_NOTEON_VELOCITY: u8 = 0x1B;
/// SRCNの中央に該当するノート（基準ピッチ）の上位8bit
pub const DSP_ADDRESS_SRCN_CENTER_NOTE_HIGH: u8 = 0x2A;
/// SRCNの中央に該当するノート（基準ピッチ）の下位8bit
pub const DSP_ADDRESS_SRCN_CENTER_NOTE_LOW: u8 = 0x2B;
/// SRCNのボリューム値（1bitフラグ + 7bit固定ボリューム値）
pub const DSP_ADDRESS_SRCN_VOLUME: u8 = 0x3A;
/// SRCNのパン値（1bitフラグ + 7bit固定パン値）
pub const DSP_ADDRESS_SRCN_PAN: u8 = 0x3B;
/// SRCNのピッチベンドセンシティビティ（1bitフラグ + 下位ビットで半音単位で幅を指定）
pub const DSP_ADDRESS_SRCN_PITCHBEND_SENSITIVITY: u8 = 0x4A;
/// SRCNのリバーブセンド（1bitフラグ + 下位ビットでセンド量を指定）
pub const DSP_ADDRESS_SRCN_REVERB_SEND: u8 = 0x4B;
/// SRCNのコーラスセンド（下位ビットでセンド量を指定）
pub const DSP_ADDRESS_SRCN_CHORUS_SEND: u8 = 0x5A;
/// SRCNの出力チャンネル MSSSDDDD
/// M: 送信元チャンネルのミュートフラグ
/// S: 送信元SPCチャンネル
/// D: 送信先MIDIチャンネル
pub const DSP_ADDRESS_SRCN_CHANNEL_ROUTING: u8 = 0x5B;
/// 全体設定フラグ VV000000
/// V: ボリュームカーブ（00: 平方根、01: 対数、10: 線形）
pub const DSP_ADDRESS_CONFIGURE_FLAG: u8 = 0x6B;
/// ノートオンフラグ
pub const DSP_ADDRESS_NOTEON: u8 = 0x7A;
/// エンベロープ・ボリューム・ピッチベンド更新間隔(ms)
pub const DSP_ADDRESS_PLAYBACK_PARAMETER_UPDATE_PERIOD: u8 = 0x6A;

/// ノートオン時のピッチベンド設定値
const NOTEON_PITCH_BEND: u16 = 8192;

/// ボリュームカーブ
#[derive(Copy, Clone, Debug)]
pub enum MIDIVolumeCurve {
    /// 平方根
    /// MIDIのボリューム値の2乗がSPCの振幅に比例するよう(GMの推奨値)に変換
    SquareRoot,
    /// 対数
    Log,
    /// 線形
    /// ゲインをそのままボリューム値に変換。ほとんどの場合音圧が小さくなるため非推奨だがデバッグに有効
    Linear,
}

/// ボイス再生ステータス
#[derive(Copy, Clone, Debug)]
struct VoicePlaybackStatus {
    /// 発声した音のノート番号
    note: u8,
    /// 発声した音源サンプル
    sample_source: usize,
    /// 発声したチャンネル
    channel: u8,
    /// ピッチベンドの基準ピッチ（最後に発声した音のピッチ）
    pitch_bend_base: u16,
}

/// チャンネル再生ステータス
#[derive(Copy, Clone, Debug)]
struct ChannelPlaybackStatus {
    /// ボリューム設定値
    volume: u8,
    /// パン設定値
    pan: u8,
    /// リバーブセンド
    reverb_send: u8,
    /// コーラスセンド
    chorus_send: u8,
    /// エクスプレッション
    expression: u8,
    /// ピッチベンド
    pitch_bend: u16,
    /// プログラム番号
    program: u8,
}

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
    sample_source: usize,
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
    /// ミュートフラグ
    ch_mute: bool,
    /// 再生中のステータス
    status: VoicePlaybackStatus,
}

/// 各サンプルに対応するMIDI再生パラメータ
struct SRCNMIDIParameter {
    /// ミュートするか
    mute: bool,
    /// プログラム番号（音色）
    program: u8,
    /// 基準ノート（ピッチ） 整数部7bit, 小数部9bit
    center_note: u16,
    /// ノートオンベロシティ
    noteon_velocity: u8,
    /// ピッチベンドセンシティビティ
    pitch_bend_sensitibity: u8,
    /// エンベロープ出力有効か
    output_envelope: bool,
    /// パンを自動更新するか
    auto_pan: bool,
    /// パン
    fixed_pan: u8,
    /// ボリュームを自動更新するか
    auto_volume: bool,
    /// ボリューム
    fixed_volume: u8,
    /// リバーブセンド
    fixed_reverb_send: u8,
    /// コーラスセンド
    chorus_send: u8,
    /// ピッチベンド出力有効か
    output_pitch_bend: bool,
    /// エコーをリバーブセンドとして出力するか
    echo_as_reverb_send: bool,
    /// ノートオン後に再生パラメータを更新するか
    update_parameter_after_noteon: bool,
    /// 出力先チャンネルルーティング 最上位ビットはミュートフラグ
    channel_routing: [u8; 8],
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
    /// 各チャンネルの再生状態
    channel_status: [ChannelPlaybackStatus; 16],
    /// 各サンプル番号に対応するマップ
    sample_source_map: [SRCNMIDIParameter; 256],
    /// 設定対象のサンプル番号
    sample_source_target: usize,
    /// エンベロープ・ボリューム・ピッチベンド更新間隔カウンタ
    playback_parameter_count: u16,
    /// エンベロープ・ボリューム・ピッチベンド更新間隔サンプル
    playback_parameter_update_period: u16,
    /// 最後に出力したチャンネルメッセージのステータスバイト
    status_byte: u8,
    /// ボリュームカーブ
    volume_curve: MIDIVolumeCurve,
    /// レジスタ値
    /// SDSPとして動いている際にレジスタが読みだされることがあるため保持
    dsp_register: [u8; 128],
}

/// ステータスバイト情報付きMIDIメッセージ
struct MIDIOutputWithStatusByte {
    /// MIDI出力メッセージ
    message: MIDIOutput,
    /// ステータスバイト
    status_byte: u8,
}

/// デフォルトのサンプル対応マップ
const DEFAULT_SRCN_MIDI_PARAMETER: SRCNMIDIParameter = SRCNMIDIParameter {
    mute: false,
    program: 0,
    center_note: 64 << 9, // 中心ノートは64で仮置き
    noteon_velocity: 0x7F,
    pitch_bend_sensitibity: 12,
    output_envelope: true,
    auto_pan: true,
    fixed_pan: 64,
    auto_volume: false,
    fixed_volume: 100,
    fixed_reverb_send: 40, // GM/GS/XGのよくある初期設定値
    chorus_send: 0,
    output_pitch_bend: true,
    echo_as_reverb_send: true,
    update_parameter_after_noteon: true,
    channel_routing: [0, 1, 2, 3, 4, 5, 6, 7], // SPCの出力チャンネルに合わせる
};

/// ピッチをMIDIノート番号に変換
fn pitch_to_note(center_note: u16, pitch: u16) -> u8 {
    // pitch(2^12を基準とする再生速度)から半音単位でのずれを計算
    // 例1）pitch = 2048 -> semitone = -12(-1 octave)
    // 例2）pitch = 4096 -> semitone =   0
    // 例3）pitch = 8192 -> semitone =  12(+1 octave)
    // 12 * log2(pitch / 4096) = 12 * (log2(pitch) - 12)
    let mut semitone = 12.0 * (libm::log2f(pitch as f32) - 12.0);
    // 基準ノート値（固定小数 7bit整数/9bit小数）を加算
    const NOTE_FRACTION_FACTOR: f32 = 1.0 / 512.0;
    semitone += center_note as f32 * NOTE_FRACTION_FACTOR;
    libm::roundf(semitone).clamp(0.0, 127.0) as u8
}

/// ゲイン[0,127]をMIDIのボリューム設定値に変換
fn gain_to_midi_volume(volume_curve: MIDIVolumeCurve, gain: f32) -> u8 {
    let volume = match volume_curve {
        MIDIVolumeCurve::SquareRoot => libm::sqrtf(gain * 127.0),
        MIDIVolumeCurve::Log => {
            const NORMALIZE_FACTOR: f32 = 59.89151875002212; // 126 / log10(127)
            if gain > 0.0 {
                NORMALIZE_FACTOR * libm::log10f(gain) + 1.0
            } else {
                0.0
            }
        }
        MIDIVolumeCurve::Linear => gain,
    };

    libm::roundf(volume).clamp(0.0, 127.0) as u8
}

/// LRボリュームをボリュームとパンの組に変換
/// LRボリュームは負値がありうるが、絶対値を取って前方パン・非負ボリュームに変換する
/// MIDIは前方のパンのみ考えるため
fn lrvolume_to_volume_and_pan(volume_curve: MIDIVolumeCurve, lrvolume: &[i8; 2]) -> (u8, u8) {
    let abs_lrvolume = [lrvolume[0].unsigned_abs(), lrvolume[1].unsigned_abs()];
    let volume = gain_to_midi_volume(volume_curve, abs_lrvolume[0].max(abs_lrvolume[1]) as f32);
    let pan = if abs_lrvolume[0] == abs_lrvolume[1] {
        64
    } else if abs_lrvolume[0] == 0 {
        127
    } else if abs_lrvolume[1] == 0 {
        0
    } else {
        const FACTOR: f32 = 256.0 / PI;
        libm::roundf(FACTOR * libm::atan2f(abs_lrvolume[1] as f32, abs_lrvolume[0] as f32)) as u8
    };
    (volume, pan)
}

/// エコーボリュームをリバーブセンドに変換
fn echovolume_to_reverb_send(echo_volume: &[i8; 2]) -> u8 {
    (echo_volume[0].unsigned_abs() as u16 + echo_volume[1].unsigned_abs() as u16) as u8 / 2
}

/// ピッチ・ピッチ基準値からピッチベンド設定値の計算
fn pitch_to_pitch_bend(pitch: u16, pitch_base: u16, sensitivity: u8) -> u16 {
    let max_semitone = sensitivity as f32;
    // [-max_semitone,max_semitone]半音を[-8192,8192]に対応付ける
    let pitchbend_ratio = libm::log2f((pitch as f32) / (pitch_base as f32)) * 12.0 / max_semitone;
    (libm::roundf((pitchbend_ratio * 8192.0).clamp(-8192.0, 8191.0)) as i16 + 8192) as u16
}

impl MIDIOutputWithStatusByte {
    /// チャンネルメッセージを追加
    fn push_channel_message(&mut self, mute: bool, data: &[u8]) {
        if mute {
            return;
        }
        assert!(data.len() <= 3);
        assert!(self.message.num_messages < MAX_NUM_MIDI_OUTPUT_MESSAGES);

        // 先頭1バイト（ステータスバイト）を見て直前と同じならばステータスバイトを省略（ランニングステータス）
        if self.status_byte == data[0] {
            self.message.messages[self.message.num_messages].data[0..(data.len() - 1)]
                .copy_from_slice(&data[1..data.len()]);
            self.message.messages[self.message.num_messages].length = data.len() - 1;
        } else {
            self.message.messages[self.message.num_messages].data[..data.len()]
                .copy_from_slice(&data);
            self.message.messages[self.message.num_messages].length = data.len();
            self.status_byte = data[0];
        }
        self.message.num_messages += 1;
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
            ch_mute: false,
            status: VoicePlaybackStatus {
                note: 0,
                sample_source: 0,
                channel: 0,
                pitch_bend_base: 0,
            },
        }
    }

    /// 32kHz定期処理
    fn tick(
        &mut self,
        echo_volume: u8,
        global_counter: u16,
        playback_parameter_update: bool,
        volume_curve: MIDIVolumeCurve,
        srn_map: &[SRCNMIDIParameter],
        channel_status: &mut [ChannelPlaybackStatus; 16],
        out: &mut MIDIOutputWithStatusByte,
    ) {
        // キーオンが入ったとき
        if self.keyon {
            self.keyon = false;
            // キーオフが漏れていた場合はノートオフを送信
            if self.noteon {
                let param = &srn_map[self.status.sample_source];
                let mute = self.ch_mute
                    || param.mute
                    || (param.channel_routing[self.channel as usize] & 0x80) != 0;
                out.push_channel_message(
                    mute,
                    &[MIDIMSG_NOTE_OFF | self.status.channel, self.status.note, 0],
                );
            }
            // エンベロープ設定
            self.eg.keyon();
            // ノートオン
            let param = &srn_map[self.sample_source];
            let program = param.program;
            let ch_routing = param.channel_routing[self.channel as usize];
            let channel = if program <= 0x7F {
                ch_routing & 0x7F
            } else {
                MIDI_PERCUSSION_CHANNEL
            };
            let mute = self.ch_mute || param.mute || (ch_routing & 0x80) != 0;
            let ch_status = &mut channel_status[channel as usize];
            if program <= 0x7F {
                // 音色が変わっていたらプログラムチェンジを送信
                if program != ch_status.program {
                    out.push_channel_message(mute, &[MIDIMSG_PROGRAM_CHANGE | channel, program]);
                    // ピッチベンドセンシティビティ設定
                    let first_byte = MIDIMSG_CONTROL_CHANGE | channel;
                    out.push_channel_message(mute, &[first_byte, MIDICC_RPN_MSB, 0x00]);
                    out.push_channel_message(mute, &[first_byte, MIDICC_RPN_LSB, 0x00]);
                    out.push_channel_message(
                        mute,
                        &[
                            first_byte,
                            MIDICC_RPN_DATA_ENTRY_LSB,
                            param.pitch_bend_sensitibity,
                        ],
                    );
                    out.push_channel_message(mute, &[first_byte, MIDICC_RPN_DATA_ENTRY_MSB, 0]);
                    ch_status.program = program;
                }
            }
            // ボリューム・パン
            let (volume, pan) = lrvolume_to_volume_and_pan(volume_curve, &self.volume);
            let noteon_volume = if param.auto_volume {
                volume
            } else {
                param.fixed_volume
            };
            if noteon_volume != ch_status.volume {
                out.push_channel_message(
                    mute,
                    &[
                        MIDIMSG_CONTROL_CHANGE | channel,
                        MIDICC_CHANNEL_VOLUME,
                        noteon_volume,
                    ],
                );
            }
            let noteon_pan = if param.auto_pan { pan } else { param.fixed_pan };
            if noteon_pan != ch_status.pan {
                out.push_channel_message(
                    mute,
                    &[MIDIMSG_CONTROL_CHANGE | channel, MIDICC_PANPOT, noteon_pan],
                );
            }
            // リバーブセンド
            let noteon_reverb_send = if param.echo_as_reverb_send {
                if self.echo {
                    echo_volume
                } else {
                    0
                }
            } else {
                param.fixed_reverb_send
            };
            if noteon_reverb_send != ch_status.reverb_send {
                out.push_channel_message(
                    mute,
                    &[
                        MIDIMSG_CONTROL_CHANGE | channel,
                        MIDICC_REVERB_DEPTH,
                        noteon_reverb_send,
                    ],
                );
            }
            // コーラスセンド
            let noteon_chorus_send = param.chorus_send;
            if noteon_chorus_send != ch_status.chorus_send {
                out.push_channel_message(
                    mute,
                    &[
                        MIDIMSG_CONTROL_CHANGE | channel,
                        MIDICC_CHORUS_DEPTH,
                        noteon_chorus_send,
                    ],
                );
            }
            // エクスプレッション
            let noteon_expression = if param.output_envelope {
                // ADSR使用かつAttack Rateが31だったらexpressionを最大とする（SPC700では2サンプルでゲインが最大に達するが、MIDIでそのような挙動は再現しにくいため）
                let adsr1 = self.eg.get_adsr1();
                if (adsr1 & 0x80 != 0) && (adsr1 & 0x0F == 0x0F) {
                    0x7F
                } else {
                    gain_to_midi_volume(volume_curve, self.eg.gain as f32 / 16.0)
                }
            } else {
                0x7F
            };
            if noteon_expression != ch_status.expression {
                out.push_channel_message(
                    mute,
                    &[
                        MIDIMSG_CONTROL_CHANGE | channel,
                        MIDICC_EXPRESSION,
                        noteon_expression,
                    ],
                );
            }
            // ピッチベンド（基準ピッチベンド値から変わっていれば）
            if NOTEON_PITCH_BEND != ch_status.pitch_bend {
                out.push_channel_message(
                    mute,
                    &[
                        MIDIMSG_PITCH_BEND | channel,
                        (NOTEON_PITCH_BEND & 0x7F) as u8, // LSB
                        ((NOTEON_PITCH_BEND >> 7) & 0x7F) as u8, // MSB
                    ],
                );
            }
            // ノートオン発行
            let note = if program < 0x80 {
                pitch_to_note(param.center_note, self.pitch)
            } else {
                program - 0x80
            };
            out.push_channel_message(
                mute,
                &[MIDIMSG_NOTE_ON | channel, note, param.noteon_velocity],
            );
            ch_status.volume = noteon_volume;
            ch_status.pan = noteon_pan;
            ch_status.reverb_send = noteon_reverb_send;
            ch_status.chorus_send = noteon_chorus_send;
            ch_status.expression = noteon_expression;
            ch_status.pitch_bend = NOTEON_PITCH_BEND;
            self.status.note = note;
            self.status.sample_source = self.sample_source;
            self.status.channel = channel;
            self.status.pitch_bend_base = self.pitch;
            self.noteon_drum = program >= 0x80;
            self.noteon = true;
        }

        // キーオフが入ったとき
        if self.keyoff {
            self.keyoff = false;
            // ノートオフ
            if self.noteon {
                let param = &srn_map[self.status.sample_source];
                let mute = self.ch_mute
                    || param.mute
                    || (param.channel_routing[self.channel as usize] & 0x80) != 0;
                out.push_channel_message(
                    mute,
                    &[MIDIMSG_NOTE_OFF | self.status.channel, self.status.note, 0],
                );
                self.noteon = false;
            }
        }

        // エンベロープ内部状態更新
        self.eg.update(global_counter);

        // 再生パラメータ更新（過剰に送ると遅延するので間引く）
        let param = &srn_map[self.status.sample_source];
        if self.noteon && param.update_parameter_after_noteon && playback_parameter_update {
            let ch_status = &mut channel_status[self.status.channel as usize];
            let mute = self.ch_mute
                || param.mute
                || (param.channel_routing[self.channel as usize] & 0x80) != 0;
            // エクスプレッション（エンベロープ）
            let expression = gain_to_midi_volume(volume_curve, self.eg.gain as f32 / 16.0);
            if ch_status.expression != expression && param.output_envelope {
                out.push_channel_message(
                    mute,
                    &[
                        MIDIMSG_CONTROL_CHANGE | self.status.channel,
                        MIDICC_EXPRESSION,
                        expression,
                    ],
                );
                ch_status.expression = expression;
            }
            // ボリューム・パン
            let (volume, pan) = lrvolume_to_volume_and_pan(volume_curve, &self.volume);
            if ch_status.volume != volume && param.auto_volume {
                out.push_channel_message(
                    mute,
                    &[
                        MIDIMSG_CONTROL_CHANGE | self.status.channel,
                        MIDICC_CHANNEL_VOLUME,
                        volume,
                    ],
                );
                ch_status.volume = volume;
            }
            if ch_status.pan != pan && param.auto_pan {
                out.push_channel_message(
                    mute,
                    &[
                        MIDIMSG_CONTROL_CHANGE | self.status.channel,
                        MIDICC_PANPOT,
                        pan,
                    ],
                );
                ch_status.pan = pan;
            }
            // ピッチベンド
            let pitch_bend = pitch_to_pitch_bend(
                self.pitch,
                self.status.pitch_bend_base,
                param.pitch_bend_sensitibity,
            );
            if ch_status.pitch_bend != pitch_bend && param.output_pitch_bend {
                // 7bitを2分割
                out.push_channel_message(
                    mute,
                    &[
                        MIDIMSG_PITCH_BEND | self.status.channel,
                        (pitch_bend & 0x7F) as u8,        // LSB
                        ((pitch_bend >> 7) & 0x7F) as u8, // MSB
                    ],
                );
                ch_status.pitch_bend = pitch_bend;
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
                MIDIVoiceRegister::new(0),
                MIDIVoiceRegister::new(1),
                MIDIVoiceRegister::new(2),
                MIDIVoiceRegister::new(3),
                MIDIVoiceRegister::new(4),
                MIDIVoiceRegister::new(5),
                MIDIVoiceRegister::new(6),
                MIDIVoiceRegister::new(7),
            ],
            channel_status: [ChannelPlaybackStatus {
                volume: 100, // GM/GS/XGのよくある初期設定値
                pan: 0,
                reverb_send: 40, // GM/GS/XGのよくある初期設定値
                chorus_send: 0,
                expression: 0,
                pitch_bend: NOTEON_PITCH_BEND,
                program: 0,
            }; 16],
            global_counter: 0,
            sample_source_map: [DEFAULT_SRCN_MIDI_PARAMETER; 256],
            sample_source_target: 0,
            playback_parameter_count: 0,
            playback_parameter_update_period: 160,
            status_byte: 0,
            volume_curve: MIDIVolumeCurve::SquareRoot,
            dsp_register: [0u8; 128],
        }
    }

    /// 128バイトメモリから初期化
    fn initialize(&mut self, ram: &mut [u8], dsp_register: &[u8; 128]) {
        // メンバ初期化
        *self = Self::new();
        self.playback_parameter_update_period = 160;
        self.volume_curve = MIDIVolumeCurve::SquareRoot;
        self.sample_source_map = [DEFAULT_SRCN_MIDI_PARAMETER; 256];

        // DIRは先に設定（初期状態でKONがある場合にアドレスを正しくするため）
        self.write_register(ram, DSP_ADDRESS_DIR, dsp_register[DSP_ADDRESS_DIR as usize]);

        // すべてのレジスタを設定
        for i in 0..128 {
            match i {
                DSP_ADDRESS_SRCN_TARGET
                | DSP_ADDRESS_SRCN_FLAG
                | DSP_ADDRESS_SRCN_PROGRAM
                | DSP_ADDRESS_SRCN_CENTER_NOTE_HIGH
                | DSP_ADDRESS_SRCN_CENTER_NOTE_LOW
                | DSP_ADDRESS_SRCN_VOLUME
                | DSP_ADDRESS_SRCN_PAN
                | DSP_ADDRESS_SRCN_NOTEON_VELOCITY
                | DSP_ADDRESS_SRCN_PITCHBEND_SENSITIVITY
                | DSP_ADDRESS_SRCN_REVERB_SEND
                | DSP_ADDRESS_SRCN_CHORUS_SEND
                | DSP_ADDRESS_SRCN_CHANNEL_ROUTING
                | DSP_ADDRESS_CONFIGURE_FLAG
                | DSP_ADDRESS_NOTEON
                | DSP_ADDRESS_CHANNEL_MUTE
                | DSP_ADDRESS_PLAYBACK_PARAMETER_UPDATE_PERIOD => {
                    // 独自追加アドレスに書き込まれると不定動作となるためメンバには書き込まない
                    // メモリには保持
                    self.dsp_register[i as usize] = dsp_register[i as usize];
                }
                _ => {
                    self.write_register(ram, i, dsp_register[i as usize]);
                }
            }
        }
    }

    /// DSPレジスタの書き込み処理
    fn write_register(&mut self, _ram: &[u8], address: u8, value: u8) {
        trace!("DSPW: {:02X} <- {:02X}", address, value);
        // 保持しているレジスタに書き込み
        self.dsp_register[address as usize] = value;
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
            DSP_ADDRESS_SRCN_TARGET => {
                self.sample_source_target = value as usize;
            }
            DSP_ADDRESS_SRCN_FLAG => {
                let param = &mut self.sample_source_map[self.sample_source_target];
                param.mute = (value & 0x80) != 0;
                param.output_envelope = (value & 0x40) != 0;
                param.update_parameter_after_noteon = (value & 0x20) != 0;
            }
            DSP_ADDRESS_SRCN_PROGRAM => {
                self.sample_source_map[self.sample_source_target].program = value;
            }
            DSP_ADDRESS_SRCN_CENTER_NOTE_HIGH => {
                let note = self.sample_source_map[self.sample_source_target].center_note;
                self.sample_source_map[self.sample_source_target].center_note =
                    ((value as u16) << 8) | (note & 0x00FF);
            }
            DSP_ADDRESS_SRCN_CENTER_NOTE_LOW => {
                let note = self.sample_source_map[self.sample_source_target].center_note;
                self.sample_source_map[self.sample_source_target].center_note =
                    ((value as u16) << 0) | (note & 0xFF00);
            }
            DSP_ADDRESS_SRCN_VOLUME => {
                let map = &mut self.sample_source_map[self.sample_source_target];
                map.auto_volume = (value & 0x80) != 0;
                map.fixed_volume = value & 0x7F;
            }
            DSP_ADDRESS_SRCN_PAN => {
                let map = &mut self.sample_source_map[self.sample_source_target];
                map.auto_pan = (value & 0x80) != 0;
                map.fixed_pan = value & 0x7F;
            }
            DSP_ADDRESS_SRCN_NOTEON_VELOCITY => {
                self.sample_source_map[self.sample_source_target].noteon_velocity = value;
            }
            DSP_ADDRESS_SRCN_PITCHBEND_SENSITIVITY => {
                let map = &mut self.sample_source_map[self.sample_source_target];
                map.output_pitch_bend = (value & 0x80) != 0;
                map.pitch_bend_sensitibity = value & 0x7F;
            }
            DSP_ADDRESS_SRCN_REVERB_SEND => {
                let map = &mut self.sample_source_map[self.sample_source_target];
                map.echo_as_reverb_send = (value & 0x80) != 0;
                map.fixed_reverb_send = value & 0x7F;
            }
            DSP_ADDRESS_SRCN_CHORUS_SEND => {
                let map = &mut self.sample_source_map[self.sample_source_target];
                map.chorus_send = value & 0x7F;
            }
            DSP_ADDRESS_SRCN_CHANNEL_ROUTING => {
                let ch_mute = value & 0x80;
                let src_ch = (value >> 4) & 0x7;
                let dst_ch = (value >> 0) & 0xF;
                self.sample_source_map[self.sample_source_target].channel_routing
                    [src_ch as usize] = ch_mute | dst_ch;
            }
            DSP_ADDRESS_CONFIGURE_FLAG => {
                self.volume_curve = match (value >> 6) & 0x3 {
                    0 => MIDIVolumeCurve::SquareRoot,
                    1 => MIDIVolumeCurve::Log,
                    2 => MIDIVolumeCurve::Linear,
                    _ => panic!("Invalid MIDI Curve Type!"),
                };
            }
            DSP_ADDRESS_NOTEON => {
                for ch in 0..8 {
                    self.voice[ch].noteon = ((value >> ch) & 0x1) != 0;
                }
            }
            DSP_ADDRESS_CHANNEL_MUTE => {
                for ch in 0..8 {
                    self.voice[ch].ch_mute = ((value >> ch) & 0x1) != 0;
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
                    }
                    DSP_ADDRESS_V0VOLR => {
                        self.voice[ch].volume[1] = value as i8;
                    }
                    DSP_ADDRESS_V0PITCHL => {
                        self.voice[ch].pitch = (self.voice[ch].pitch & 0xFF00) | (value as u16);
                    }
                    DSP_ADDRESS_V0PITCHH => {
                        self.voice[ch].pitch =
                            ((value as u16) << 8) | (self.voice[ch].pitch & 0x00FF);
                    }
                    DSP_ADDRESS_V0SRCN => {
                        self.voice[ch].sample_source = value as usize;
                    }
                    DSP_ADDRESS_V0ADSR1 => {
                        self.voice[ch].eg.set_adsr1(value);
                    }
                    DSP_ADDRESS_V0ADSR2 => {
                        self.voice[ch].eg.set_adsr2(value);
                    }
                    DSP_ADDRESS_V0GAIN => {
                        self.voice[ch].eg.set_gain(value);
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
            DSP_ADDRESS_ENDX => 0, // !! 0を返す !!
            DSP_ADDRESS_EFB => self.dsp_register[DSP_ADDRESS_EFB as usize], // 保持していた値を返す
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
            DSP_ADDRESS_ESA => self.dsp_register[DSP_ADDRESS_ESA as usize], // 保持していた値を返す
            DSP_ADDRESS_EDL => self.dsp_register[DSP_ADDRESS_EDL as usize], // 保持していた値を返す
            DSP_ADDRESS_FIR0 | DSP_ADDRESS_FIR1 | DSP_ADDRESS_FIR2 | DSP_ADDRESS_FIR3
            | DSP_ADDRESS_FIR4 | DSP_ADDRESS_FIR5 | DSP_ADDRESS_FIR6 | DSP_ADDRESS_FIR7 => {
                let index = address >> 4;
                // 保持していた値を返す
                self.dsp_register[(DSP_ADDRESS_FIR0 + index) as usize]
            }
            DSP_ADDRESS_SRCN_TARGET => self.sample_source_target as u8,
            DSP_ADDRESS_SRCN_FLAG => {
                let mut value = 0;
                let param = &self.sample_source_map[self.sample_source_target];
                if param.mute {
                    value |= 0x80;
                }
                if param.output_envelope {
                    value |= 0x40;
                }
                if param.update_parameter_after_noteon {
                    value |= 0x20;
                }
                value
            }
            DSP_ADDRESS_SRCN_PROGRAM => self.sample_source_map[self.sample_source_target].program,
            DSP_ADDRESS_SRCN_CENTER_NOTE_HIGH => {
                ((self.sample_source_map[self.sample_source_target].center_note >> 8) & 0xFF) as u8
            }
            DSP_ADDRESS_SRCN_CENTER_NOTE_LOW => {
                ((self.sample_source_map[self.sample_source_target].center_note >> 0) & 0xFF) as u8
            }
            DSP_ADDRESS_SRCN_VOLUME => {
                let mut value = self.sample_source_map[self.sample_source_target].fixed_volume;
                if self.sample_source_map[self.sample_source_target].auto_volume {
                    value |= 0x80;
                }
                value
            }
            DSP_ADDRESS_SRCN_PAN => {
                let mut value = self.sample_source_map[self.sample_source_target].fixed_pan;
                if self.sample_source_map[self.sample_source_target].auto_pan {
                    value |= 0x80;
                }
                value
            }
            DSP_ADDRESS_SRCN_NOTEON_VELOCITY => {
                self.sample_source_map[self.sample_source_target].noteon_velocity
            }
            DSP_ADDRESS_SRCN_PITCHBEND_SENSITIVITY => {
                let mut value =
                    self.sample_source_map[self.sample_source_target].pitch_bend_sensitibity;
                if self.sample_source_map[self.sample_source_target].output_pitch_bend {
                    value |= 0x80;
                }
                value
            }
            DSP_ADDRESS_SRCN_REVERB_SEND => {
                let mut value = self.sample_source_map[self.sample_source_target].fixed_reverb_send;
                if self.sample_source_map[self.sample_source_target].echo_as_reverb_send {
                    value |= 0x80;
                }
                value
            }
            DSP_ADDRESS_SRCN_CHORUS_SEND => {
                self.sample_source_map[self.sample_source_target].chorus_send
            }
            DSP_ADDRESS_SRCN_CHANNEL_ROUTING => {
                // 書き込み専用レジスタのため0を返す。どのチャンネルを設定したか不定のため
                0
            }
            DSP_ADDRESS_CONFIGURE_FLAG => {
                let mut ret = 0;
                ret |= match self.volume_curve {
                    MIDIVolumeCurve::SquareRoot => 0x00,
                    MIDIVolumeCurve::Log => 0x40,
                    MIDIVolumeCurve::Linear => 0x80,
                };
                ret
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
            DSP_ADDRESS_CHANNEL_MUTE => {
                let mut ret = 0;
                let mut bit = 1;
                for ch in 0..8 {
                    if self.voice[ch].ch_mute {
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
                    DSP_ADDRESS_V0SRCN => (self.voice[ch].sample_source & 0xFF) as u8,
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
        let mut out = MIDIOutputWithStatusByte {
            message: MIDIOutput {
                messages: [MIDIMessage {
                    data: [0; 3],
                    length: 0,
                }; MAX_NUM_MIDI_OUTPUT_MESSAGES],
                num_messages: 0,
            },
            status_byte: self.status_byte,
        };

        // エンベロープ・ボリューム・ピッチベンド更新するか
        let playback_parameter_update = if self.playback_parameter_update_period <= 1 {
            true
        } else {
            if self.playback_parameter_count >= self.playback_parameter_update_period {
                self.playback_parameter_count = 1;
                true
            } else {
                self.playback_parameter_count += 1;
                false
            }
        };

        // 全チャンネルの周期処理を実行
        let reverb_depth = echovolume_to_reverb_send(&self.echo_volume);
        for ch in 0..8 {
            self.voice[ch].tick(
                reverb_depth,
                self.global_counter,
                playback_parameter_update,
                self.volume_curve,
                &self.sample_source_map,
                &mut self.channel_status,
                &mut out,
            );
        }
        // グローバルカウンタの更新
        update_global_counter(&mut self.global_counter);

        // ステータスバイト更新
        self.status_byte = out.status_byte;

        // ミュートならばメッセージなし
        if self.mute || out.message.num_messages == 0 {
            None
        } else {
            Some(out.message)
        }
    }
}
