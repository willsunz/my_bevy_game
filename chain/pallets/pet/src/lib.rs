#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The runtime event
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// The maximum length of a metadata string.
		#[pallet::constant]
		type StringLimit: Get<u32>;
	}


	#[derive(
		Clone, Encode, Decode, PartialEqNoBound, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen,
	)]
	#[scale_info(skip_type_params(T))]
	pub struct Pet<T: Config> {
		pub owner: T::AccountId,
		pub name: BoundedVec<u8, T::StringLimit>,
		pub species: Species,
	}

	#[derive(
		Clone, Encode, Decode, PartialEqNoBound, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen,
	)]
	pub enum Species {
		Snake,
		Turtle,
		Rabbit,
	}

	#[pallet::storage]
	pub type Pets<T: Config> = StorageMap<_, Blake2_128Concat, u32, Pet<T>>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		PetMinted(T::AccountId, u32),
	}

	#[pallet::error]
	pub enum Error<T> {
		PetAlreadyExists
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn mint_pet(
			origin: OriginFor<T>,
			id: u32,
			name: BoundedVec<u8, T::StringLimit>,
			species: Species,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			ensure!(!Pets::<T>::contains_key(id), Error::<T>::PetAlreadyExists);

			let pet = Pet { owner: sender.clone(), name, species };
			Pets::<T>::insert(id, pet);

			Self::deposit_event(Event::PetMinted(sender, id));
			Ok(().into())
		}

	}
}
