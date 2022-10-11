use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_for_create() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		assert_eq!(KittiesModule::next_kitty_id(), 1);

		assert_noop!(KittiesModule::transfer(Origin::signed(2), 0, 1), Error::<Test>::NotOwner);
	});
}

#[test]
fn it_fails_for_create_when_token_not_enough() {
	new_test_ext().execute_with(|| {
		assert_noop!(KittiesModule::create(Origin::signed(3)), Error::<Test>::TokenNotEnough);
	});
}

#[test]
fn it_fails_for_create_when_exceeding_max_kitty_owned() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		assert_ok!(KittiesModule::create(Origin::signed(1)));

		assert_noop!(KittiesModule::create(Origin::signed(1)), Error::<Test>::ExceedMaxKittyOwned);
	});
}

#[test]
fn it_works_for_breed() {
	new_test_ext().execute_with(|| {
		let _ = KittiesModule::create(Origin::signed(1));
		let _ = KittiesModule::create(Origin::signed(1));

		assert_ok!(KittiesModule::breed(Origin::signed(1), 0, 1));
	});
}

#[test]
fn it_fails_for_breed_when_exceeding_max_kitty_owned() {
	new_test_ext().execute_with(|| {
		let _ = KittiesModule::create(Origin::signed(1));
		let _ = KittiesModule::create(Origin::signed(1));
		let _ = KittiesModule::create(Origin::signed(1));
		assert_noop!(
			KittiesModule::breed(Origin::signed(1), 0, 1),
			Error::<Test>::ExceedMaxKittyOwned
		);
	});
}

#[test]
fn it_fails_for_breed_when_invalid_kitty_id() {
	new_test_ext().execute_with(|| {
		let _ = KittiesModule::create(Origin::signed(1));
		let _ = KittiesModule::create(Origin::signed(1));
		assert_noop!(KittiesModule::breed(Origin::signed(1), 0, 2), Error::<Test>::InvalidKittyId);
		assert_noop!(KittiesModule::breed(Origin::signed(1), 2, 0), Error::<Test>::InvalidKittyId);
		assert_noop!(KittiesModule::breed(Origin::signed(1), 2, 1), Error::<Test>::InvalidKittyId);
		assert_noop!(KittiesModule::breed(Origin::signed(1), 2, 3), Error::<Test>::InvalidKittyId);
	});
}

#[test]
fn it_fails_for_breed_when_same_kitty_id() {
	new_test_ext().execute_with(|| {
		let _ = KittiesModule::create(Origin::signed(1));
		let _ = KittiesModule::create(Origin::signed(1));
		assert_noop!(KittiesModule::breed(Origin::signed(1), 0, 0), Error::<Test>::SameKittyId);
	});
}

#[test]
fn it_works_for_transfer() {
	new_test_ext().execute_with(|| {
		let _ = KittiesModule::create(Origin::signed(1));
		assert_ok!(KittiesModule::transfer(Origin::signed(1), 0, 2));
	});
}

#[test]
fn it_fails_for_transfer_when_kitty_not_exists() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			KittiesModule::transfer(Origin::signed(1), 0, 2),
			Error::<Test>::InvalidKittyId
		);
	});
}

#[test]
fn it_fails_for_transfer_when_not_owner() {
	new_test_ext().execute_with(|| {
		let _ = KittiesModule::create(Origin::signed(1));
		assert_noop!(KittiesModule::transfer(Origin::signed(2), 0, 3), Error::<Test>::NotOwner);
	});
}

#[test]
fn it_fails_for_transfer_when_exceeding_max_kitty_owned() {
	new_test_ext().execute_with(|| {
		let _ = KittiesModule::create(Origin::signed(1));
		let _ = KittiesModule::create(Origin::signed(2));
		let _ = KittiesModule::create(Origin::signed(2));
		let _ = KittiesModule::create(Origin::signed(2));
		assert_noop!(
			KittiesModule::transfer(Origin::signed(1), 0, 2),
			Error::<Test>::ExceedMaxKittyOwned
		);
	});
}

#[test]
fn it_fails_for_transfer_when_receiver_not_exists() {
	new_test_ext().execute_with(|| {
		let _ = KittiesModule::create(Origin::signed(1));
		assert_noop!(
			KittiesModule::transfer(Origin::signed(1), 0, 4),
			Error::<Test>::ReceiverNotExist
		);
	});
}
