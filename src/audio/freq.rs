pub const SAMPLE_RATE: u32 = 44_100;

#[derive(Copy, Clone)]
pub enum Pitch {
    C,
    Cs,
    D,
    Ds,
    E,
    F,
    Fs,
    G,
    Gs,
    A,
    As,
    B,
}

impl TryFrom<char> for Pitch {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Self::A),
            'B' => Ok(Self::B),
            'C' => Ok(Self::C),
            'D' => Ok(Self::D),
            'E' => Ok(Self::E),
            'F' => Ok(Self::F),
            'G' => Ok(Self::G),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Freq(pub(super) f32);

impl Freq {
    pub const ZERO: Self = Self(0.);

    // https://www.liutaiomottola.com/formulae/freqtab.htm

    /// C0, MIDI note #12
    pub const C0: Self = Self(16.351);
    pub const CS0: Self = Self(17.324);
    pub const D0: Self = Self(18.354);
    pub const DS0: Self = Self(19.445);
    pub const E0: Self = Self(20.601);
    pub const F0: Self = Self(21.827);
    pub const FS0: Self = Self(23.124);
    pub const G0: Self = Self(24.499);
    pub const GS0: Self = Self(25.956);
    /// A0, the lowest note of a piano
    pub const A0: Self = Self(27.5);
    pub const AS0: Self = Self(29.135);
    /// B0, the lowest note of a 5 string bass
    pub const B0: Self = Self(30.868);
    /// C1, the lowest note of double bass with C extension
    pub const C1: Self = Self(32.703);
    pub const CS1: Self = Self(34.648);
    pub const D1: Self = Self(36.708);
    pub const DS1: Self = Self(38.891);
    /// E1, the lowest note of a bass
    pub const E1: Self = Self(41.203);
    pub const F1: Self = Self(43.654);
    pub const FS1: Self = Self(46.249);
    pub const G1: Self = Self(48.999);
    pub const GS1: Self = Self(51.913);
    pub const A1: Self = Self(55.);
    pub const AS1: Self = Self(58.27);
    pub const B1: Self = Self(61.735);
    pub const C2: Self = Self(65.406);
    pub const CS2: Self = Self(69.296);
    pub const D2: Self = Self(73.416);
    pub const DS2: Self = Self(77.782);
    /// E2, the lowest note of a guitar.
    pub const E2: Self = Self(82.407);
    pub const F2: Self = Self(87.307);
    pub const FS2: Self = Self(92.499);
    pub const G2: Self = Self(97.999);
    pub const GS2: Self = Self(103.826);
    pub const A2: Self = Self(110.);
    pub const AS2: Self = Self(116.541);
    pub const B2: Self = Self(123.471);
    pub const C3: Self = Self(130.813);
    pub const CS3: Self = Self(138.591);
    pub const D3: Self = Self(146.832);
    pub const DS3: Self = Self(155.563);
    pub const E3: Self = Self(164.814);
    pub const F3: Self = Self(174.614);
    pub const FS3: Self = Self(184.997);
    /// G3, the lowest note of a violin.
    pub const G3: Self = Self(195.998);
    pub const GS3: Self = Self(207.652);
    pub const A3: Self = Self(220.);
    pub const AS3: Self = Self(233.082);
    pub const B3: Self = Self(246.942);
    /// C4, the "middle C".
    pub const C4: Self = Self(261.626);
    pub const CS4: Self = Self(277.183);
    pub const D4: Self = Self(293.665);
    pub const DS4: Self = Self(311.127);
    pub const E4: Self = Self(329.628);
    pub const F4: Self = Self(349.228);
    pub const FS4: Self = Self(369.994);
    pub const G4: Self = Self(391.995);
    pub const GS4: Self = Self(415.305);
    /// A4, the tuning reference note.
    pub const A4: Self = Self(440.);
    pub const AS4: Self = Self(466.164);
    pub const B4: Self = Self(493.883);
    pub const C5: Self = Self(523.251);
    pub const CS5: Self = Self(554.365);
    pub const D5: Self = Self(587.33);
    pub const DS5: Self = Self(622.254);
    pub const E5: Self = Self(659.255);
    pub const F5: Self = Self(698.456);
    pub const FS5: Self = Self(739.989);
    pub const G5: Self = Self(783.991);
    pub const GS5: Self = Self(830.609);
    pub const A5: Self = Self(880.);
    pub const AS5: Self = Self(932.328);
    pub const B5: Self = Self(987.767);
    pub const C6: Self = Self(1046.502);
    pub const CS6: Self = Self(1108.731);
    pub const D6: Self = Self(1174.659);
    pub const DS6: Self = Self(1244.508);
    pub const E6: Self = Self(1318.51);
    pub const F6: Self = Self(1396.913);
    pub const FS6: Self = Self(1479.978);
    pub const G6: Self = Self(1567.982);
    pub const GS6: Self = Self(1661.219);
    pub const A6: Self = Self(1760.);
    pub const AS6: Self = Self(1864.655);
    pub const B6: Self = Self(1975.533);
    pub const C7: Self = Self(2093.005);
    pub const CS7: Self = Self(2217.461);
    pub const D7: Self = Self(2349.318);
    pub const DS7: Self = Self(2489.016);
    pub const E7: Self = Self(2637.021);
    pub const F7: Self = Self(2793.826);
    pub const FS7: Self = Self(2959.955);
    pub const G7: Self = Self(3135.964);
    pub const GS7: Self = Self(3322.438);
    pub const A7: Self = Self(3520.);
    pub const AS7: Self = Self(3729.31);
    pub const B7: Self = Self(3951.066);
    /// C8, the highest note of a piano.
    pub const C8: Self = Self(4186.009);
    pub const CS8: Self = Self(4434.922);
    pub const D8: Self = Self(4698.636);
    pub const DS8: Self = Self(4978.032);
    pub const E8: Self = Self(5274.042);
    pub const F8: Self = Self(5587.652);
    pub const FS8: Self = Self(5919.91);
    pub const G8: Self = Self(6271.928);
    pub const GS8: Self = Self(6644.876);
    pub const A8: Self = Self(7040.);
    pub const AS8: Self = Self(7458.62);
    pub const B8: Self = Self(7902.132);
    pub const C9: Self = Self(8372.018);
    pub const CS9: Self = Self(8869.844);
    pub const D9: Self = Self(9397.272);
    pub const DS9: Self = Self(9956.064);
    pub const E9: Self = Self(10548.084);
    pub const F9: Self = Self(11175.304);
    pub const FS9: Self = Self(11839.82);
    pub const G9: Self = Self(12543.856);
    /// G#9, MIDI note #128, the top of the MIDI tuning range.
    pub const GS9: Self = Self(13289.752);
    pub const A9: Self = Self(14080.);
    pub const AS9: Self = Self(14917.24);
    /// B9. For most of adults, it is already beyond the hearing range.
    pub const B9: Self = Self(15804.264);

