//use std::io::SeekFrom;

pub struct Velist<T> {
    data: Vec<Node<T>>,
    next_free: Vec<usize>,
    first: usize,
    //last: usize,
    size: usize,

    full: bool // depracted
}


struct Node<T> {
    next: usize,
    prev: usize,
    // use option ?
    // whether this
    #[deprecated]
    active: bool,
    writes_count: u64,
    content: Option<T>
}

/*impl<T> Clone for Node<T> {
    fn clone(&self) -> Self {
        Self {
            next: self.next,
            prev: self.prev,
            content: self.content
        }
    }
}*/


pub struct VelistAmbigiousIter<T> {
    index: usize,
    last_known_id: u64,
    parent: *const Velist<T>
}

impl<T> VelistAmbigiousIter<T> where T: Copy {
    pub fn get(&mut self, velist: &mut Velist<T>) -> T {
        if !self.is_valid(velist) {
            panic!()
        };
        velist.at( self.index ).clone().unwrap()
    }
}

impl<'a, T> VelistAmbigiousIter<T> {
    pub fn get_mut(&mut self, velist: &'a mut Velist<T>) -> &'a mut T {
        if !self.is_valid(velist) {
            panic!()
        };
        velist.get_element_n_mut(self.index)
    }
}

impl<T> VelistAmbigiousIter<T> {

    fn is_valid(&self, velist: & Velist<T>) -> bool {
        if ( self.parent != velist ) {
            return false
        }
        if ( velist.data[ self.index].writes_count != self.last_known_id ) {
            return false
        }
        if  velist.data[ self.index].content.is_none() {
            return false
        }
        return true
    }

    pub fn move_forward(&mut self, velist: &mut Velist<T> ) {
        //let index = iterator.index;
        if !self.is_valid(velist) {
            panic!()
        };
        self.index = velist.get_next(self.index)
    }

    /*fn access_by_vague_iterator(&mut self, iterator: AmbigiousIter<T> ) -> &mut T {
        //if (self. )
        let index = iterator.index;
        if ( self.data[index].writes_count != iterator.last_known_id ) {
            panic!()
        };
        return self.data[index].content.as_mut().unwrap()
    }*/
}

pub struct VelistIter<'a, T> {
    index: usize,
    pub pos: i64,
    parent: &'a mut Velist<T>
}

pub struct VelistIterIm<'a, T> {
    index: usize,
    pub pos: i64,
    parent: &'a Velist<T>
}

pub trait VelistIterable<T> {

    fn move_next(&mut self);
    fn move_prev(&mut self);
  //  fn pos(& self) -> usize;

    fn at_the_end(&self) -> bool;

}

impl  <'a, T> VelistIterIm<'a, T>  {
    fn new(index: usize, velist: &'a mut Velist<T>) -> Self<> {
        VelistIterIm {
            index: index,
            parent: velist,
            pos: 0,
        }
    }

    fn get(&self) -> & T {
        self.parent.get_element_n_ref( self.index )
    }
}

impl <T> VelistIterable<T> for VelistIterIm<'_, T> {

    fn move_next(&mut self) {
        self.index = self.parent.get_next( self.index );
        self.pos += 1;
    }

    fn move_prev(&mut self) {
        self.index = self.parent.get_prev( self.index );
        self.pos -= 1;
    }

    fn at_the_end(&self) -> bool {
        if self.parent.get_next( self.index ) == self.parent.first {true} else {false}
    }

}

impl<'a, T> VelistIter<'a, T> {

    fn new(index: usize, velist: &'a mut Velist<T>) -> Self<> {
        VelistIter {
            index: index,
            parent: velist,
            pos: 0,
        }
    }

    pub fn get_mut(&mut self) -> &mut T {
        self.parent.get_element_n_mut( self.index )
    }

    pub fn get_ref(&self) -> & T {
        self.parent.get_element_n_ref( self.index )
    }

    pub fn pop(&mut self) -> T {

        let out = self.parent.pop_me(self.index);
        self.move_next();
        self.pos -= 1; // because .next adds one
        out
    }
}


impl<'a, T> Iterator for VelistIter<'a, T> where T: Copy {
    type Item = T ;

    fn next(&mut self) -> Option<Self::Item> {
        //let out = Option::Some(self.parent.get_element_n_mut( self.index ));
        //self.pos += 1;
        let out =  self.parent.at( self.index );

        self.move_next();
        if self.index == self.parent.first {
            return Option::None
        }
        out
       // self.index = self.parent.get_next( self.index );
        //if ( self.index == self.parent.first ) {
      //      return Option::None
       // }
       // out
        //Option::Some( self.get() )
    }
}

impl <T> VelistIterable<T> for VelistIter<'_, T> {

    fn move_next(&mut self) {
        self.pos += 1;
        self.index = self.parent.get_next( self.index );
    }
    fn move_prev(&mut self) {
        self.index = self.parent.get_prev( self.index );
        self.pos -= 1;
    }
    fn at_the_end(&self) -> bool {
        if self.parent.get_next( self.index ) == self.parent.first {true} else {false}
    }
}


