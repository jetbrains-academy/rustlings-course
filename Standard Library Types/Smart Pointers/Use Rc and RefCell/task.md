## Switch the Lamp

In this task, you have a single lamp, and several external switchers which are able to operate on the lamp. To make this happen we need both `Rc` and `RefCell` smart pointers. `Rc` allows us to have multiple owners, and `RefCell` provides mutability with `borrow_mut`. 

We'd like to implement the following scenario:

* create a lamp
* create a switcher for a lamp 
* create another switcher for the same lamp 
* operate on different switchers and make sure that the lamp is on, and then off again

In fact, this scenario is already implemented in `main.rs`. Edit the `switchers` module by following the hints there to make the code compile.

<div class="hint">Read the main function to understand which type should we use for <code>lamp</code> in <code>Switcher</code>.</div>

<div class="hint">Note the <code>&</code> in the call to <code>Switcher::new</code>.</div>

<div class="hint">To clone the reference, you may need <code>Rc::clone</code>.</div>

<div class="hint"><code>RefCell</code> gives you <code>borrow</code> and <code>borrow_mut</code> methods. When should you use them?</div>
