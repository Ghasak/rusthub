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

    // First we will make a global static class-variable
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


    // We push the object that we created to the data container we created using the `lazy_static`
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
    let _emp3 = Employee::new("Jason".to_string());



    println!("Counter: {}", get_counter());

    for emp in get_instances() {
        println!("-> {}::{}", get_counter(), emp.name)
    }
}




