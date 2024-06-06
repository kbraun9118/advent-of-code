open Lib

(* let () = *)
(*   let input = get_test_lines "09" |> List.hd in *)
(*   let intcode = IntCode.create input in *)
(*   let output = IntCode.execute intcode |> IntCode.output in *)
(*   print_endline (string_of_int output) *)

module InfArr = struct
  type 'a t = { mutable items : 'a array; creator: (int -> 'a) }

  let init len f = { items = Array.init len f; creator = f }

  let nth idx arr = 
    let len = Array.length arr.items in
    if idx < len then
      arr.items.(idx)
    else
      let appended_len = idx - len + 1 in
      let appended = Array.init appended_len arr.creator in
      arr.items <- Array.append arr.items appended;
      arr.items.(idx)
end
