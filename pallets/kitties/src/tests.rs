use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_for_create() {
    new_test_ext().execute_with(||{
        let kitty_id = 0;
        let account_id = 1;

        assert_eq!(KittiesModule::next_kitty_id(), kitty_id);
        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));
		assert_noop!(KittiesModule::transfer(RuntimeOrigin::signed(account_id), 0, 1), Error::<Test>::InvalidKittyId);
    })
}

#[test]
fn it_works_for_transfer() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		System::set_block_number(1);
	    assert_ok!(KittiesModule::create(RuntimeOrigin::signed(1)));
		assert_ok!(KittiesModule::transfer(RuntimeOrigin::signed(1), 1, 0));
		assert_eq!(KittiesModule::next_kitty_id(), 1);
	});
}

#[test]
fn it_works_for_breed() {
	new_test_ext().execute_with(|| {
		let account_id: u64 = 1;

		let kitty_id_1 = 0u32;
		let kitty_id_2 = 1u32;
		let kitty_id_3: u32 = 2u32;

		// 创建Kitty
		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));
		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));
		// 繁殖
		assert_ok!(KittiesModule::breed(RuntimeOrigin::signed(account_id), kitty_id_1, kitty_id_2));
		// 检查拥有者
		assert_eq!(KittiesModule::kitty_owner(kitty_id_3), Some(account_id));
	});
}