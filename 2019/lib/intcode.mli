type t

val create : ?argument:int -> string -> t
val execute : t -> t
val return_code : t -> int
val opcodes : t -> unit

val replace_value : int -> int -> t -> t
(** [replace_value index value intcode] replaces the value at [index] with [value] in [intcode] *)

val output : t -> int
