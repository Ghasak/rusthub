# Rust count Instances

## Implementation

```rust

#![allow(dead_code)]
#[macro_use]
extern crate lazy_static;

use std::sync::atomic;
use std::sync::{Arc, Mutex, Weak};

fn main() {
    #[derive(Debug, Clone)]
    pub struct Employee {
        pub name: String,
    }

    impl Employee {
        pub fn new(name: String) -> Arc<Self> {
          //  println!("Entity named {} is created ..", name);
            let emp = Arc::new(Employee { name });

            bump_counter();
            remember_instance(emp.clone());
            emp
        }

        /*
         * Lets adde -1- each time we create an employee
         */
    }

    static COUNTER: atomic::AtomicUsize = atomic::AtomicUsize::new(0);

    fn bump_counter() {
        COUNTER.fetch_add(1, atomic::Ordering::SeqCst);
    }

    pub fn get_counter() -> usize {
        COUNTER.load(atomic::Ordering::SeqCst)
    }

    lazy_static! {
        static ref INSTANCES: Mutex<Vec<Weak<Employee>>> = Mutex::new(vec![]);
    }


    fn remember_instance(entity: Arc<Employee>) {
        // Downgrade to a weak reference.  Type constraint is just for clarity.
        let entity: Weak<Employee> = Arc::downgrade(&entity);
        INSTANCES
            // Lock mutex
            .lock()
            .expect("INSTANCES mutex was poisoned")
            // Push entity
            .push(entity);
    }

    pub fn get_instances() -> Vec<Arc<Employee>> {
        /*
        This is about as inefficient as I could write this, but again, without
        knowing your access patterns, I can't really do any better.
        */
        INSTANCES
            // Lock mutex
            .lock()
            .expect("INSTANCES mutex was poisoned")
            // Get a borrowing iterator from the Vec.
            .iter()
            /*
            Convert each `&Weak<Entity>` into a fresh `Arc<Entity>`.  If we
            couldn't (because the weak ref is dead), just drop that element.
            */
            .filter_map(|weak_entity| weak_entity.upgrade())
            // Collect into a new `Vec`.
            .collect()
    }

    let _emp1 = Employee::new("Jack".to_string());
    // println!("{:#?}", _emp1);
    // println!("{:#?}", get_counter());
    let _emp2 = Employee::new("Michael".to_string());
    // println!("{:#?}", _emp2);
    // println!("{:#?}", get_counter());

    println!("Counter: {}", get_counter());

    for ent in get_instances() {
        println!("-> {}", ent.name)
    }
}
```



## You can find two Implementations existed here:

- [Object counter (num.instances of object that
  exist](https://stackoverflow.com/questions/67959660/object-counter-num-instances-of-object-that-exist)
- [Detecting new struct intitalization](https://stackoverflow.com/questions/36993255/detecting-new-struct-initialization)


```rust
#[macro_use]
extern crate lazy_static;

use std::any::{TypeId};
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
  static ref OBJCOUNTER: Mutex<HashMap<TypeId, u64>> = Mutex::new(HashMap::new());
}

pub trait Counting : Sized  where Self:'static{
  fn new() -> Self {
    let new_instance = Self::new_instance();
    let typeid = TypeId::of::<Self>();
    *OBJCOUNTER.lock().unwrap().entry(typeid).or_insert(0) += 1;
    new_instance
  }
  fn new_instance() -> Self;

  fn destroy(&self) {
    let typeid = TypeId::of::<Self>();
    *OBJCOUNTER.lock().unwrap().get_mut(&typeid).unwrap() -= 1;
  }

  fn count() -> u64 {
    let typeid = TypeId::of::<Self>();
    *OBJCOUNTER.lock().unwrap().get(&typeid).unwrap_or(&0)
  }
}


struct Struct1 {}

impl Counting for Struct1 {
  fn new_instance() -> Struct1 {
    Struct1 {}
  }
}

impl Drop for Struct1 {
  fn drop(&mut self) {
    self.destroy();
  }
}

struct Struct2 {}
impl Counting for Struct2 {
  fn new_instance() -> Struct2 {
    Struct2 {}
  }
}

impl Drop for Struct2 {
    fn drop(&mut self) {
      self.destroy();
    }
  }

fn main() {
assert_eq!(Struct2::count(), 0);
    let _s2_1 = Struct2::new();
    assert_eq!(Struct2::count(), 1);

    assert_eq!(Struct1::count(), 0);
    let _s1_1 = Struct1::new();
    {
      let _s1_2 = Struct1::new();
      assert_eq!(Struct1::count(), 2);
    }
    assert_eq!(Struct1::count(), 1);
    println!("Passed asserts");
}
```

