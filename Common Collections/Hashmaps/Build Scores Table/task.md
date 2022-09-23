## Build Scores Table

Given is a list of scores (one per line) of a soccer match. Each line 
is of the form:
```
<team_1_name>,<team_2_name>,<team_1_goals>,<team_2_goals>
```

Example: `England,France,4,2` (England scored 4 goals, France 2).

You have to build a scores table containing the name of the team, goals
the team scored, and goals the team conceded. One approach to build
the scores table is to use a Hashmap. The solution is partially
written to use a Hashmap, complete it to pass the test.

Make me pass the tests!

<div class="hint">
Use the <code>entry()</code> and <code>or_insert()</code> methods of <code>HashMap</code> to insert entries corresponding to each team in the scores table.

Learn more in [The Book](https://doc.rust-lang.org/stable/book/ch08-03-hash-maps.html#only-inserting-a-value-if-the-key-has-no-value).
</div>


<div class="hint">
If there is already an entry for a given key, the value returned by <code>entry()</code> can be updated based on the existing value.

Learn more in [The Book](https://doc.rust-lang.org/book/ch08-03-hash-maps.html#updating-a-value-based-on-the-old-value).
</div>
