## Count Completed Jobs

Make this compile!

The idea is the thread spawned on line 12 is completing jobs while the main thread is monitoring progress until 10 jobs are completed. Due to the sleep time difference between the spawned threads and the waiting threads, when you see 6 lines of "waiting..." and the program ends without timing out the playground, you've got it :)

<div class="hint">
  <code>Arc</code> is an Atomic Reference Counted pointer that allows safe shared access to <b>immutable</b> data. But we want to change the number of <code>jobs_completed</code>, so we'll need to also use another type that will only allow one thread to mutate the data at a time.
  Take a look at <a href ="https://doc.rust-lang.org/stable/book/ch16-03-shared-state.html#atomic-reference-counting-with-arct">this section of the book</a>
  and keep scrolling if you'd like more hints :)
</div>

<div class="hint">
  Do you now have an <code>Arc</code> <code>Mutex</code> <code>JobStatus</code> at the beginning of main? Like:

  
  <code>let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));</code>
  
  Similar to the code in the example in the book that happens after the text that says "We can use Arc\<T\> to fix this.".
  If not, give that a try!
  If you do and would like more hints, keep scrolling!!
</div>

<div class="hint">
  Make sure neither of your threads are holding onto the lock of the mutex while they are sleeping, since this will prevent the other thread from being allowed to get the lock. Locks are automatically released when they go out of scope.
</div>

<div class="hint">
  Ok, so, real talk, this was actually tricky for <i>me</i> to do too.
  And I could see a lot of different problems you might run into, so at this point I'm not sure which one you've hit :)
  Please see a few possible <a href="https://github.com/carols10cents/rustlings/issues/3 ">answers</a> -- mine is a little more complicated because I decided I wanted to see the number of jobs currently done when I was checking the status.

  Please open an issue if you're still running into a problem that these hints are not helping you with, or if you've looked at the sample answers and don't understand why they work and yours doesn't.

  If you've learned from the sample solutions, I encourage you to come back to this exercise and try it again in a few days to reinforce what you've learned :)
</div>
