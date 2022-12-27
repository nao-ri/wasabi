(module
  (type (;0;) (func (param i32)))
  (type (;1;) (func (param i32 i32)))
  (type (;2;) (func (param i32 i32 i32)))
  (type (;3;) (func (param i32 i32 i32 i32)))
  (type (;4;) (func))
  (import "imports" "output" (func (;0;) (type 0)))
  (import "__wasabi_hooks" "start" (func (;1;) (type 1)))
  (import "__wasabi_hooks" "begin_function" (func (;2;) (type 1)))
  (import "__wasabi_hooks" "i32_const" (func (;3;) (type 2)))
  (import "__wasabi_hooks" "call_i" (func (;4;) (type 3)))
  (import "__wasabi_hooks" "call_post" (func (;5;) (type 1)))
  (import "__wasabi_hooks" "return" (func (;6;) (type 1)))
  (import "__wasabi_hooks" "end_function" (func (;7;) (type 1)))
  (func (;8;) (type 4)
    (local i32)
    global.get 0
    if  ;; label = @1
      i32.const 0
      global.set 0
      i32.const 1
      i32.const -1
      call 1
    end
    i32.const 1
    i32.const -1
    call 2
    i32.const 42
    i32.const 1
    i32.const 0
    i32.const 42
    call 3
    local.tee 0
    i32.const 1
    i32.const 1
    i32.const 0
    local.get 0
    call 4
    call 0
    i32.const 1
    i32.const 1
    call 5
    i32.const 1
    i32.const -1
    call 6
    i32.const 1
    i32.const 2
    call 7)
  (global (;0;) (mut i32) (i32.const 1))
  (start 8))
