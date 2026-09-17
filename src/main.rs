use crate::velist::{Velist, VelistIterable};

mod velist;
mod tests;
//mod tests;

//use velist as vele;

//use tests;

struct pointer_owner<T> {
    content: T
}


impl<T> pointer_owner<T> {

    fn create_one(mut content: T) -> Self<> {
        pointer_owner {
            content: content // &mut content as *mut T
        }
    }

    fn get_it(&mut self) -> &mut T {
        unsafe{ return  &mut self.content }
    }

}

static mut COUNTER: usize = 0;

struct Dropper {
    string: String
}

impl Drop for Dropper {
    fn drop(&mut self) {
        println!("im dropped: {}", self.string)
    }
}

impl Dropper {

    fn new(string: String) -> Self {
        println!("Created dropper: {}", string);
        Dropper {
            string: string
        }
    }


}

struct NoClone {
    val: i64
}




fn main() {

    let mut vel: Velist<i64> = Velist::new();

    vel.push_last(1);
    vel.push_last(2);
    vel.push_last(3);
    vel.push_last(4);
    vel.push_last(5);



    let mut veit = vel.get_iter();

    println!("Vel element: {}", veit.get_mut());
    veit.move_next();
    println!("Vel element: {}", veit.get_mut());
    veit.move_next();
    println!("Vel element: {}", veit.get_mut());
    veit.move_next();
    println!("Vel element: {}", veit.get_mut());
    veit.move_next();
    println!("Vel element: {}", veit.get_mut());

    veit.move_next();
    println!("Vel element: {}", veit.get_mut());

    veit.move_next();
    println!("Vel element: {}", veit.get_mut());


    veit.move_prev();
    println!("Vel element: {}", veit.get_mut());

    veit.move_prev();
    println!("Vel element: {}", veit.get_mut());

    veit.move_prev();
    println!("Vel element: {}", veit.get_mut());

    veit.move_prev();
    println!("Vel element: {}", veit.get_mut());
    veit.pop();
    println!("POP");


    let mut veit2 = vel.get_iter();


    println!("Vel element: {}", veit2.get_mut());
    veit2.move_next();
    println!("Vel element: {}", veit2.get_mut());

    veit2.move_next();
    println!("Vel element: {}", veit2.get_mut());

    veit2.move_next();
    println!("Vel element: {}", veit2.get_mut());

    veit2.move_next();
    println!("Vel element: {}", veit2.get_mut());

    veit2.move_next();
    println!("Vel element: {}", veit2.get_mut());
    veit2.move_next();
    println!("Vel element: {}", veit2.get_mut());


    //ne.get();

   // println!("Vel element: {}", veit.get());

   // let mut veit2 = vel.get_iter();


   // let firsto = veit.get();

  //  println!("Vel element: {}", veit2.get());



    if vel.is_empty() {
        println!("sdsd")
    }


    //vel.

   // let mut tester = Dropper::new( "first".to_string() );

    //let sdsd: Box<pointer_owner<Dropper>> = Box::new(  pointer_owner::create_one(tester) );

    //let mut owner = pointer_owner::create_one(tester);

   // let cont = owner.get_it();


    //let owner = pointer_owner::create_one(tester);

   // let mut refer = &mut tester;

    let mut b = 54;

    let mut b_ref = & b;

    let sd = b_ref.clone();


    let mut vector = vec![NoClone {val:0}, NoClone {val:1}, NoClone {val:3} ];

    let mut val1 = &mut vector[0];
    let mut val1 = 0;
    //drop(val1);
    vector.push( NoClone {val:99} );
    let mut val2 = &mut vector[1];


    for mut i in vector {
        i.val += 1;
    }
    //let mut val1 = &mut vector[0];
    //val1.val += 1;

  //  b += 1;

    let c = 5 + *b_ref;
    //*b_ref += 1;

    {

        let mut a = Dropper::new( "a".to_string() );
        //refer = &mut a;
    }


    //println!("{}", refer.string.to_string());

    println!("Hello, world!");


}