impl<T> Node<T> {
    fn new(mut content: T, prev: usize, next: usize ) -> Self {
        Node {
            next,
            prev,
            content: Option::Some(content),  //&mut content as *mut T
            active: true,
            writes_count: 0
        }
    }
}


/*impl<T> Iterator for Velist<T> where T: Copy {
    type Item = T ;

    fn next(&mut self) -> Option<Self::Item> {
        //let out = Option::Some(self.parent.get_element_n_mut( self.index ));
        //self.pos += 1;

        let out = self.at(self.first);
        self.first = self.get_next(self.first);
        out

        /*if self.is_empty() {
            Option::None
        } else {
            Option::Some(self.pop_me(self.first))
        }*/
    }
}*/


impl<T> Velist<T> where T: Copy {
    fn at(&mut self, n: usize) -> Option<T>{
        self.data[n].content.clone()
    }
}

impl<T> Velist<T> {

    pub fn new() -> Velist<T> {
        Velist {
            data: Vec::new(),
            next_free: Vec::new(),
            full: true,
            //last: 0,
            size: 0,
            first: 0,
        }
    }

    // potentially unsafe
    fn last(&self) -> usize {
        if (self.data.len() == 0) {return 0}
        self.data[ self.first ].prev
    }

    ///
    /// Pushes obj to a Velist and returns index of obj node in velist
    /// # Unsafe
    /// As if right after calling there is no element in velist pointing to index of obj, meaning if we lose return index - obj will stay in velist forever
    /// resulting in memory leak
    unsafe fn push_ambigious(&mut self, obj: T) -> usize {
        let obj_node = Node::new( obj, Self::last(self), self.first  );
        self.size += 1;

        let next = if self.next_free.len() == 0 {
            // we add obj to the end of data
            self.data.push( obj_node );
            // index of a obj we just pushed
            self.data.len() - 1
        } else {
            // index of where we gonna push obj
            let next = self.next_free.pop().unwrap();
            // push the obj
            self.data[ next ] = obj_node;
            // index of an obj we just pushed
            next
        };

        self.data[next].active = true;
        self.data[next].writes_count += 1;
        next
    }

    pub fn push_last(&mut self, obj: T) {

        let new = unsafe{ self.push_ambigious(obj) };

        //upodating index of last item
        let last_ind = Self::last(self);
        self.data[ last_ind ].next = new;
        //  updating index of the first so it points to obj behin
        self.data[ self.first ].prev = new;
    }

    /// Pushes element at the beginning
    pub fn push_first(&mut self, obj: T) {

        let new =  unsafe{ self.push_ambigious(obj) };

        //upodating index of last item
        let last_ind = Self::last(self);
        self.data[ last_ind ].next = new;
        // updating index of the first so it points to obj behind
        self.data[ self.first ].prev = new;


        //updating first so now new item is first
        self.first = new;
    }

    /// Returns true if there is no elements in this velist
    pub fn is_empty(&self) -> bool {
        if self.size == 0 {true} else {false}
    }

    #[deprecated]
    pub fn get_first(&self) -> &Node<T> {
        if self.is_empty() {panic!()};

        &self.data[ self.first ]
    }

    /// Returns index of element after element n
    fn get_next(&self, cur_index: usize) -> usize {
        self.data[cur_index].next
    }

    /// Returns index of element before element n
    fn get_prev(&self, n: usize) -> usize {
        self.data[n].prev
    }

    /// Pops element n
    fn pop_element_n(&mut self, n: usize) -> T {
        self.data[n].writes_count += 1;
        self.data[n].active = false;
        self.data[n].content.take().unwrap()
    }

    ///
    /// Returns mutable to element n
    /// # Panics
    /// If element n has content = None
    fn get_element_n_mut(&mut self, n: usize) -> &mut T {
        self.data[n].content.as_mut().unwrap()
    }

    ///
    /// Returns reference to element n
    /// # Panics
    /// If element n has content = None
    fn get_element_n_ref(& self, n: usize) -> & T {
         self.data[n].content.as_ref().unwrap()
    }

    //unsafe? UNDONE
    fn pop_me(&mut self, index: usize) -> T {
        self.next_free.push( index );
        let prev = self.data[index].prev;
        let next = self.data[index].next;

        self.data[prev].next = next;
        self.data[next].prev = prev;

        self.data[index].active = false;
        self.data[index].writes_count += 1;

        self.size -= 1;

        if (index == self.first) {
            self.first = next;
        }

        self.data[index].content.take().unwrap()

       // self.data[index].content;

      //  drop( self.data[index].content )
    }

    pub fn get_vague_iter(&self) -> VelistAmbigiousIter<T> {
        VelistAmbigiousIter {
            index: self.first,
            last_known_id: self.data[self.first].writes_count,
            parent: self as *const Velist<T>
        }
    }

    pub fn get_iter(&mut self) -> VelistIter<T> {
        VelistIter::new(
            self.first,
            self
        )
    }


    pub fn get_iter_im(&mut self) -> VelistIterIm<T> {
        VelistIterIm::new(
            self.first,
            self
        )
    }

}