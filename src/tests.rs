use std::collections::VecDeque;
use crate::velist::{Velist, VelistIterable};
use super::velist;

#[test]
fn test_the_tests() {
    println!("tests work!")
}


#[test]
fn test_traversal() {
    let numbers = [1,2,3,4,5,6,7];


    let mut vel: Velist<i64> = Velist::new();

    for number in numbers {
        vel.push_last(number);
    }

    let mut veit = vel.get_iter();

    while !veit.at_the_end() {
       // println!("veit item: {}, arra item: {}", veit.get_im(), numbers[ veit.pos.clone() as usize ]);
        assert_eq!( * veit.get_im(), numbers[ veit.pos.clone() as usize ]  );
        veit.next();
    }

    veit.next();
    while !veit.at_the_end() {
        // println!("veit item: {}, arra item: {}", veit.get_im(), numbers[ veit.pos.clone() as usize ]);
        assert_eq!( * veit.get_im(), numbers[ (veit.pos.clone() ) as usize % numbers.len()]  );
        veit.next();
    }

    veit.prev();
    while !veit.at_the_end() {
        println!("veit item: {}, pos {}, arra item: {}", veit.get_im(), veit.pos, numbers[ (veit.pos.clone() ) as usize % numbers.len() ]);
        assert_eq!( * veit.get_im(), numbers[ (veit.pos.clone() ) as usize % numbers.len()]  );
        veit.prev();
    }

    veit.prev();

    while !veit.at_the_end() {
        println!("veit item: {}, pos {}, arra item: {}", veit.get_im(), veit.pos, numbers[ (veit.pos.clone() ) as usize % numbers.len() ]);
        assert_eq!( * veit.get_im(), numbers[ (veit.pos.clone() ) as usize % numbers.len()]  );
        veit.prev();
    }

}

#[test]
fn test_push() {
    let numbers = vec![1,2,3,4,5,6,7];

    let mut vel: Velist<i64> = Velist::new();

    for number in  & numbers {
        vel.push_last(*number);
    }
    //numbers.item
    let mut veit = vel.get_iter();


    while !veit.at_the_end() {
        println!("veit item: {}, pos {}, arra item: {}", veit.get_im(), veit.pos, numbers[ (veit.pos.clone() ) as usize % numbers.len() ]);
        veit.next();
    }

    veit.

    //for i in veit {
//
    //}



}
