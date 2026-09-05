use std::io::SeekFrom;

pub struct Velist<T> {
    data: Vec<Node<T>>,
    next_free: Vec<usize>,
    first: usize,
    last: usize,
    size: usize,

    full: bool // depracted
}


struct Node<T> {
    next: usize,
    prev: usize,
    content: T
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

    fn next(&mut self);
    fn prev(&mut self);
  //  fn pos(& self) -> usize;

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
        self.parent.get_element_n_im( self.index )
    }
}

impl <T> VelistIterable<T> for VelistIterIm<'_, T> {

    fn next(&mut self) {
        self.index = self.parent.get_next( self.index );
        self.pos += 1;
    }

    fn prev(&mut self) {
        self.index = self.parent.get_prev( self.index );
        self.pos -= 1;
    }

   // fn pos(&self) -> usize {
   //     self.pos
   // }
}

impl<'a, T> VelistIter<'a, T> {

    fn new(index: usize, velist: &'a mut Velist<T>) -> Self<> {
        VelistIter {
            index: index,
            parent: velist,
            pos: 0,
        }
    }

    pub fn get(&mut self) -> &mut T {
        self.parent.get_element_n( self.index )
    }

    pub fn pop(self) {
        self.parent.pop_me(self.index)
    }
}


impl <T> VelistIterable<T> for VelistIter<'_, T> {

    fn next(&mut self) {
        self.index = self.parent.get_next( self.index );
    }
    fn prev(&mut self) {
        self.index = self.parent.get_prev( self.index );
        self.pos -= 1;
    }
  //  fn pos(&self) -> usize {
  //      self.pos
   // }
}


impl<T> Node<T> {
    fn new(mut content: T, prev: usize, next: usize ) -> Self {
        Node {
            next,
            prev,
            content: content  //&mut content as *mut T
        }
    }
}


impl<T> Velist<T> {

    pub fn new() -> Velist<T> {
        Velist {
            data: Vec::new(),
            next_free: Vec::new(),
            full: true,
            last: 0,
            size: 0,
            first: 0,
        }
    }

    /*
    Pushes obj to a Velist
    and return index of where it was pushed
    Unsafe because the only way to access obj after this method is to use returned index - and if you lose this index obj become unaccessible
     */
    unsafe fn push_ambigious(&mut self, obj: T) -> usize {
        let obj_node = Node::new( obj, self.last, self.first  );
        self.size += 1;

        if self.next_free.len() == 0 {
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
        }
    }

    pub fn push_last(&mut self, obj: T) {

        let new = unsafe{ self.push_ambigious(obj) };

        //updating current last so it points to the new one
        self.data[ self.last ].next = new;
        // updating index of the last to index of current obj
        self.last =  new;
    }

    pub fn push_first(&mut self, obj: T) {

        let new =  unsafe{ self.push_ambigious(obj) };

        //updating current first so it points to the new one
        self.data[ self.first ].prev = new;
        // updating index of the first to index of current obj
        self.first =  new;
    }

    pub fn is_empty(&self) -> bool {
        if self.size == 0 {true} else {false}
    }

    pub fn get_first(&self) -> &Node<T> {
        if self.is_empty() {panic!()};

        &self.data[ self.first ]
    }


    /*pub fn pop_first(&mut self) -> T {
        if self.is_empty() {panic!()};

        let out =  self.data[ self.first ].content. ;

        self.next_free.push( self.first );
        self.first = self.data[self.first].next;
        self.data[self.last].next = self.data[self.first].next;

        out
    }*/

    // returns reference to content
    fn get_element_n(&mut self, n: usize) -> &mut T {
        &mut self.data[n].content
    }

    // returns index of next element
    fn get_next(&self, cur_index: usize) -> usize {
        self.data[cur_index].next
    }

    fn get_prev(&self, cur_index: usize) -> usize {
        self.data[cur_index].prev
    }

    // returns reference to content
    fn get_element_n_im(& self, n: usize) -> & T {
        & self.data[n].content
    }

    //unsafe? UNDONE
    fn pop_me(&mut self, index: usize) {
        self.next_free.push( index );
        let prev = self.data[index].prev;
        let next = self.data[index].next;

        self.data[prev].next = next;
        self.data[next].prev = prev;

        self.size -= 1;
       // self.data[index].content;

      //  drop( self.data[index].content )
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

  //  pub fn pop_this(&mut self, node: Node<T>) -> Node<T> {
//
   // }

}