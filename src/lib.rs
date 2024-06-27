use std::cell::{Cell, UnsafeCell};
use std::ptr::NonNull;

type TimesBorrowed = isize;
const EMPTY: isize = 0;
const BORROWED_MUT: isize = -1;

pub struct OwnedRefCell<T> {
    value: UnsafeCell<T>,
    borrow: Cell<TimesBorrowed>,
}

pub struct OwnedRef<T> {
    value: NonNull<T>,
    borrowed_pointer: *const OwnedRefCell<T>,
}

impl<T> Drop for OwnedRef<T> {
    fn drop(&mut self) {
        unsafe {
            let owned_ref_cell = &*self.borrowed_pointer;
            let check_borrowed = owned_ref_cell.borrow.get();
            assert!(check_borrowed > EMPTY);
            owned_ref_cell.borrow.set(check_borrowed - 1);
        }
    }
}

impl<T> AsRef<T> for OwnedRef<T> {
    fn as_ref(&self) -> &T {
        unsafe { self.value.as_ref() }
    }
}

pub struct OwnedRefMut<T> {
    value: NonNull<T>,
    borrowed_pointer: *const OwnedRefCell<T>,
}

impl<T> AsRef<T> for OwnedRefMut<T> {
    fn as_ref(&self) -> &T {
        unsafe { self.value.as_ref() }
    }
}

impl<T> Drop for OwnedRefMut<T> {
    fn drop(&mut self) {
        unsafe {
            let owned_ref_cell = &*self.borrowed_pointer;
            owned_ref_cell.borrow.set(EMPTY);
        }
    }
}

impl<T> AsMut<T> for OwnedRefMut<T> {
    fn as_mut(&mut self) -> &mut T {
        unsafe { self.value.as_mut() }
    }
}

impl<T> OwnedRefCell<T> {
    pub fn new(value: T) -> Self {
        OwnedRefCell {
            value: UnsafeCell::new(value),
            borrow: Cell::new(EMPTY),
        }
    }

    pub fn borrow(&self) -> OwnedRef<T> {
        match self.try_borrow() {
            Some(owned_ref) => owned_ref,
            None => panic!("Someone else is mutable borrowing this. Cannot borrow."),
        }
    }

    pub fn borrow_mut(&self) -> OwnedRefMut<T> {
        match self.try_borrow_mut() {
            Some(owned_ref_mut) => owned_ref_mut,
            None => panic!(
                "Someone else is borrowing or mutable borrowing this. Cannot borrow mutably."
            ),
        }
    }

    pub fn try_borrow(&self) -> Option<OwnedRef<T>> {
        let borrowed = self.borrow.get();
        if borrowed >= EMPTY {
            self.borrow.set(borrowed + 1);
            Some(OwnedRef {
                value: unsafe { NonNull::new_unchecked(self.value.get()) },
                borrowed_pointer: self,
            })
        } else {
            None
        }
    }

    pub fn try_borrow_mut(&self) -> Option<OwnedRefMut<T>> {
        let borrowed = self.borrow.get();
        if borrowed == EMPTY {
            self.borrow.set(BORROWED_MUT);
            Some(OwnedRefMut {
                value: unsafe { NonNull::new_unchecked(self.value.get()) },
                borrowed_pointer: self,
            })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_owned_ref_cell() {
        let owned_ref_cell = OwnedRefCell::new(5);

        let owned_ref = owned_ref_cell.borrow();
        assert_eq!(*owned_ref.as_ref(), 5);
    }

    #[test]
    fn test_owned_ref_cell_borrow_multiple_times() {
        let owned_ref_cell = OwnedRefCell::new(5);

        let owned_ref = owned_ref_cell.borrow();
        let owned_ref2 = owned_ref_cell.borrow();
        assert_eq!(*owned_ref.as_ref(), 5);
        assert_eq!(*owned_ref2.as_ref(), 5);
    }

    #[test]
    fn test_owned_ref_cell_borrow_mut_and_mutate() {
        let owned_ref_cell = OwnedRefCell::new(5);

        let mut owned_ref_mut = owned_ref_cell.borrow_mut();
        *owned_ref_mut.as_mut() = 10;
        assert_eq!(*owned_ref_mut.as_ref(), 10);
    }

    #[test]
    #[should_panic]
    fn test_owned_ref_cell_borrow_mut_borrow_after_should_panic() {
        let owned_ref_cell = OwnedRefCell::new(5);

        let _owned_ref_mut = owned_ref_cell.borrow_mut();
        let _owned_ref = owned_ref_cell.borrow();
    }

    #[test]
    #[should_panic]
    fn test_owned_ref_cell_borrow_mut_twice_panic() {
        let owned_ref_cell = OwnedRefCell::new(5);

        let _owned_ref_mut = owned_ref_cell.borrow_mut();
        let _owned_ref_mut2 = owned_ref_cell.borrow_mut();
    }

    #[test]
    #[should_panic]
    fn test_owned_ref_cell_borrow_after_borrow_mut_should_panic() {
        let owned_ref_cell = OwnedRefCell::new(5);

        let _owned_ref = owned_ref_cell.borrow();
        let _owned_ref_mut = owned_ref_cell.borrow_mut();
    }

    #[test]
    fn test_owned_ref_cell_drop_not_borrow() {
        let owned_ref_cell = OwnedRefCell::new(5);
        {
            let _owned_ref = owned_ref_cell.borrow_mut();
        }
        let owned_ref = owned_ref_cell.borrow();
        assert_eq!(*owned_ref.as_ref(), 5);
    }

    #[test]
    fn test_owned_ref_cell_borrow_mut_drop_borrow_after() {
        let owned_ref_cell = OwnedRefCell::new(5);
        {
            let _owned_ref = owned_ref_cell.borrow();
        }
        let mut owned_ref = owned_ref_cell.borrow_mut();
        *owned_ref.as_mut() = 10;
        assert_eq!(*owned_ref.as_ref(), 10);
    }
}
