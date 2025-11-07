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

#[derive(Copy, Clone, Debug)]
enum SPCVoiceGainMode {
    Fixed { gain: u8 },
    LinearDecrease { rate: u8 },
    ExponentialDecrease { rate: u8 },
    LinearIncrease { rate: u8 },
    BentIncrease { rate: u8 },
}

#[derive(Copy, Clone, Debug)]
struct SPCVoiceRegister {
    volume: [i8; 2],
    pitch: u16,
    sample_source: u8, // BRR dir = sample_dir_page * 0x100 + sample_source * 4
    adsr_enable: bool,
    attack_rate: u8,
    decay_rate: u8,
    sustain_rate: u8,
    sustain_level: u8,
    gain_mode: SPCVoiceGainMode,
    envelope_value: u8,
    output_sample: i8,
    keyon: bool,
    keyoff: bool,
    end: bool,
    pitch_mod: bool,
    noise: bool,
}

/// S-DSP
pub struct SPCDSP {
    volume: [i8; 2],
    echo_volume: [i8; 2],
    flag: u8,
    echo_feedback: i8,
    echo: [bool; 8],
    sample_dir_page: u8,
    echo_start_page: u8,
    echo_delay: u8,
    fir_coef: [i8; 8],
    voice: [SPCVoiceRegister; 8],
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
            end: false,
            pitch_mod: false,
            noise: false,
        }
    }

    fn compute_sample(&mut self, ram: &[u8]) -> [i16; 2] {
        let mut out = [0i16; 2];
        let address = (self.sample_source << 2) as usize;
        println!("Voice {:?} {:?}", self, ram[address..(address + 9)].to_vec());
        out
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
            sample_dir_page: 0,
            echo_start_page: 0,
            echo_delay: 0,
            fir_coef: [0; 8],
            voice: [SPCVoiceRegister::new(); 8],
        }
    }

    /// DSPレジスタの書き込み処理
    pub fn write_dsp_register(&mut self, address: u8, value: u8) {
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
                for id in 0..8 {
                    self.voice[id].keyon = ((value >> id) & 0x1) != 0;
                }
            }
            KOFF_ADDRESS => {
                for id in 0..8 {
                    self.voice[id].keyoff = ((value >> id) & 0x1) != 0;
                }
            }
            FLG_ADDRESS => {
                self.flag = value;
            }
            ENDX_ADDRESS => {
                for id in 0..8 {
                    self.voice[id].end = ((value >> id) & 0x1) != 0;
                }
            }
            EFB_ADDRESS => {
                self.echo_feedback = value as i8;
            }
            PMON_ADDRESS => {
                for id in 1..8 {
                    /* NOTE! 0は無効 */
                    self.voice[id].pitch_mod = ((value >> id) & 0x1) != 0;
                }
            }
            NON_ADDRESS => {
                for id in 0..8 {
                    self.voice[id].noise = ((value >> id) & 0x1) != 0;
                }
            }
            EON_ADDRESS => {
                for id in 0..8 {
                    self.echo[id] = ((value >> id) & 0x1) != 0;
                }
            }
            DIR_ADDRESS => {
                self.sample_dir_page = value;
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
                let id = (address >> 4) as usize;
                match address & 0xF {
                    V0VOLL_ADDRESS => {
                        self.voice[id].volume[0] = value as i8;
                    }
                    V0VOLR_ADDRESS => {
                        self.voice[id].volume[1] = value as i8;
                    }
                    V0PITCHL_ADDRESS => {
                        self.voice[id].pitch = (self.voice[id].pitch & 0xFF00) | (value as u16);
                    }
                    V0PITCHH_ADDRESS => {
                        self.voice[id].pitch =
                            ((value as u16) << 8) | (self.voice[id].pitch & 0x00FF);
                    }
                    V0SRCN_ADDRESS => {
                        self.voice[id].sample_source = value;
                    }
                    V0ADSR1_ADDRESS => {
                        self.voice[id].adsr_enable = (value >> 7) != 0;
                        self.voice[id].attack_rate = value & 0xF;
                        self.voice[id].decay_rate = (value >> 4) & 0x7;
                    }
                    V0ADSR2_ADDRESS => {
                        self.voice[id].sustain_rate = value & 0x1F;
                        self.voice[id].sustain_level = (value >> 5) & 0x7;
                    }
                    V0GAIN_ADDRESS => {
                        if (value >> 7) == 0 {
                            self.voice[id].gain_mode =
                                SPCVoiceGainMode::Fixed { gain: value & 0x7F };
                        } else {
                            let rate = value & 0x1F;
                            self.voice[id].gain_mode = match (value >> 5) & 0x3 {
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
                        self.voice[id].envelope_value = value;
                    }
                    V0OUTX_ADDRESS => {
                        // 書き込めるけど意味はない（読み取り用レジスタ）
                        self.voice[id].output_sample = value as i8;
                    }
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

    /// DSPレジスタの読み込み処理
    pub fn read_dsp_register(&self, address: u8) -> u8 {
        match address {
            MVOLL_ADDRESS => self.volume[0] as u8,
            MVOLR_ADDRESS => self.volume[1] as u8,
            EVOLL_ADDRESS => self.echo_volume[0] as u8,
            EVOLR_ADDRESS => self.echo_volume[1] as u8,
            KON_ADDRESS => {
                let mut ret = 0;
                let mut bit = 1;
                for id in 0..8 {
                    if self.voice[id].keyon {
                        ret |= bit;
                    }
                    bit <<= 1;
                }
                ret
            }
            KOFF_ADDRESS => {
                let mut ret = 0;
                let mut bit = 1;
                for id in 0..8 {
                    if self.voice[id].keyoff {
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
                for id in 0..8 {
                    if self.voice[id].end {
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
                for id in 1..8 {
                    /* NOTE! id==0は常に無効 */
                    if self.voice[id].pitch_mod {
                        ret |= bit;
                    }
                    bit <<= 1;
                }
                ret
            }
            NON_ADDRESS => {
                let mut ret = 0;
                let mut bit = 1;
                for id in 0..8 {
                    if self.voice[id].noise {
                        ret |= bit;
                    }
                    bit <<= 1;
                }
                ret
            }
            EON_ADDRESS => {
                let mut ret = 0;
                let mut bit = 1;
                for id in 0..8 {
                    if self.echo[id] {
                        ret |= bit;
                    }
                    bit <<= 1;
                }
                ret
            }
            DIR_ADDRESS => self.sample_dir_page,
            ESA_ADDRESS => self.echo_start_page,
            EDL_ADDRESS => self.echo_delay,
            FIR0_ADDRESS | FIR1_ADDRESS | FIR2_ADDRESS | FIR3_ADDRESS | FIR4_ADDRESS
            | FIR5_ADDRESS | FIR6_ADDRESS | FIR7_ADDRESS => {
                let index = address >> 4;
                self.fir_coef[index as usize] as u8
            }
            address if ((address & 0xF) <= 0x9) => {
                let id = (address >> 4) as usize;
                match address & 0xF {
                    V0VOLL_ADDRESS => self.voice[id].volume[0] as u8,
                    V0VOLR_ADDRESS => self.voice[id].volume[1] as u8,
                    V0PITCHL_ADDRESS => (self.voice[id].pitch & 0xFF) as u8,
                    V0PITCHH_ADDRESS => ((self.voice[id].pitch >> 8) & 0xFF) as u8,
                    V0SRCN_ADDRESS => self.voice[id].sample_source,
                    V0ADSR1_ADDRESS => {
                        let adsr_flag = if self.voice[id].adsr_enable {
                            0x80
                        } else {
                            0x00
                        };
                        adsr_flag | (self.voice[id].decay_rate << 4) | self.voice[id].attack_rate
                    }
                    V0ADSR2_ADDRESS => {
                        (self.voice[id].sustain_level << 5) | self.voice[id].sustain_rate
                    }
                    V0GAIN_ADDRESS => match self.voice[id].gain_mode {
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
                    V0ENVX_ADDRESS => self.voice[id].envelope_value,
                    V0OUTX_ADDRESS => self.voice[id].output_sample as u8,
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
        let mut out = [0i16; 2];
        // 全チャンネルの出力をミックス
        for ch in 0..8 {
            let vout =
                self.voice[ch].compute_sample(&ram[((self.sample_dir_page as usize) << 8)..]);
            out[0] += vout[0];
            out[1] += vout[1];
        }
        out
    }
}
