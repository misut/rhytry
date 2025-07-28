enum Datten {
    Fuchi,
    Men,
}

enum Uchite {
    Hidari,
    Migi,
    Ryou,
}

struct Tataki {
    beat: Decimal,
    datten: Datten,
    uchite: Uchite,
}

#[derive(Debug, PartialEq)]
pub enum Hantei {
    Ryou,
    Ka,
    Fuka,
    Renda,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Onpu {
    Don(Fraction),
    Kat(Fraction),
    Renda(Fraction, Fraction),
    OokiDon(Fraction),
    OokiKat(Fraction),
    OokiRenda(Fraction, Fraction),
}

pub trait Beat {
    fn at(&self) -> f32;

    fn diff(&self, other: &dyn Beat) -> f32 {
        (self.at() - other.at()).abs()
    }
}

pub struct Decimal(f32);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Fraction {
    pub denominator: u32,
    pub numerator: u32,
}

impl Beat for Decimal {
    fn at(&self) -> f32 {
        self.0
    }
}

impl Beat for Fraction {
    fn at(&self) -> f32 {
        self.numerator as f32 / self.denominator as f32
    }
}

const JUDGE_THRESHOLD: f32 = 0.1;

trait Judge {
    fn judge(&self, tataki: &Tataki) -> Option<Hantei>;
}

impl Judge for Onpu {
    fn judge(&self, tataki: &Tataki) -> Option<Hantei> {
        match self {
            Onpu::Don(beat) | Onpu::OokiDon(beat) => {
                let diff: f32 = beat.diff(&tataki.beat);
                match tataki.datten {
                    Datten::Fuchi => {
                        if diff > JUDGE_THRESHOLD {
                            None
                        } else {
                            Some(Hantei::Fuka)
                        }
                    },
                    Datten::Men => {
                        if diff > JUDGE_THRESHOLD {
                            None
                        } else if diff > JUDGE_THRESHOLD / 2.0 {
                            Some(Hantei::Fuka)
                        } else if diff > JUDGE_THRESHOLD / 4.0 {
                            Some(Hantei::Ka)
                        } else {
                            Some(Hantei::Ryou)
                        }
                    }
                }
            },
            Onpu::Kat(beat) | Onpu::OokiKat(beat) => {
                let diff: f32 = beat.diff(&tataki.beat);
                match tataki.datten {
                    Datten::Fuchi => {
                        if diff > JUDGE_THRESHOLD {
                            None
                        } else if diff > JUDGE_THRESHOLD / 2.0 {
                            Some(Hantei::Fuka)
                        } else if diff > JUDGE_THRESHOLD / 4.0 {
                            Some(Hantei::Ka)
                        } else {
                            Some(Hantei::Ryou)
                        }
                    },
                    Datten::Men => {
                        if diff > JUDGE_THRESHOLD {
                            None
                        } else {
                            Some(Hantei::Fuka)
                        }
                    }
                }
            },
            Onpu::Renda(beg, end) | Onpu::OokiRenda(beg, end) =>
                if tataki.beat.at() < beg.at() {
                    None
                } else if tataki.beat.at() < end.at() {
                    Some(Hantei::Renda)
                } else {
                    None
                }
        }
    }
}

pub struct Shousetsu(pub Vec<Onpu>);

pub struct Fumen(pub Vec<Shousetsu>);

#[derive(Debug, Default, Clone, Copy)]
pub struct TaikoState {
    pub beat_index: f32,
    pub shousetsu_index: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_be_none() {
        // Given
        let onpu_and_tataki: Vec<(Onpu, Tataki)> = vec![
            (Onpu::Don(Fraction { denominator: 4, numerator: 1 }), Tataki { beat: Decimal(0.), datten: Datten::Men, uchite: Uchite::Hidari }),  // Early don
            (Onpu::Don(Fraction { denominator: 4, numerator: 1 }), Tataki { beat: Decimal(0.5), datten: Datten::Men, uchite: Uchite::Hidari }),  // Late don
            (Onpu::Kat(Fraction { denominator: 4, numerator: 1 }), Tataki { beat: Decimal(0.), datten: Datten::Fuchi, uchite: Uchite::Hidari }),  // Early kat
            (Onpu::Kat(Fraction { denominator: 4, numerator: 1 }), Tataki { beat: Decimal(0.5), datten: Datten::Fuchi, uchite: Uchite::Hidari }),  // Late kat
            (Onpu::Renda(Fraction { denominator: 4, numerator: 1 }, Fraction { denominator: 4, numerator: 2 }), Tataki { beat: Decimal(0.), datten: Datten::Fuchi, uchite: Uchite::Hidari }),  // Early renda
            (Onpu::Renda(Fraction { denominator: 4, numerator: 1 }, Fraction { denominator: 4, numerator: 2 }), Tataki { beat: Decimal(0.5), datten: Datten::Fuchi, uchite: Uchite::Hidari }),  // Late renda
        ];

        // When
        let results: Vec<Option<Hantei>> = onpu_and_tataki.iter().map(|(onpu, tataki)| onpu.judge(tataki)).collect();

        // Then
        assert!(results.iter().all(|result| result.is_none()));
    }

