val get_input_lines : string -> string list
val get_test_lines : string -> string list
val split_once : char -> string -> string * string
val min_list_by_key : ('a -> 'b) -> 'a -> 'a list -> 'a

module IntCode = Intcode
