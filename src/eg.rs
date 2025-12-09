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
        let updated = (self.rate > 0) && ((global_counter + COUNTER_OFFSETS[self.rate as usize]) % COUNTER_RATES[self.rate as usize] == 0);
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
