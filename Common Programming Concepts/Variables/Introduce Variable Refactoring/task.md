## Mastering the IDE: Introduce Variable Refactoring
In 1993, a developer wrote a code to compute what year it was ten, five, and one year before.

In 2021, another developer came across this code and noticed that it didn't produce correct results anymore.

This developer decided to [refactor the code](https://en.wikipedia.org/wiki/Code_refactoring) 
and extract `1993` to a new variable called `year`. 
This way, future developers coming across this code would be able to fix it by changing only one place instead of three places.

Let's also refactor the code!

%IDE_NAME% has a handy functionality called **"Introduce Variable Refactoring"**, which allows you to quickly do this.

Select any occurrence of `1993` and then either press &shortcut:IntroduceVariable; 
or choose *Refactor -> Introduce Variable...* from the right-click menu. 

*Note*: you can also use &shortcut:IntroduceConstant; or *Refactor -> Introduce Constant...* from the right-click menu
to create constants.