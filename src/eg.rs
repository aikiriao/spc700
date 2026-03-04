/// ボイスゲインとそのパラメータ
#[derive(Copy, Clone, Debug)]
enum GainMode {
    /// 固定ゲイン
    Fixed { gain: u8 },
    /// 線形増加
    LinearDecrease { rate: u8 },
    /// 指数的減衰
    ExponentialDecrease { rate: u8 },
    /// 線形減衰
    LinearIncrease { rate: u8 },
    /// ベンド増加
    BentIncrease { rate: u8 },
}

/// エンベロープの状態
#[derive(Copy, Clone, Debug, PartialEq)]
enum EnvelopeState {
    /// アタック
    Attack,
    /// ディケイ
    Decay,
    /// サステイン
    Sustain,
    /// リリース
    Release,
}

/// エンベロープジェネレータ
#[derive(Copy, Clone, Debug)]
pub struct EnvelopeGenerator {
    /// エンベロープ更新間隔
    rate: u8,
    /// ADSR有効か否か
    adsr_enable: bool,
    /// ゲインモード
    gain_mode: GainMode,
    /// ゲイン設定値
    gain_value: u8,
    /// エンベロープ状態
    state: EnvelopeState,
    /// アタック状態の更新サンプル間隔
    attack_rate: u8,
    /// ディケイ状態の更新サンプル間隔
    decay_rate: u8,
    /// サステイン状態の更新サンプル間隔
    sustain_rate: u8,
    /// サステイン状態に移行するゲイン値
    sustain_level: u8,
    /// 最後に計算したゲイン適用値
    pub gain: i32,
}

/// グローバルカウンタのマスク
/// Anomie's S-DSP Docから引用
const GLOBAL_COUNTER_MASKS: [u16; 32] = [
    0x0000, 0xFFE0, 0x3FF8, 0x1FE7, 0x7FE0, 0x1FF8, 0x0FE7, 0x3FE0, 0x0FF8, 0x07E7, 0x1FE0, 0x07F8,
    0x03E7, 0x0FE0, 0x03F8, 0x01E7, 0x07E0, 0x01F8, 0x00E7, 0x03E0, 0x00F8, 0x0067, 0x01E0, 0x0078,
    0x0027, 0x00E0, 0x0038, 0x0007, 0x0060, 0x0018, 0x0020, 0x0000,
];

/// グローバルカウンタのXORマスク
/// Anomie's S-DSP Docから引用
const GLOBAL_COUNTER_XORS: [u16; 32] = [
    0xFFFF, 0x0000, 0x3E08, 0x1D04, 0x0000, 0x1E08, 0x0D04, 0x0000, 0x0E08, 0x0504, 0x0000, 0x0608,
    0x0104, 0x0000, 0x0208, 0x0104, 0x0000, 0x0008, 0x0004, 0x0000, 0x0008, 0x0004, 0x0000, 0x0008,
    0x0004, 0x0000, 0x0008, 0x0004, 0x0000, 0x0008, 0x0000, 0x0000,
];

impl EnvelopeGenerator {
    pub fn new() -> Self {
        Self {
            rate: 0,
            adsr_enable: false,
            gain_mode: GainMode::Fixed { gain: 0 },
            gain_value: 0,
            state: EnvelopeState::Release,
            attack_rate: 0,
            decay_rate: 0,
            sustain_rate: 0,
            sustain_level: 0,
            gain: 0,
        }
    }

    /// キーオン時の処理
    pub fn keyon(&mut self) {
        self.state = EnvelopeState::Attack;
        if self.adsr_enable {
            self.gain = 0;
            self.rate = self.attack_rate;
        } else {
            match self.gain_mode {
                GainMode::Fixed { gain } => {
                    self.gain = (gain as i32) << 4;
                    self.rate = 0;
                }
                GainMode::LinearDecrease { rate }
                | GainMode::ExponentialDecrease { rate }
                | GainMode::LinearIncrease { rate }
                | GainMode::BentIncrease { rate } => {
                    self.rate = rate;
                }
            }
        }
    }

    /// キーオフ時の処理
    pub fn keyoff(&mut self) {
        self.state = EnvelopeState::Release;
        self.rate = 31; // 毎サンプル更新
    }

    /// 即時ミュート
    pub fn mute(&mut self) {
        self.state = EnvelopeState::Release;
        self.gain = 0;
    }

    /// ADSR1の設定処理
    pub fn set_adsr1(&mut self, value: u8) {
        self.adsr_enable = (value & 0x80) != 0;
        self.attack_rate = 2 * (value & 0xF) + 1;
        self.decay_rate = 2 * ((value >> 4) & 0x7) + 16;
        // 動作中のADSRのレート更新
        if self.adsr_enable {
            match self.state {
                EnvelopeState::Attack => {
                    self.rate = self.attack_rate;
                }
                EnvelopeState::Decay => {
                    self.rate = self.decay_rate;
                }
                _ => {}
            }
        } else {
            // ADSRが無効になった場合にレートを書き換え
            match self.gain_mode {
                GainMode::LinearDecrease { rate }
                | GainMode::ExponentialDecrease { rate }
                | GainMode::LinearIncrease { rate }
                | GainMode::BentIncrease { rate } => {
                    self.rate = rate;
                }
                _ => {}
            }
        }
    }

