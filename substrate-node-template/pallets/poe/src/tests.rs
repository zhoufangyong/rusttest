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
//为lib.rs 创建存证create_claim()创建测试用例

*/		


//------------------------------------------------------------------------------

#[test]//测试用例
fn create_claim_works() {

	//mock中的测试帮助环境
	//new_test_ext().execute_with(execute:||{
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
        assert_eq!(Proofs::<Test>::get(&claim), Some((1, frame_system::Pallet::<Test>::block_number())));
    })
}
#[test]
fn create_claim_failed_when_claim_already_exist() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

        assert_noop!(
            PoeModule::create_claim(Origin::signed(1), claim),
            Error::<Test>::ProofAlreadyClaimed
        );
    })
}

#[test]
fn create_claim_failed_when_claim_is_too_long() {
	new_test_ext().execute_with(|| {
        let claim: Vec<u8> = (0..33).collect();
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

        assert_noop!(
            PoeModule::create_claim(Origin::signed(1), claim),
            Error::<Test>::ProofTooLong
        );
    })
}

#[test]
fn create_claim_failed_when_claim_is_too_short() {
	new_test_ext().execute_with(|| {
        let claim: Vec<u8> = vec![];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

        assert_noop!(
            PoeModule::create_claim(Origin::signed(1), claim),
            Error::<Test>::ProofTooShort
        );
    })
}

#[test]
fn revoke_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

        assert_ok!(PoeModule::revoke_claim(Origin::signed(1), claim));
    })
}

#[test]
fn revoke_claim_failed_when_claim_is_not_exist() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];

        assert_noop!(
            PoeModule::revoke_claim(Origin::signed(1), claim),
            Error::<Test>::NoSuchProof
        );
    })
}

#[test]
fn revoke_claim_failed_with_wrong_owner() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

        assert_noop!(
            PoeModule::revoke_claim(Origin::signed(2), claim),
            Error::<Test>::NotProofOwner
        );
    })
}

#[test]
fn transfer_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

        assert_ok!(PoeModule::transfer_claim(Origin::signed(1), 2, claim.clone()));
        assert_eq!(Proofs::<Test>::get(&claim), Some((2, frame_system::Pallet::<Test>::block_number())));
    })
}

#[test]
fn transfer_claim_failed_when_claim_is_not_exist() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];

        assert_noop!(
            PoeModule::transfer_claim(Origin::signed(1), 2, claim),
            Error::<Test>::NoSuchProof
        );
    })
}

#[test]
fn transfer_claim_failed_with_wrong_owner() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

        assert_noop!(
            PoeModule::transfer_claim(Origin::signed(2), 3, claim),
            Error::<Test>::NotProofOwner
        );
    })
}
