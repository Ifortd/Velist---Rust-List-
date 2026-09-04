

struct Velist<T> {
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

struct NodeLight<T> {
    index: *mut Node<T>,
    parent: *mut Velist<T>
}


impl<T> Node<T> {
    fn new(mut content: T, prev: usize, next: usize ) -> Self {
        Node {
            next,
            prev,
            content: content //&mut content as *mut T
        }
    }
}


impl<T> Velist<T> {

    fn new() -> Velist<T> {
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

  /*  pub fn pop_first(&mut self) -> T {
        if self.is_empty() {panic!()};

        let out = &mut self.data[ self.first ].content ;

        self.next_free.push( self.first );
        self.first = self.data[self.first].next;
        self.data[self.last].next = self.data[self.first].next;
        out
    }*/



  //  pub fn pop_this(&mut self, node: Node<T>) -> Node<T> {
//
   // }

}