    /// ADSR2の設定処理
    pub fn set_adsr2(&mut self, value: u8) {
        self.sustain_rate = value & 0x1F;
        if self.adsr_enable {
            self.sustain_level = (value >> 5) & 0x7;
        } else {
            // ADSRが無効のときは V0GAIN_ADDRESS の上位3bit
            self.sustain_level = (self.gain_value >> 5) & 0x7;
        }
        // 動作中のADSRのレート更新
        if self.adsr_enable {
            match self.state {
                EnvelopeState::Sustain => {
                    self.rate = self.sustain_rate;
                }
                _ => {}
            }
        }
    }

    /// GAINの設定処理
    pub fn set_gain(&mut self, value: u8) {
        if (value & 0x80) == 0 {
            self.gain_mode = GainMode::Fixed { gain: value & 0x7F };
        } else {
            self.gain_mode = match (value >> 5) & 0x3 {
                0 => GainMode::LinearDecrease { rate: value & 0x1F },
                1 => GainMode::ExponentialDecrease { rate: value & 0x1F },
                2 => GainMode::LinearIncrease { rate: value & 0x1F },
                3 => GainMode::BentIncrease { rate: value & 0x1F },
                _ => unreachable!("Unsupported Gain Type!"),
            };
        }
        // ADSRが無効であれば即時反映
        if self.state != EnvelopeState::Release && !self.adsr_enable {
            match self.gain_mode {
                GainMode::Fixed { gain } => {
                    self.gain = (gain as i32) << 4;
                }
                GainMode::LinearDecrease { rate }
                | GainMode::ExponentialDecrease { rate }
                | GainMode::LinearIncrease { rate }
                | GainMode::BentIncrease { rate } => {
                    self.rate = rate;
                }
            }
        }
        // sustain_levelの設定で参照するため設定値を保持
        self.gain_value = value;
    }

    /// ADSR1の取得処理
    pub fn get_adsr1(&self) -> u8 {
        let adsr_flag = if self.adsr_enable { 0x80 } else { 0x00 };
        adsr_flag | (self.decay_rate << 4) | self.attack_rate
    }

    /// ADSR2の取得処理
    pub fn get_adsr2(&self) -> u8 {
        (self.sustain_level << 5) | self.sustain_rate
    }

    /// GAINの取得処理
    pub fn get_gain(&self) -> u8 {
        self.gain_value
    }

    /// エンベロープ状態更新
    pub fn update(&mut self, global_counter: u16) -> bool {
        // アクション発生判定
        let updated = (global_counter & GLOBAL_COUNTER_MASKS[self.rate as usize])
            ^ GLOBAL_COUNTER_XORS[self.rate as usize]
            == 0;
        if updated {
            // エンベロープゲイン更新
            if self.state == EnvelopeState::Release {
                // Release状態時はADSR有効無効にかかわらずゲインを下げる
                self.gain -= 8;
            } else {
                if self.adsr_enable {
                    match self.state {
                        EnvelopeState::Attack => {
                            if self.attack_rate == 31 {
                                self.gain += 1024;
                            } else {
                                // rate = aaaa1のLinear increaseと同じ
                                self.gain += 32;
                            }
                        }
                        EnvelopeState::Decay => {
                            // rate = 1ddd0のExp. decreaseと同じ
                            let diff = ((self.gain - 1) >> 8) + 1;
                            self.gain -= diff;
                        }
                        EnvelopeState::Sustain => {
                            // rate = rrrrrのExp. decreaseと同じ
                            let diff = ((self.gain - 1) >> 8) + 1;
                            self.gain -= diff;
                        }
                        _ => unreachable!("Release state MUST already processd"),
                    }
                } else {
                    match self.gain_mode {
                        GainMode::Fixed { gain } => {
                            self.gain = (gain as i32) << 4;
                        }
                        GainMode::LinearDecrease { .. } => {
                            self.gain -= 32;
                        }
                        GainMode::ExponentialDecrease { .. } => {
                            let diff = ((self.gain - 1) >> 8) + 1;
                            self.gain -= diff;
                        }
                        GainMode::LinearIncrease { .. } => {
                            self.gain += 32;
                        }
                        GainMode::BentIncrease { .. } => {
                            self.gain += if self.gain < 0x600 { 32 } else { 8 };
                        }
                    }
                }
            }

            // エンベロープ状態更新（これはエンベロープの有効無効に関係なく実行）
            // ゲインは範囲制限前の値を使用
            match self.state {
                EnvelopeState::Attack => {
                    if self.gain >= 0x7E0 {
                        self.state = EnvelopeState::Decay;
                        self.rate = self.decay_rate;
                    }
                }
                EnvelopeState::Decay => {
                    if ((self.gain >> 8) & 0x7) <= (self.sustain_level as i32) {
                        self.state = EnvelopeState::Sustain;
                        self.rate = self.sustain_rate;
                    }
                }
                EnvelopeState::Sustain | EnvelopeState::Release => {}
            }

            // ゲインの範囲制限
            self.gain = self.gain.clamp(0, 0x7FF);
        }
        updated
    }
}
