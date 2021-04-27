(* open Core *)
(* open Bigarray *)

type t
val approximate_error : int64 -> float

val min_b : int
val max_b : int

val get_error_test : int array -> int -> float

val findsize : float -> int option
val init : float -> t option
val add : t -> int64 -> unit
