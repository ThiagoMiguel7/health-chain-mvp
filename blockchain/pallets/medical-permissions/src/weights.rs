#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

pub trait WeightInfo {
    fn create_record() -> Weight { Weight::from_parts(10_000, 0) }
    // Adicione outras funções se necessário, ou deixe genérico
}

impl WeightInfo for () {}