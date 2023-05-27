use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell;

struct Owner {
	name: String,
	gadgets:RefCell<Vec<Weak<Gadget>>>
}


struct Gadget {
	id: i32,
	owner: Rc<Owner>,
}

fn main() {
	let owner = Rc::new(
		Owner{
			name:"Felipe".to_string(),
            gadgets: RefCell::new(vec![]),});

	let gadget1 = Rc::new(
		Gadget { id:1, owner: Rc::clone(&owner)});
	let gadget2 = Rc::new(
		Gadget { id:32, owner: Rc::clone(&owner)});

   {
        let mut gadgets = owner.gadgets
		   .borrow_mut();
        gadgets.push(Rc::downgrade(&gadget1));
        gadgets.push(Rc::downgrade(&gadget2));

        // `RefCell` dynamic borrow ends here.
    }


    for gadget_weak in owner.gadgets.borrow().iter() {

        // `gadget_weak` is a `Weak<Gadget>`. Since `Weak` pointers can't
        // guarantee the allocation still exists, we need to call
        // `upgrade`, which returns an `Option<Rc<Gadget>>`.
        //
        // In this case we know the allocation still exists, so we simply
        // `unwrap` the `Option`. In a more complicated program, you might
        // need graceful error handling for a `None` result.

        let gadget = gadget_weak.upgrade().unwrap();
        println!("Gadget {} owned by {}", gadget.id, gadget.owner.name);
    }
}
