# scfia-lib

## API Concepts

- Scfia maintains weakrefs to all (active and retired) symbols
- Retiring and dropping is triggered by the respective `Drop` handlers

## Brainstorming
- tranformation function derivation will need to record changes to state and memory
	- can we use it to handle forks?
	- intercept all writes to struct members and memory cells
	- intercept all reads from struct members and memory, deliver unflushed write if exists
	- commit non-forks by applying all pending writes
	- handle forks by cloning the uncommi
	-> recording should be optional
	- derive transformation from pending writes
- allow arbitrary numberic types with `<T: Add + Sub + Ord + ...>`?

### Write Buffer
- have write buffer in step context
- struct access/memory access writes to WB, reads from WB with priority
- fork
	- asserts WB is present
	- clones the current state and WB
	- schedules continuation with clone, and an assert other the negated fork symbol
	- apply WB

## Problems
- Value drop handlers need mutable access to scfia
- scfia must be pinned or RCed, otherwise the value's pointer to scfia might be invalid
- if we use RCed scfia:
	- Value drops must not happen while scfia is mutborrowed
	- we could enforce that by
		- passing only &values
		- returning all create values




