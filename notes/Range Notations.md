## Range Notations
### General Notations
Interval Notations
- using parantheses `()` for exclusive endpoints
	- (a, b)
		- numbers between $a$ and $b$
		- not including $a$ and $b$  
- using square brackets `[]` for inclusive endpoints
	- [a, b]
		- number from $a$ to $b$
		- including $a$ and $b$
- combined as half-open `[)` for left-included and right excluded
	- `[a, b)`
	- numbers from $a$ inclusive
	- up to but not inclusing $b$

Set-Builder Notation
- use curly braces `{}` and inequalities
	- $\{ a \in \mathbb{N} | a \gt 0\}$ all natural numbers greater than zero

Combine Both
	- $ [a, b)  = \{x \in \mathbb{R} | a \le x \lt b \}$

### Dijkstra's Argument
Why numbering should start at zero
- using zero-indexed, half-open interval notation for ranges in programming 
	- `[0, N)` or $0 \le i \lt N$
- why half-open?
	- upper minus lower bound gives you the actual length of the range
	- adjacent intervals of natural numbers meet cleanly (no overlap, no gap)  
- why legt-inclusive and right-exclusive?
	- lower bound:
		- must take smallest natural number (0 or 1)
		- when exclusive, would take smaller than smallest -> ugly
	- upper bound:
		- exclusive to cleanly represent empty ranges, no overlap, no gap
		- if lower bound starts at 1 then upper bound N+1 is awkward for infinity

Conclusions
	- best, natural $0 \le i \lt N$
- this works for
	- empty sequences
	- for prefixes
	- for slicing
	- makes bounds & sizes trivial

Source
- https://www.cs.utexas.edu/~EWD/transcriptions/EWD08xx/EWD831.html