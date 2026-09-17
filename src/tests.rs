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
        assert_eq!(* veit.get_ref(), numbers[ veit.pos.clone() as usize ]  );
        veit.move_next();
    }

    veit.move_next();
    while !veit.at_the_end() {
        // println!("veit item: {}, arra item: {}", veit.get_im(), numbers[ veit.pos.clone() as usize ]);
        assert_eq!(* veit.get_ref(), numbers[ (veit.pos.clone() ) as usize % numbers.len()]  );
        veit.move_next();
    }

    veit.move_prev();
    while !veit.at_the_end() {
        println!("veit item: {}, pos {}, arra item: {}", veit.get_ref(), veit.pos, numbers[ (veit.pos.clone() ) as usize % numbers.len() ]);
        assert_eq!(* veit.get_ref(), numbers[ (veit.pos.clone() ) as usize % numbers.len()]  );
        veit.move_prev();
    }

    veit.move_prev();

    while !veit.at_the_end() {
        println!("veit item: {}, pos {}, arra item: {}", veit.get_ref(), veit.pos, numbers[ (veit.pos.clone() ) as usize % numbers.len() ]);
        assert_eq!(* veit.get_ref(), numbers[ (veit.pos.clone() ) as usize % numbers.len()]  );
        veit.move_prev();
    }

}

#[test]
fn test_push() {
    println!("push test");
    let numbers = vec![1,2,3,4,5,6,7];

    let mut vel: Velist<i64> = Velist::new();



    for number in  & numbers {
        vel.push_last(*number);
    }
    //numbers.item
    let mut veit = vel.get_iter();


    while !veit.at_the_end() {
        println!("veit item: {}, pos {}, arra item: {}", veit.get_ref(), veit.pos, numbers[ (veit.pos.clone() ) as usize % numbers.len() ]);
        veit.move_next();
    }

    for i in &mut ( vel.get_iter()) {
        print!(" el {},", i)
    }
    println!();

    println!("after iter ");
    let mut veit = vel.get_iter();

    while !veit.at_the_end() {
        println!("veit item: {}, pos {}, arra item: {}", veit.get_ref(), veit.pos, numbers[ (veit.pos.clone() ) as usize % numbers.len() ]);
        assert_eq!(* veit.get_ref(), numbers[ (veit.pos.clone() ) as usize % numbers.len()]  );
        veit.move_next();
    }


    struct NoClone {
        val: i64
    }
    let mut vel2: Velist<NoClone> = Velist::new();
    vel2.push_first( NoClone{val: 1} );
    vel2.push_first( NoClone{val: 2} );
    vel2.push_first( NoClone{val: 3} );

    //for i in  &mut vel2.get_iter() {
    //    print!(" el {},", i)
    //}
    //for i in veit {
//
    }

#[test]
fn test_ambigious_iter() {
    let numbers = vec![1,2,3,4,5,6,7];

    let mut vel: Velist<i64> = Velist::new();



    for number in  & numbers {
        vel.push_last(*number);
    }
    
    let mut amb = vel.get_vague_iter();
    let mut amb2 = vel.get_vague_iter();
    
    

    println!("amb item {}", amb.get(&mut vel));
    amb.move_forward( &mut vel );
    println!("amb item {}", amb.get(&mut vel));
    amb.move_forward( &mut vel );

    amb2.move_forward( &mut vel )
}





