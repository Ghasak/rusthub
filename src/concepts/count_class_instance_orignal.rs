/*!
Because we need the `lazy_static` crate, you need to add the following to your
`Cargo.toml` file:

```cargo
[dependencies]
lazy_static = "0.2.1"
```
*/

#[macro_use]
extern crate lazy_static;

mod entity {
    use std::sync::atomic;
    use std::sync::{Arc, Mutex, Weak};

    pub struct Entity {
        pub name: String,
    }

    impl Entity {
        pub fn new(name: String) -> Arc<Self> {
            println!("Entity named {} was made.", name);
            let ent = Arc::new(Entity { name });
            bump_counter();
            remember_instance(ent.clone());
            ent
        }
    }

    /*
    The counter is simple enough, though I'm not clear on *why* you even want
    it in the first place.  You don't appear to be using it for anything...
    */
    //static COUNTER: atomic::AtomicUsize = atomic::ATOMIC_USIZE_INIT;
    static COUNTER: atomic::AtomicUsize = atomic::AtomicUsize::new(0);

    fn bump_counter() {
        // Add one using the most conservative ordering.
        COUNTER.fetch_add(1, atomic::Ordering::SeqCst);
    }

    pub fn get_counter() -> usize {
        COUNTER.load(atomic::Ordering::SeqCst)
    }

    /*
    There are *multiple* ways of doing this part, and you simply haven't given
    enough information on what it is you're trying to do.  This is, at best,
    a *very* rough guess.

    `Mutex` lets us safely mutate the vector from any thread, and `Weak`
    prevents `INSTANCES` from keeping every instance alive *forever*.  I mean,
    maybe you *want* that, but you didn't specify.

    Note that I haven't written a "cleanup" function here to remove dead weak
    references.
    */
    lazy_static! {
        static ref INSTANCES: Mutex<Vec<Weak<Entity>>> = Mutex::new(vec![]);
    }

    fn remember_instance(entity: Arc<Entity>) {
        // Downgrade to a weak reference.  Type constraint is just for clarity.
        let entity: Weak<Entity> = Arc::downgrade(&entity);
        INSTANCES
            // Lock mutex
            .lock()
            .expect("INSTANCES mutex was poisoned")
            // Push entity
            .push(entity);
    }

    pub fn get_instances() -> Vec<Arc<Entity>> {
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
}

fn main() {
    use entity::Entity;

    let e0 = Entity::new("Entity 0".to_string());
    println!("e0: {}", e0.name);
    {
        let e1 = Entity::new("Entity 1".to_string());
        println!("e1: {}", e1.name);

        /*
        `e1` is dropped here, which should cause the underlying `Entity` to
        stop existing, since there are no other "strong" references to it.
        */
    }
    let e2 = Entity::new("Entity 2".to_string());
    println!("e2: {}", e2.name);

    println!("Counter: {}", entity::get_counter());

    println!("Instances:");
    for ent in entity::get_instances() {
        println!("- {}", ent.name);
    }
}
