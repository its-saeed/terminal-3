use crate::{mock::*, Error, Usernames};
use frame_support::{assert_err, assert_ok};
use sp_core::ConstU32;
use sp_runtime::BoundedVec;

#[test]
fn set_username_works() {
    new_test_ext().execute_with(|| {
        // Go past genesis block so events get deposited
        System::set_block_number(1);

        // Test valid username
        let valid_username = b"alice".to_vec();
        assert_ok!(Template::set_username(
            RuntimeOrigin::signed(1),
            valid_username.clone()
        ));

        // Read pallet storage and assert the username was set
        assert_eq!(
            Usernames::<Test>::get(1),
            BoundedVec::<u8, ConstU32<32>>::try_from(valid_username).unwrap()
        );

        use frame_system::RawOrigin;

        let user = RuntimeOrigin::from(RawOrigin::Signed(1));
        println!("{:?}", user);

        // Test username that's too long
        let long_username = b"this_username_is_way_too_long_for_our_storage".to_vec();
        assert_err!(
            Template::set_username(RuntimeOrigin::signed(2), long_username),
            Error::<Test>::UsernameTooLong
        );

        // Test invalid characters
        let invalid_username = b"invalid@name".to_vec();
        assert_err!(
            Template::set_username(RuntimeOrigin::signed(3), invalid_username),
            Error::<Test>::InvalidUsername
        );
    });
}