    #[test]
    fn should_be_fuka() {
        // Given
        let onpu_and_tataki: Vec<(Onpu, Tataki)> = vec![
            (Onpu::Don(Fraction { denominator: 4, numerator: 1 }), Tataki { beat: Decimal(0.15), datten: Datten::Men, uchite: Uchite::Hidari }),  // Early don
            (Onpu::Don(Fraction { denominator: 4, numerator: 1 }), Tataki { beat: Decimal(0.25), datten: Datten::Fuchi, uchite: Uchite::Hidari }),  // Fuchi when don
            (Onpu::Don(Fraction { denominator: 4, numerator: 1 }), Tataki { beat: Decimal(0.35), datten: Datten::Men, uchite: Uchite::Hidari }),  // Late don
            (Onpu::Kat(Fraction { denominator: 4, numerator: 1 }), Tataki { beat: Decimal(0.15), datten: Datten::Fuchi, uchite: Uchite::Hidari }),  // Early kat
            (Onpu::Kat(Fraction { denominator: 4, numerator: 1 }), Tataki { beat: Decimal(0.25), datten: Datten::Men, uchite: Uchite::Hidari }),  // Men when kat
            (Onpu::Kat(Fraction { denominator: 4, numerator: 1 }), Tataki { beat: Decimal(0.35), datten: Datten::Fuchi, uchite: Uchite::Hidari }),  // Late kat
        ];

        // When
        let results: Vec<Option<Hantei>> = onpu_and_tataki.iter().map(|(onpu, tataki)| onpu.judge(tataki)).collect();

        // Then
        assert!(results.iter().all(|result| result == &Some(Hantei::Fuka)));
    }

    #[test]
    fn should_be_ka() {
        // Given
        let onpu_and_tataki: Vec<(Onpu, Tataki)> = vec![
            (Onpu::Don(Fraction { denominator: 4, numerator: 1 }), Tataki { beat: Decimal(0.2), datten: Datten::Men, uchite: Uchite::Hidari }),  // Early don
            (Onpu::Don(Fraction { denominator: 4, numerator: 1 }), Tataki { beat: Decimal(0.2999), datten: Datten::Men, uchite: Uchite::Hidari }),  // Late don
            (Onpu::Kat(Fraction { denominator: 4, numerator: 1 }), Tataki { beat: Decimal(0.2), datten: Datten::Fuchi, uchite: Uchite::Hidari }),  // Early kat
            (Onpu::Kat(Fraction { denominator: 4, numerator: 1 }), Tataki { beat: Decimal(0.2999), datten: Datten::Fuchi, uchite: Uchite::Hidari }),  // Late kat
        ];

        // When
        let results: Vec<Option<Hantei>> = onpu_and_tataki.iter().map(|(onpu, tataki)| onpu.judge(tataki)).collect();

        // Then
        assert!(results.iter().all(|result| result == &Some(Hantei::Ka)));
    }

    #[test]
    fn should_be_ryou() {
        // Given
        let onpu_and_tataki: Vec<(Onpu, Tataki)> = vec![
            (Onpu::Don(Fraction { denominator: 4, numerator: 1 }), Tataki { beat: Decimal(0.25), datten: Datten::Men, uchite: Uchite::Hidari }),
            (Onpu::OokiDon(Fraction { denominator: 4, numerator: 1 }), Tataki { beat: Decimal(0.25), datten: Datten::Men, uchite: Uchite::Hidari }),
            (Onpu::Kat(Fraction { denominator: 4, numerator: 1 }), Tataki { beat: Decimal(0.25), datten: Datten::Fuchi, uchite: Uchite::Hidari }),
            (Onpu::OokiKat(Fraction { denominator: 4, numerator: 1 }), Tataki { beat: Decimal(0.25), datten: Datten::Fuchi, uchite: Uchite::Hidari }),
        ];

        // When
        let results: Vec<Option<Hantei>> = onpu_and_tataki.iter().map(|(onpu, tataki)| onpu.judge(tataki)).collect();

        // Then
        assert!(results.iter().all(|result| result == &Some(Hantei::Ryou)));
    }

    #[test]
    fn should_be_renda() {
        // Given
        let onpu_and_tataki: Vec<(Onpu, Tataki)> = vec![
            (Onpu::Renda(Fraction { denominator: 4, numerator: 1 }, Fraction { denominator: 4, numerator: 2 }), Tataki { beat: Decimal(0.25), datten: Datten::Fuchi, uchite: Uchite::Hidari }),
            (Onpu::OokiRenda(Fraction { denominator: 4, numerator: 1 }, Fraction { denominator: 4, numerator: 2 }), Tataki { beat: Decimal(0.25), datten: Datten::Fuchi, uchite: Uchite::Hidari }),
        ];

        // When
        let results: Vec<Option<Hantei>> = onpu_and_tataki.iter().map(|(onpu, tataki)| onpu.judge(tataki)).collect();

        // Then
        assert!(results.iter().all(|result| result == &Some(Hantei::Renda)));
    }
}
