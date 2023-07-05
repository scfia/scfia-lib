## ScfiaComposition

## Value Lifetimes
Values are concrete or expressions.
A value is considered **active** if it is directly referenced by a register, memory cell, or expression.
Every value must live as long as it is active or might have an impact on active symbols.

Values might have an impact on their parents:
If `a <= 5` has been asserted, the less-than expression and `5` must live as long as `a`.
Values might also have an impact on **connected** symbols:
If `b == a` and `b > 4` exist and have been asserted `a`, `b`, `a <= 5`, `b == a`, and `b > 4` must live as long as each other.

To satisfy the these requirements, scfia-lib converts active values into **inactive** values when they go out of scope, and all affected values **inherit** them.
Parent values are connected when an active expression over them becomes inactive.
All connected values inherit the new inactive value, all inactive values the value has had, and all connections.

## Value Implementation
For performance reasons, concrete calculations are conducted without calling into libz3 and without heap allocations.

### Active Values
Therefore, `ActiveValue` is an enum over `BoolConcrete`, `BVConcrete`, and `Rc<RefCell<ActiveValueExpression<SC>>>`.
`BoolConcrete` and `BVConcrete` are fully constrained, do not impose lifetime requirements, and thus don't own anything.
`ActiveValueExpression`s take ownership of their parents and `InactiveValue`s, and weak references to their connected `ActiveValueExpression`s.

Whenever an expression over a concrete `ActiveValue` is constructed, a new one `Z3Ast` is created in `ActiveValue::get_z3_ast`.
TODO how do we take ownership of a concrete parent's AST?
- have ActiveValue::BVConcrete and ActiveValueExpression::BVConcrete, where the latter owns the ast?
    - all expressions need ActiveValueExpression parents then
    - the way from ActiveValueExpression to ActiveValue is trivial
- handle this within Z3Ast? This would require RCed Z3Asts (either in rust, or through z3 while we keep a clone alive)

### Inactive Values
When an `ActiveValueExpression` goes out of scope, it is converted into an `InactiveValueExpression`.
Concrete parents are attached to the new `InactiveValueExpression`, because they have to outlive their children.
When an `InactiveValueExpression` is cloned to a new scfia, concrete parents are created first.

### Constructing new ActiveValues

### Cloning to new Scfias
To prevent duplicate clones, BTreeMaps are passed through the recursive cloning process.
Before a value is cloned, all parents are cloned.
After a value is cloned, all inherited and discovered values are cloned and given to the clone.
