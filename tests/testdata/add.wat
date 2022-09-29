(module
  (type $t0 (func (param i32) (result i32)))
  (func $module/test (type $t0) (param $0 i32) (result i32)
    local.get $0
    i32.const 1
    i32.add)
  (memory $memory 0)
  (export "test" (func $module/test))
  (export "memory" (memory 0)))
