let get_lines file = 
  let lines = In_channel.input_lines @@ In_channel.open_text file in
  let rec remove_end_newline lines = 
    match lines with
    | hd :: ""  :: [] -> [hd]
    | [] -> []
    | hd :: tl -> hd :: remove_end_newline tl
  in
  remove_end_newline lines

let get_input_lines day = get_lines @@ Printf.sprintf "./input/day-%s/input.txt" day

let get_test_lines day = get_lines @@ Printf.sprintf "./input/day-%s/test.txt" day

let split_once delimiter input = 
  match String.split_on_char delimiter input with 
  | left :: right :: [] -> left, right
  | _ -> failwith ("Cannot split once: " ^ input)
