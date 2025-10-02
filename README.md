# Rust Garbage Collection 
This project is a test for managing stack and heap memory through Rust ownership roles, emulating garbage collection. 

## Method Descriptions 

### `reference_counting(string_vec: Vec<String>) -> RefCountMem`
Simulates reference counting — a garbage collection strategy where each object keeps track of how many references point to it. When the count drops to 0, the object is freed.

### `reachable(stack: &Vec<Vec<u32>>, heap: &Vec<Option<(String, Vec<u32>)>>) -> Vec<u32>`
Finds all heap nodes that can be reached from any object currently referenced in the stack.

### `mark_and_sweep(mem: &mut Memory)`
Implements the mark-and-sweep garbage collection algorithm.

### `stop_and_copy(mem: &mut Memory, alive: u32)`
Implements the stop-and-copy garbage collection algorithm.

