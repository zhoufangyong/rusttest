//test case
//step 1 remove default
//step 2 lib.rs add mod mock/tests

use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};
//测试用例在子模块，引入父级
use super::*;

/* step 1 remove default

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::do_something(Origin::signed(1), 42));
		// Read pallet storage and assert an expected result.
		assert_eq!(TemplateModule::something(), Some(42));
	});
}

#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_noop!(
			TemplateModule::cause_error(Origin::signed(1)),
			Error::<Test>::NoneValue
		);
	});
}

*/

//step 3 add test case
/*    
//为lib.rs 创建测试用例

*/		


//------------------------------------------------------------------------------

#[test]
fn create_works() {
    new_test_ext().execute_with(|| {
        assert_ok!(KittiesModule::create(Origin::signed(1)));
        
        assert!(Kitties::<Test>::contains_key(0));
        assert_eq!(Owner::<Test>::get(0), Some(1));
        assert_eq!(KittiesCount::<Test>::get(), Some(1));
        System::assert_has_event(mock::Event::KittiesModule(Event::KittyCreate(1,0)));
    }); 
}

#[test]
fn create_failed() {
    new_test_ext().execute_with(|| {
        assert_noop!(KittiesModule::create(Origin::signed(4)), Error::<Test>::InsufficientBalance);
        KittiesCount::<Test>::put(u32::max_value());
        assert_noop!(
            KittiesModule::create(Origin::signed(1)),
            Error::<Test>::KittiesCountOverflow 
        );
    }); 
}

#[test]
fn transfer_works() {
    new_test_ext().execute_with(|| {
        assert_ok!(KittiesModule::create(Origin::signed(1)));
        assert_ok!(KittiesModule::transfer(Origin::signed(1), 2, 0));
        assert!(Kitties::<Test>::contains_key(0));
        assert_eq!(Owner::<Test>::get(0), Some(2));
        assert_eq!(KittiesCount::<Test>::get(), Some(1));
        System::assert_has_event(mock::Event::KittiesModule(Event::KittyTransfer(1, 2, 0)));
    })
}

#[test]
fn transfer_failed() {
    new_test_ext().execute_with(|| {
        assert_ok!(KittiesModule::create(Origin::signed(1)));
        assert_noop!(KittiesModule::transfer(Origin::signed(2), 3, 0), Error::<Test>::NotKittyOwner);        
    }) 
}

#[test]
fn breed_works() {
    new_test_ext().execute_with(|| {
        assert_ok!(KittiesModule::create(Origin::signed(1)));
        assert_ok!(KittiesModule::create(Origin::signed(1)));
        assert_ok!(KittiesModule::breed(Origin::signed(1), 0, 1));
        assert!(Kitties::<Test>::contains_key(0));
        assert_eq!(Owner::<Test>::get(0), Some(1));
        assert_eq!(Owner::<Test>::get(1), Some(1));
        assert_eq!(Owner::<Test>::get(2), Some(1));
        assert_eq!(KittiesCount::<Test>::get(), Some(3));
        System::assert_has_event(mock::Event::KittiesModule(Event::KittyCreate(1,2)));
    }) 
}

#[test]
fn breed_failed() {
    new_test_ext().execute_with(|| {
        assert_ok!(KittiesModule::create(Origin::signed(1)));
        assert_noop!(KittiesModule::breed(Origin::signed(1), 0, 0), Error::<Test>::SameParentIndex);
        assert_noop!(KittiesModule::breed(Origin::signed(1), 0, 1), Error::<Test>::InvalidKittyIndex);
        assert_noop!(KittiesModule::breed(Origin::signed(1), 1, 0), Error::<Test>::InvalidKittyIndex);

        assert_ok!(KittiesModule::create(Origin::signed(1)));
        KittiesCount::<Test>::put(u32::max_value());
        assert_noop!(
            KittiesModule::breed(Origin::signed(1), 0, 1),
            Error::<Test>::KittiesCountOverflow 
        );
    }) 
}

#[test]
fn sell_kitty_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		assert_ok!(KittiesModule::sell_kitty(Origin::signed(1), 0, Some(666)));
        assert!(Kitties::<Test>::contains_key(0));
        assert_eq!(Owner::<Test>::get(0), Some(1));
        assert_eq!(Price::<Test>::get(0), Some(666));
        assert_eq!(KittiesCount::<Test>::get(), Some(1));
        System::assert_has_event(mock::Event::KittiesModule(Event::KittySale(1, 0, Some(666))));
	})
}

#[test]
fn sell_kitty_failed() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		assert_noop!(KittiesModule::sell_kitty(Origin::signed(2), 0, Some(666)), Error::<Test>::NotKittyOwner);
	})
}

#[test]
fn buy_kitty_works() {
    new_test_ext().execute_with(|| {
        assert_ok!(KittiesModule::create(Origin::signed(1)));
        assert_ok!(KittiesModule::sell_kitty(Origin::signed(1), 0, Some(666)));
        assert_eq!(Price::<Test>::contains_key(0), true);
        assert_eq!(Price::<Test>::get(0), Some(666));
        assert_ok!(KittiesModule::buy_kitty(Origin::signed(2), 0));
        assert_eq!(Price::<Test>::contains_key(0), false);
        assert!(Kitties::<Test>::contains_key(0));
        assert_eq!(Owner::<Test>::get(0), Some(2));
        assert_eq!(KittiesCount::<Test>::get(), Some(1));
        System::assert_has_event(mock::Event::KittiesModule(Event::KittyTransfer(1, 2, 0)));
    }) 
}

#[test]
fn buy_kitty_failed() {
    new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		assert_noop!(KittiesModule::buy_kitty(Origin::signed(2), 1), Error::<Test>::InvalidKittyIndex);
		assert_noop!(KittiesModule::buy_kitty(Origin::signed(1), 0), Error::<Test>::BuyFromSelf);
		assert_noop!(KittiesModule::buy_kitty(Origin::signed(2), 0), Error::<Test>::KittyNotForSale);
		assert_ok!(KittiesModule::sell_kitty(Origin::signed(1), 0, Some(666)));
		assert_noop!(KittiesModule::buy_kitty(Origin::signed(4), 0), Error::<Test>::InsufficientBalance);
    }) 
}