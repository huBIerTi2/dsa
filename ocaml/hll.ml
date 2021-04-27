open Core
open Bigarray

type t =
  {
    regs: (char, int8_unsigned_elt, c_layout) Array1.t;
    zeros: int ref;
    b: int;
  }

let approximate_error size = 1.04 /. (sqrt @@ Float.of_int64 size)

let (min_b, max_b) = (6, 21)

let range a b =
  Array.init (b - a + 1) ~f:(fun x -> x + a)

let get_error_test arr idx =
  approximate_error Int64.(pow 2L
                             (of_int @@ Array.get arr idx))

let findsize error =
  let get_error arr idx =
    approximate_error Int64.(pow
                               (of_int 2)
                               (of_int @@ Array.get arr idx))
  in
  Binary_search.binary_search
    ~compare:(fun x key -> Float.compare key x)
    ~length:Array.length
    ~get:get_error
    (range min_b max_b)
    `First_greater_than_or_equal_to
    error

let init maxerr =
  match findsize maxerr with
  | Some sz ->
    let arr = Array1.create char C_layout sz in
    Some { regs = arr; zeros = ref 0; b = sz }
  | None -> None

let mask len =
  Int64.((1L lsl len) - 1L)

let add hll h =
  let b = hll.b in
  let addr =
    (* An exception should never be thrown as long as max_b is lower than 32 *)
    Int64.(to_int_exn @@
           (mask b) land (h lsl (Int.(-) 64 b)))
  in
  let cval = Char.to_int hll.regs.{addr} in
  let score =
    1 + Int64.(clz (shift_left h b)) in
  if cval = 0
  then decr hll.zeros;
  hll.regs.{addr} <- Char.of_int_exn (max cval score)

let cardinality hll =
