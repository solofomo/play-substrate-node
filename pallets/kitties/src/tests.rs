use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_for_create() {
    new_test_ext().execute_with(||{
        let kitty_id = 0;
        let account_id = 1;

        assert_eq!(KittiesModule::next_kitty_id(), kitty_id);
        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));

        // 检查是否存在创建Kitty的事件
        System::assert_last_event(RuntimeEvent::KittiesModule(Event::KittyCreated{
			who: account_id,
			kitty_id: kitty_id,
			kitty: KittiesModule::kitties(kitty_id).unwrap(),
		}));

		assert_eq!(KittiesModule::next_kitty_id(), kitty_id + 1);
		assert_eq!(KittiesModule::kitties(kitty_id).is_some(), true);
		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));
		assert_eq!(KittiesModule::kitty_parents(kitty_id), None);

		crate::NextKittyId::<Test>::set(crate::KittyId::max_value());
		assert_noop!(
			KittiesModule::create(RuntimeOrigin::signed(account_id)),
			Error::<Test>::InvalidKittyId
		);
    })
}

#[test]
fn it_works_for_transfer() {
	new_test_ext().execute_with(|| {
		let account_id: u64 = 1;
		let kitty_id = 0u32;
		let to = 2;

		// System::set_block_number(1);
	    assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));
		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));

		assert_noop!(
			KittiesModule::transfer(RuntimeOrigin::signed(to), account_id, kitty_id),
			Error::<Test>::NotOwner
		);
		assert_ok!(KittiesModule::transfer(RuntimeOrigin::signed(account_id), to, kitty_id));
		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(to));
		assert_ok!(KittiesModule::transfer(RuntimeOrigin::signed(to), account_id, kitty_id));
		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));

		System::assert_last_event(RuntimeEvent::KittiesModule(Event::KittyTransferred {
			who: to, 
			kitty_id: kitty_id,
			to: account_id,
		}));
	});
}

#[test]
fn it_works_for_breed() {
	new_test_ext().execute_with(|| {
		let account_id: u64 = 1;

		let kitty_id = 0u32;

		assert_noop!(
			KittiesModule::breed(RuntimeOrigin::signed(account_id), kitty_id, kitty_id),
			Error::<Test>::SameKittyId
		);

		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));
		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));

		assert_eq!(KittiesModule::next_kitty_id(), kitty_id + 2);

		assert_ok!(
			KittiesModule::breed(RuntimeOrigin::signed(account_id), kitty_id, kitty_id + 1)
		);

		System::assert_last_event(RuntimeEvent::KittiesModule(Event::KittyBreed {
			who: account_id, 
			kitty_id: kitty_id + 2, // 新的繁殖出来的Kitty的ID应该是kitty_id + 2
			kitty: KittiesModule::kitties(kitty_id + 2).unwrap(),
		}));

		let breed_kitty_id = 2;
		assert_eq!(KittiesModule::next_kitty_id(), breed_kitty_id + 1);
		assert_eq!(KittiesModule::kitties(breed_kitty_id).is_some(), true);
		assert_eq!(KittiesModule::kitty_owner(breed_kitty_id), Some(account_id));
		assert_eq!(
			KittiesModule::kitty_parents(breed_kitty_id),
			Some((kitty_id, kitty_id + 1))
		);

	});
}