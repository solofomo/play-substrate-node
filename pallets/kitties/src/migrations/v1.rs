use crate::{Config, Kitties};
use frame_support::{
	migration::storage_key_iter, pallet_prelude::*, weights::Weight, StoragePrefixedMap,
};

use super::VERSION;
pub type KittyId = u32;
pub type KittyDna = [u8; 16];
pub type KittyName = [u8; 13];

#[derive(Clone, PartialEq, Eq, Default, TypeInfo, Encode, Decode, MaxEncodedLen, RuntimeDebug)]
pub struct Kitty {
	pub name: KittyName,
	pub dna: KittyDna,
}

pub fn upgrade_v1<T: Config>(current_version: StorageVersion) -> Weight {
	if current_version != VERSION {
		return Weight::zero()
	}

	let module = Kitties::<T>::module_prefix();
	let items = Kitties::<T>::storage_prefix();

	for (index, kitty) in
		storage_key_iter::<KittyId, Kitty, Blake2_128Concat>(module, items).drain()
	{
		let _name = *b"v1_ketty_id_1";
		let new_kitty = Kitty { name: _name, dna: kitty.dna };
	}
	Weight::zero()
}