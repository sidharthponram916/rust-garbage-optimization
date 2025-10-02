# Rust Garbage Collection 
This project showcases different garbage collection methods built in Rust, including reference counting, mark-and-sweep, and stop-and-copy. It simulates how memory is allocated, referenced, and freed, providing a clear look at how these techniques manage stack and heap memory efficiently.

## Method Descriptions 

### `reference_counting(string_vec: Vec<String>) -> RefCountMem`
Simulates reference counting — a garbage collection strategy where each object keeps track of how many references point to it. When the count drops to 0, the object is freed.

### `reachable(stack: &Vec<Vec<u32>>, heap: &Vec<Option<(String, Vec<u32>)>>) -> Vec<u32>`
Finds all heap nodes that can be reached from any object currently referenced in the stack.

### `mark_and_sweep(mem: &mut Memory)`
Implements the mark-and-sweep garbage collection algorithm.

### `stop_and_copy(mem: &mut Memory, alive: u32)`
Implements the stop-and-copy garbage collection algorithm.

