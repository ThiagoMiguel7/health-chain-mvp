#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// Configuração do Pallet Reader.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type WeightInfo: WeightInfo;
    }

    /// Interface de pesos (placeholder por enquanto)
    pub trait WeightInfo {
        fn read_record() -> Weight { Weight::zero() }
    }
    
    // Implementação padrão para ()
    impl WeightInfo for () {}

    #[pallet::event]
    pub enum Event<T: Config> {
        // Eventos futuros de leitura (se necessário)
    }

    #[pallet::error]
    pub enum Error<T> {
        RecordNotFound,
        AccessDenied,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        // Aqui virão as funções de leitura nas próximas issues (#11 e #12)
    }
}