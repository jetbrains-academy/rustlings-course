## String Transformer

Let's build a little machine in form of a function.
As input, we're going to give a list of strings and commands. These commands
determine what action is going to be applied to the string. It can either be:
 - Uppercase the string
 - Trim the string
 - Append "bar" to the string a specified amount of times

The exact form of this will be:
 - The input is going to be a Vector of a 2-length tuple,
   the first element is the string, the second one is the command.
 - The output element is going to be a Vector of strings.
