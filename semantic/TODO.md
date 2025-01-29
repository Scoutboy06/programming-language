# Declaration and scoping checks

- [ ] Variable Declarations: Ensure variables are declared before use, with valid types.
- [ ] Function Declarations: Validate function signatures (parameters, return type) and check for duplicates in the same scope.
- [ ] Name Resolution: Verify all identifiers (variables, functions, types) reference valid declarations.
- [ ] Scope Validity: Check variable accessibility within scopes (e.g., no outer scope access after shadowing).
- [ ] Shadowing: Allow/disallow variable shadowing based on language rules.

# Type checking

- [ ] Expression Types: Validate operand types for operators (e.g., no `object + array`)
- [ ] Assignments: Ensure the right-hand side type matches the left-hand side.
- [ ] Function Returns: Confirm return statements match the declared return type.
- [ ] Parameter Types: Check function call arguments match parameter types (arity and compatibility).
- [ ] Type Inference: Resolve inferred types (e.g., `let x = 10` → `x` is `number`)
- [ ] Implicit Conversions: Validate allowed coercions (e.g., `number` to `string` if permitted).

# Control Flow and Validity

- [ ] Unreachable Code: Detect code after unconditional returns/breaks.
- [ ] Loop Validity: Ensure `break`/`continue` are only inside loops.
- [ ] Exhaustive Returns: Check all code paths in non-void functions return a value

# Composite Types and Structures

- [ ] Field/Method access: Verify struct/class fields and methods exist and are visible (e.g., private/public).
- [ ] Inheritance: Check class hierarchies for cycles, method overrides (signature compatibility), and interface implementation.
- [ ] Enum Handling: Ensure `switch` statements exhaustively cover all enum variants (if required)

# Advanced Features

- [ ] Generics/Templates: Validate type parameters satisfy constraints (e.g., traits/interfaces).
- [ ] Operator Overloading: Check custom operator definitions have correct signatures
- [ ] Array/Collection Bounds: Statically validate constant indices (e.g. `arr[5]` for a size-3 array is invalid).
- [ ] Constant Expressions: Ensure compile-time constants (e.g. array sizes) are evaluable.

# Semantic Constraints

- [ ] Initialization Before Use: Variable must be assigned before access
- [ ] Cyclic Dependencies: Detect invalid circular type references
- [ ] Visibility Modifiers: Enforce access rules (e.g. private members only accessed within their scope).

# Function-Specific Checks

- [ ] Overloading: Resolve overloads uniquely (if supported) or disallow duplicates.
- [ ] Recursion: Allow recursive calls (no semantic issue, but track for stack overflow warnings if desired).

# Error Handling

- [ ] CUstom Error Types: Ensure user-defined errors are property propagated (if applicable).

# Miscellaneous

- [ ] Imports/Modules: Resolve cross-module references and visibility.
- [ ] Dead Code: Warn about unused variables/functions.