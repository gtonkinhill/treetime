use crate::make_error;
use color_eyre::{Section, SectionExt};
use eyre::{eyre, Report, WrapErr};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Nuc {
  T,
  A,
  W,
  C,
  Y,
  M,
  H,
  G,
  K,
  R,
  D,
  S,
  B,
  V,
  N,

  #[serde(rename = "-")]
  Gap,
}

impl ToString for Nuc {
  fn to_string(&self) -> String {
    String::from(from_nuc(*self))
  }
}

impl Nuc {
  #[inline]
  pub const fn is_acgt(self) -> bool {
    matches!(self, Nuc::A | Nuc::C | Nuc::G | Nuc::T)
  }

  #[inline]
  pub const fn is_acgtn(self) -> bool {
    matches!(self, Nuc::A | Nuc::C | Nuc::G | Nuc::T | Nuc::N)
  }
}

impl Default for Nuc {
  fn default() -> Self {
    Nuc::Gap
  }
}

#[inline]
pub fn to_nuc(letter: char) -> Result<Nuc, Report> {
  match letter {
    'T' => Ok(Nuc::T),
    'A' => Ok(Nuc::A),
    'W' => Ok(Nuc::W),
    'C' => Ok(Nuc::C),
    'Y' => Ok(Nuc::Y),
    'M' => Ok(Nuc::M),
    'H' => Ok(Nuc::H),
    'G' => Ok(Nuc::G),
    'K' => Ok(Nuc::K),
    'R' => Ok(Nuc::R),
    'D' => Ok(Nuc::D),
    'S' => Ok(Nuc::S),
    'B' => Ok(Nuc::B),
    'V' => Ok(Nuc::V),
    'N' => Ok(Nuc::N),
    '-' => Ok(Nuc::Gap),
    _ => make_error!("Unknown nucleotide: {letter}"),
  }
}

#[inline]
pub const fn from_nuc(nuc: Nuc) -> char {
  match nuc {
    Nuc::T => 'T',
    Nuc::A => 'A',
    Nuc::W => 'W',
    Nuc::C => 'C',
    Nuc::Y => 'Y',
    Nuc::M => 'M',
    Nuc::H => 'H',
    Nuc::G => 'G',
    Nuc::K => 'K',
    Nuc::R => 'R',
    Nuc::D => 'D',
    Nuc::S => 'S',
    Nuc::B => 'B',
    Nuc::V => 'V',
    Nuc::N => 'N',
    Nuc::Gap => '-',
  }
}

pub fn to_nuc_seq(str: &str) -> Result<Vec<Nuc>, Report> {
  str.chars().map(to_nuc).collect()
}

pub fn from_nuc_seq(seq: &[Nuc]) -> String {
  seq.iter().map(|nuc| from_nuc(*nuc)).collect()
}
