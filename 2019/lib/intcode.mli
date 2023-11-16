type t

val create : int list -> t
val execute : t -> t
val return_code : t -> int
val opcodes : t -> unit

(** [replace_value index value intcode] replaces the value at [index] with [value] in [intcode] *)
val replace_value : int -> int -> t -> t
