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
	pub trait Config: frame_system::Config + pallet_timestamp::Config {
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
		// pub experience_points: u32,
		// pub hungry_points: u32,
		// pub energy_points: u32,
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

	/// hungry_points is determined by last feed time.
	/// if last feed is time passed 3 days, allow others to save the pet by feeding it.
	/// if the owner didn't feed the pet for 6 days, the saver can own the cat without the agree of
	/// its original owner.
	#[pallet::storage]
	pub type LastFeedTime<T: Config> =
		StorageDoubleMap<_, Blake2_128Concat, u32, Blake2_128Concat, T::AccountId, T::Moment>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		PetMinted(T::AccountId, u32),
		PetFeeded(T::AccountId, u32),
		PetSaved(T::AccountId, u32),
		PetForceSaved(T::AccountId, u32),
	}

	#[pallet::error]
	pub enum Error<T> {
		PetAlreadyExists,
		PetNotExists,
		NotPetOwner,
		NotAvailableToSave,
		NotAvailableToForceSave,
		OwnerShouldFeedPet,
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

			let pet = Pet {
				owner: sender.clone(),
				name,
				species,
				// experience_points: 1,
				// hungry_points: 0,
				// energy_points: 10,
			};
			Pets::<T>::insert(id, pet);
			LastFeedTime::<T>::insert(id, &sender, <pallet_timestamp::Pallet<T>>::get());

			Self::deposit_event(Event::PetMinted(sender, id));
			Ok(().into())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(0)]
		pub fn feed_pet(origin: OriginFor<T>, id: u32) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			let pet = Pets::<T>::get(id).ok_or(Error::<T>::PetNotExists)?;
			ensure!(pet.owner == sender, Error::<T>::NotPetOwner);

			LastFeedTime::<T>::insert(id, &sender, <pallet_timestamp::Pallet<T>>::get());

			Self::deposit_event(Event::PetFeeded(sender, id));
			Ok(().into())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(0)]
		pub fn save_pet(origin: OriginFor<T>, id: u32) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			let pet = Pets::<T>::get(id).ok_or(Error::<T>::PetNotExists)?;
			ensure!(pet.owner != sender, Error::<T>::OwnerShouldFeedPet);

			let now = <pallet_timestamp::Pallet<T>>::get();
			let can_save = LastFeedTime::<T>::iter_prefix(id).into_iter().all(|(account, last_feed_time)| {
				(now > last_feed_time + (259_200_000 as u32).into()) // 3 days since last feed
					|| sender == account
			});

			if !can_save {
				return Err(Error::<T>::NotAvailableToSave.into())
			}

			LastFeedTime::<T>::insert(id, &sender, now);

			Self::deposit_event(Event::PetSaved(sender, id));
			Ok(().into())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(0)]
		pub fn save_pet_by_force(origin: OriginFor<T>, id: u32) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			let mut pet = Pets::<T>::get(id).ok_or(Error::<T>::PetNotExists)?;
			let previous_pet_owner = pet.owner.clone();
			ensure!(previous_pet_owner != sender, Error::<T>::OwnerShouldFeedPet);

			let now = <pallet_timestamp::Pallet<T>>::get();
			let can_force_save = LastFeedTime::<T>::iter_prefix(id).into_iter().all(|(account, last_feed_time)| {
				(previous_pet_owner == account && now > last_feed_time + (518_400_000 as u32).into()) // 6 days since last feed
					|| (sender == account && now < last_feed_time + (86_400_000 as u32).into()) // 1 day since last feed
			});

			if !can_force_save {
				return Err(Error::<T>::NotAvailableToForceSave.into())
			}

			pet.owner = sender.clone();
			Pets::<T>::insert(id, pet);
			LastFeedTime::<T>::remove(id, previous_pet_owner);

			Self::deposit_event(Event::PetForceSaved(sender, id));
			Ok(().into())
		}
	}
}
