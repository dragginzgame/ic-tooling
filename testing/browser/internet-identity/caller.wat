;; An authenticated ingress proof: return the caller bytes actually observed by IC.
;; This fixture owns no application encoding or authentication policy.
(module
  (import "ic0" "msg_caller_size" (func $size (result i32)))
  (import "ic0" "msg_caller_copy" (func $copy (param i32 i32 i32)))
  (import "ic0" "msg_reply_data_append" (func $append (param i32 i32)))
  (import "ic0" "msg_reply" (func $reply))
  (memory (export "memory") 1)
  (func (export "canister_update caller")
    (call $copy (i32.const 0) (i32.const 0) (call $size))
    (call $append (i32.const 0) (call $size))
    (call $reply)))
