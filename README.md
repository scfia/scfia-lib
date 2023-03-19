# scfia-lib

## API Brainstorming

- Symbols SHOULD be accessible via their ids later
	- this makes adding constraints and comparing states (that share symbols) easier
    -> HashMap/BTree within $struct
- Struct Hierarchy:
    - `RV32I`
        - `SystemState`
			- ActiveValue as struct member
			- implements `step` and `reset`
				- both may create new symbols and permutate memory
				-> need access to `Memory` and `SymbolManager`
					- mutable borrows could suffice
        - `Memory`
			- implements `read` and `write`
			- may yield fresh symbols (if volatile regions are read)
			- may combine or split symbols (if partial symbols are read)
			-> needs access to `SymbolManager`
				- mutable borrow could suffice
        - `SymbolManager`
			- manages symbol ids
			- manages z3 context
- Symbols need to be refcounted, either manually or by Rc<...>
	- Proposal 1:
		- `active_values: HashMap<u64, Weak<RefCell<ActiveValue>>>` in `SymbolManager`
		- `retired_values: HashMap<u64, Weak<RefCell<RetiredValue>>>` in `SymbolManager`
		- Refcounting is done by Rc


