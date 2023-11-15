# Week 4

## practice 1

Implement BFS, DFS, and traverse in ```algo``` module with the ```Node``` type with implemented in class

## practice 2

Create a pool that can run multi-thread jobs with limited workers

Each worker runs on job at a time

Keep finding unfinished jobs and assign to idle worker

## Some keypoints I didn't mentioned...

### Send and Sync trait

This two traits are used to mark the capability of safety across thread

Types with ```Send``` implemented is safe to be moved to another thread, for example, ```Arc``` and ```Mutex```, while ```Rc``` is not

```Sync``` means that the type is safe to be accessed from different threads, ```Mutex``` is the simplest example

## Annotation of trait bound of Fn, FnMut, FnOnce

When using generic on closure, the input and output type of the closure is written in trait bound, for example

```rust
fn run<F>(f: F, val: i32)
where
  F: Fn(i32) -> String  // takes i32 and return String
{
  run(val)
}
```
