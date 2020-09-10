# ABI of Maysick/Native

## Types

### Any (or variable type) and `nil`

### Integer (`int`)

### Boolean (`bool`)

### String (`string`)

## Reference counter

## Calling convention

Mostly, the calling convention of maysick complies with that of C.

### Mangling

Types of the arguments are trival from symbol names,
as they are generated as a request from the callee (like template functions).

#### Format

For example,

```
char_at(str: string, pos: int) -> int
```

will be converted into:

```
_mSi_i_char_at
```

First, `_m` is a prefix for maysick functions, followed by two argument types: `S` for `string`, and `i` for `int`.
Second, separated by `_`, the return type is `int` then a return symbol is `i`. Last, separated by `_` (again), the function name goes.

Types are converted into a single character: `S` for `string`, `i` for `integer`, `b` for `boolean`, `A` for `any`.
`nil` in the argument list is treated as `any` type, and that in the return value is treated as `void` or empty (``).

Because of the constraint on the code generator and the language spec, all return values are currently forced into `any` except intrinsics.
