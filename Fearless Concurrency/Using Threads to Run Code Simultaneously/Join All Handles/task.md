## Join All Handles

A challenge with multi-threaded applications is that the main thread can
finish before the spawned threads are completed. The `thread::spawn` function returns the `JoinHandle` struct.
Collect the JoinHandles and wait for them to finish.

<div class="hint">
Push the value you've got from the <code>thread::spawn</code> function into the
<code>handles</code> vector.
</div>

<div class="hint">
To join the thread, you have to call <code>join</code> on the corresponding 
<code>JoinHandle</code> and then <code>unwrap</code> the result.
</div>