    #[must_use]
    pub fn hz(hz: f32) -> Self {
        Self(hz)
    }

    #[must_use]
    #[expect(clippy::cast_precision_loss)]
    pub fn midi(note: u8) -> Self {
        // https://inspiredacoustics.com/en/MIDI_note_numbers_and_center_frequencies
        // https://en.wikipedia.org/wiki/Musical_note#MIDI
        let mut f: f32 = match note % 12 {
            0 => 8.1758,
            1 => 8.66,
            2 => 9.18,
            3 => 9.72,
            4 => 10.30,
            5 => 10.91,
            6 => 11.56,
            7 => 12.25,
            8 => 12.98,
            9 => 13.75,
            10 => 14.57,
            _ => 15.43,
        };
        let oct = note / 12;
        f *= (1 << oct) as f32;
        Self(f)
    }

    #[must_use]
    #[expect(clippy::cast_precision_loss)]
    pub fn note(pitch: Pitch, octave: u8) -> Self {
        // https://github.com/crbulakites/hum/blob/master/src/hum_process/hum_math.rs
        // https://en.wikipedia.org/wiki/Musical_note#Pitch_frequency_in_hertz
        let mut f: f32 = match pitch {
            Pitch::C => 16.351,
            Pitch::Cs => 17.324,
            Pitch::D => 18.354,
            Pitch::Ds => 19.445,
            Pitch::E => 20.601,
            Pitch::F => 21.827,
            Pitch::Fs => 23.124,
            Pitch::G => 24.499,
            Pitch::Gs => 25.956,
            Pitch::A => 27.5,
            Pitch::As => 29.135,
            Pitch::B => 30.868,
        };
        f *= (1 << octave) as f32;
        Freq(f)
    }
}

impl From<f32> for Freq {
    fn from(value: f32) -> Self {
        Self(value)
    }
}
