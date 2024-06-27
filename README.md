![Build](https://github.com/jproyo/owned-ref-cell/actions/workflows/build.yml/badge.svg)

# OwnedRefCell

The `OwnedRefCell` struct provides interior mutability similar to `RefCell` but without the lifetime constraints associated with the returned references. It consists of three main components:

1. **`OwnedRefCell<T>`**:
   - Wraps an `UnsafeCell<T>` for interior mutability.
   - Uses a `Cell<TimesBorrowed>` to track the number of borrows.
   - Provides methods for borrowing and mutable borrowing.

2. **`OwnedRef<T>`**:
   - Represents an immutable reference.
   - Holds a `NonNull<T>` and a pointer to the `OwnedRefCell`.
   - Decrements the borrow count when dropped.

3. **`OwnedRefMut<T>`**:
   - Represents a mutable reference.
   - Similar to `OwnedRef`, but allows mutable access.
   - Resets the borrow count when dropped.

## Example Usage

### Borrow

```rust
use owned_ref_cell::OwnedRefCell;

let my_value = 42;
let cell = OwnedRefCell::new(my_value);

// Borrow an immutable reference
let borrowed_ref = cell.borrow();
println!("Value: {}", *borrowed_ref);
```

### Borrow Mut

```rust
use owned_ref_cell::OwnedRefCell;

let my_value = 42;
let cell = OwnedRefCell::new(my_value);

// Borrow a mutable reference
let mut borrowed_mut = cell.borrow_mut();
*borrowed_mut += 10;
println!("Updated value: {}", *borrowed_mut);
```

### Panic cases on using `borrow` and `borrow_mut`

Same as `RefCell`

1. If you try to `borrow_mut` in same scope after `borrow`
2. If you try to `borrow` in same scope after `borrow_mut`

You can `borrow` multiple times if there is not `borrow_mut` in same scope.


## Run Tests

```bash
cargo test
```
