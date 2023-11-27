module SpaceImage : sig
  type t

  val create : int -> int -> string -> t
  val items : t -> char list list
  val print : t -> unit
end = struct
  type t = { image : char list list; w : int }

  let create w h input =
    let chars = String.to_seq input in
    let chunker size (chunked, current) next =
      let next = next :: current in
      if List.length next = size then (List.rev next :: chunked, [])
      else (chunked, next)
    in
    {
      image = Seq.fold_left (chunker (w * h)) ([], []) chars |> fst |> List.rev;
      w;
    }

  let items t = t.image

  let print image =
    let layered =
      List.fold_left
        (fun layered layer ->
          List.mapi
            (fun i c -> if c = '1' || c = '0' then c else List.nth layer i)
            layered)
        (List.hd image.image) image.image
    in
    let print_chars i c =
      if i mod image.w = 0 then print_newline ();
      print_string (if c = '0' then " " else "█")
    in
    List.iteri print_chars layered;
    print_newline ()
end

let part_1 image =
  let items = SpaceImage.items image in
  let min =
    Lib.min_list_by_key
      (fun item -> List.filter (( = ) '0') item |> List.length)
      (List.hd items) items
  in
  let ones = List.filter (( = ) '1') min |> List.length in
  let twos = List.filter (( = ) '2') min |> List.length in
  Printf.printf "Ones: %d\nTwos: %d\n%s\n" ones twos
  @@ String.init (List.length min) (List.nth min);
  ones * twos

let () =
  let input = Lib.get_input_lines "08" |> List.hd in
  let image = SpaceImage.create 25 6 input in
  Printf.printf "Part 1: %d\n" @@ part_1 image;
  Printf.printf "Part 2: \n";
  SpaceImage.print image;
  ()
