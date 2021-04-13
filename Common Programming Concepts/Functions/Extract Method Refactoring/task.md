## Mastering the IDE: Extract Method Refactoring
 
One developer has been tasked to change the message format in the code you see in this task.

Instead of "10 years ago in 1993: 1983" we want to print "Ten years ago in 1993: 1983".

However, the way it's implemented now the developer has to modify the message in two places which would 
become more inconvenient when this code is used in more places.

So before actually changing the message format, the developer decided to perform refactoring and create a function that prints the message.

###Task

**Step 1: Create Function**

Perform the refactoring and create a new function to replace the duplicated code.

Select the first occurrence of 

```rust
println!("Ten years ago in {}: {}", year1993, year1993 - 10);
```

and then either press &shortcut:ExtractMethod; or choose *Refactor&rarr;Extract Method...* from the right-click menu.

In the dialog that appears you can choose a more suitable name for parameter (for example, `year`):

![Image: 2020-unique-launches.png](refactoring.png)

**Step 2: Replace Duplicated Code**

After a new function is created replace the second occurrence of the code with the new function call.

**Step 3: Change the Text**

Finally, change "10" to "Ten" in the function body.

