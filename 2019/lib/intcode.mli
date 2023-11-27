type t

val create : ?argument:int -> string -> t
val push_input : int -> t -> t
val clear_output : t -> t
val is_complete : t -> bool
val execute_until_output : t -> t
val execute : t -> t
val return_code : t -> int
val opcodes : t -> unit

val replace_value : int -> int -> t -> t
(** [replace_value index value intcode] replaces the value at [index] with [value] in [intcode] *)

val output : t -> int
