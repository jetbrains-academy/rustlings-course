## Mastering the IDE: Introduce Variable Refactoring
In 1993 one developer wrote the code to compute which year it was ten, five and one year ago.

In 2021 another developer came across this code and noticed that it doesn't produce correct results anymore.

This developer decided to [refactor the code](https://en.wikipedia.org/wiki/Code_refactoring) 
and extract `1993` to a new variable called `year`. 
This way future developers also coming across this code will be able to fix it by changing only one place instead of changing three places.

Let's also refactor the code!

%IDE_NAME% has handy functionality called **"Introduce Variable Refactoring"** that allows you quickly do this.

Select any occurrence of `1993` and then either press &shortcut:IntroduceVariable; 
or choose *Refactor&rarr;Introduce Variable...* from the right-click menu. 

*Note*: you can also use &shortcut:IntroduceConstant; or *Refactor&rarr;Introduce Constant...* from the right-click menu
to create constants.