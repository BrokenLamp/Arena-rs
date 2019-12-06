pub struct Arena {
    container: Vec<u8>,
    capacity: usize,
    offset: usize,
}

impl<'a> Arena {
    pub fn new(size: usize) -> Self {
        Arena {
            container: Vec::with_capacity(size),
            capacity: size,
            offset: 0,
        }
    }
    pub fn alloc<T>(&mut self, value: T) -> Option<&'a mut T> {
        let size = std::mem::size_of::<T>();
        if self.offset + size > self.capacity {
            return None;
        }
        for _ in 0..size {
            self.container.push(0u8);
        }
        let ptr: *mut u8 = &mut self.container[self.offset];
        let ptr = ptr.cast::<T>();
        self.offset += size;
        unsafe {
            *ptr = value;
            Some(&mut *ptr)
        }
    }
}

pub struct Pool<T> {
    container: Vec<T>,
    head: usize,
    used: usize,
}

impl<'a, T> Pool<T> {
    pub fn new(size: usize) -> Option<Self> {
        if size < 2 {
            return None;
        }
        if std::mem::size_of::<T>() < std::mem::size_of::<usize>() {
            return None;
        }
        let mut container = Vec::with_capacity(size);
        let pool = Pool {
            container: container,
            head: 0,
            used: 0,
        };
        pool.create_link(0, 1);
        Some(pool)
    }
    pub fn alloc(&mut self, value: T) -> PBox<'a, T> {}

    pub(crate) fn dealloc(&mut self, ptr: *mut T) {}

    fn create_link(&mut self, head: usize, to: usize) {
        let c_ptr = &mut self.container as *mut Vec<T>;
        let c_ptr = c_ptr.cast::<usize>().offset(head as isize);
        unsafe {
            *c_ptr = to;
        }
    }
}

pub struct PBox<'a, T> {
    ptr: *mut T,
    pool: &'a Pool<T>,
}

impl<'a, T> PBox<'a, T> {
    pub(crate) fn new(ptr: *mut T, pool: &'a Pool<T>) -> Self {
        PBox {
            ptr: ptr,
            pool: pool,
        }
    }
}

impl<'a, T> Drop for PBox<'a, T> {
    fn drop(&mut self) {
        self.pool.dealloc(self.ptr);
    }
}

#[test]
fn single_allocation() {
    let mut arena = Arena::new(1024);
    let value = arena.alloc(7).unwrap();
    assert_eq!(*value, 7);
}

#[test]
fn out_of_space() {
    let mut arena = Arena::new(4);
    arena.alloc(7).unwrap();
    let value = arena.alloc(7);
    assert_eq!(value, None);
}

#[test]
fn multiple_allocations() {
    let mut arena = Arena::new(1024);
    let value_78 = arena.alloc(78).unwrap();
    let value_42 = arena.alloc(42).unwrap();

    {
        let value_17 = arena.alloc(17).unwrap();
        assert_eq!(*value_17, 17);
    }
    // value_17 is out of scope, and so cannot be used
    // but it hasn't been freed

    assert_eq!(*value_78, 78);
    assert_eq!(*value_42, 42);
    assert_eq!(*value_78 + *value_42, 78 + 42);

    // All values are now freed in one giant batch
}
