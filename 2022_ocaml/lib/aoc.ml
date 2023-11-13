module IntTuple = struct
  type t = int * int

  let compare (left : t) (right : t) = Stdlib.compare left right
end

module IntTupleSet = Set.Make (IntTuple)

type direction = Up | Down | Left | Right

let rec remove_last list =
  match list with
  | [] -> []
  | [ ""; _ ] -> []
  | [ _ ] -> []
  | x :: xs -> x :: remove_last xs

let read_lines file =
  let contents = In_channel.with_open_bin file In_channel.input_all in
  let split = String.split_on_char '\n' contents in
  remove_last split

let rec window length list =
  if List.length list < length then
    failwith "Cannot window with smaller length than list length"
  else if List.length list = length then [ list ]
  else
    let rec build remaining list =
      if remaining = length then []
      else
        let nth = List.nth list remaining in
        let built = build (remaining + 1) list in
        nth :: built
    in
    build 0 list :: window length (List.tl list)

let chunk length list =
  let rec chunk chunks current length = function
    | [] -> List.rev (List.rev current :: chunks)
    | hd :: tl ->
        let chunks, next =
          if List.length current = length then
            (List.rev current :: chunks, [ hd ])
          else (chunks, hd :: current)
        in
        chunk chunks next length tl
  in
  chunk [] [] length list

let split_once sep string =
  match String.split_on_char sep string with
  | left :: right -> (left, String.concat (String.make 1 sep) right)
  | _ -> failwith "No separator found"

let contains_substring main_string substring =
  let len_main = String.length main_string in
  let len_sub = String.length substring in
  if len_sub > len_main then false
  else
    let rec check_pos pos =
      if pos + len_sub > len_main then false
      else if String.sub main_string pos len_sub = substring then true
      else check_pos (pos + 1)
    in
    check_pos 0
