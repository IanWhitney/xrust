extern crate queen_attack;
extern crate knight_attack;

use queen_attack::*;
use knight_attack::*;

/*
 * Note:
 * This exercise requires that you've completed Queen Attack.
 * If you have not, please run `exercism fetch queen-attack before
 * starting this exercise.
 *
 * If you have completed queen-attack, but your solution is not
 * present on this computer, you can do ....something?
 *
 * To get these tests to pass you may find that you need to change
 * your Queen Attack solution. That is fine! But make sure that
 * your Queen Attack tests continue to pass.
*/

#[test]
fn queen_attack_exists() {
    assert!(Queen::new(2,1).is_ok(), "Queen attack is not available, see the note in the test file");
}

#[test]
fn test_knight_creation_with_valid_position() {
    let white_knight = Knight::new((2,4));
    assert!(white_knight.is_ok());
}

#[test]
fn test_knight_creation_with_incorrect_positions() {
    let white_knight = Knight::new((-1,2));
    assert!(white_knight.is_err());

    let white_knight = Knight::new((8,2));
    assert!(white_knight.is_err());

    let white_knight = Knight::new((5,-1));
    assert!(white_knight.is_err());

    let white_knight = Knight::new((5,8));
    assert!(white_knight.is_err());
}

#[test]
fn test_knights_can_not_attack() {
    let white_knight = Knight::new((2,4)).unwrap();
    let black_knight = Knight::new((6,6)).unwrap();
    assert_eq!(false, white_knight.can_attack(&black_knight));
}

#[test]
fn test_knights_can_attack_each_other_one() {
    let white_knight = Knight::new((4,3)).unwrap();
    let black_knight = Knight::new((2,2)).unwrap();
    assert_eq!(true, white_knight.can_attack(&black_knight));
    assert_eq!(true, black_knight.can_attack(&white_knight));
}

#[test]
fn test_knights_can_attack_each_other_two() {
    let white_knight = Knight::new((4,3)).unwrap();
    let black_knight = Knight::new((2,4)).unwrap();
    assert_eq!(true, white_knight.can_attack(&black_knight));
    assert_eq!(true, black_knight.can_attack(&white_knight));
}

#[test]
fn test_knights_can_attack_each_other_three() {
    let white_knight = Knight::new((4,3)).unwrap();
    let black_knight = Knight::new((3,5)).unwrap();
    assert_eq!(true, white_knight.can_attack(&black_knight));
    assert_eq!(true, black_knight.can_attack(&white_knight));
}

#[test]
fn test_knights_can_attack_each_other_four() {
    let white_knight = Knight::new((4,3)).unwrap();
    let black_knight = Knight::new((5,5)).unwrap();
    assert_eq!(true, white_knight.can_attack(&black_knight));
    assert_eq!(true, black_knight.can_attack(&white_knight));
}

#[test]
fn test_knights_can_attack_each_other_five() {
    let white_knight = Knight::new((4,3)).unwrap();
    let black_knight = Knight::new((6,4)).unwrap();
    assert_eq!(true, white_knight.can_attack(&black_knight));
    assert_eq!(true, black_knight.can_attack(&white_knight));
}

#[test]
fn test_knights_can_attack_each_other_six() {
    let white_knight = Knight::new((4,3)).unwrap();
    let black_knight = Knight::new((6,2)).unwrap();
    assert_eq!(true, white_knight.can_attack(&black_knight));
    assert_eq!(true, black_knight.can_attack(&white_knight));
}

#[test]
fn test_knights_can_attack_each_other_seven() {
    let white_knight = Knight::new((4,3)).unwrap();
    let black_knight = Knight::new((3,1)).unwrap();
    assert_eq!(true, white_knight.can_attack(&black_knight));
    assert_eq!(true, black_knight.can_attack(&white_knight));
}

#[test]
fn test_knights_can_attack_each_other_eight() {
    let white_knight = Knight::new((4,3)).unwrap();
    let black_knight = Knight::new((5,1)).unwrap();
    assert_eq!(true, white_knight.can_attack(&black_knight));
    assert_eq!(true, black_knight.can_attack(&white_knight));
}

#[test]
fn test_knight_can_attack_queen_but_not_vice_versa() {
    let white_knight = Knight::new((4,3)).unwrap();
    let black_queen = Queen::new((5,1)).unwrap();
    assert_eq!(true, white_knight.can_attack(&black_queen));
    assert_eq!(false, black_queen.can_attack(&white_knight));
}

#[test]
fn test_queen_can_attack_knight_but_not_vice_versa() {
    let white_knight = Knight::new((4,3)).unwrap();
    let black_queen = Queen::new((4,7)).unwrap();
    assert_eq!(false, white_knight.can_attack(&black_queen));
    assert_eq!(true, black_queen.can_attack(&white_knight));
}
