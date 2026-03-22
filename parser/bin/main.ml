(* Abstract type representing the Rust memory *)
type gap_buffer

(* Bindings *)
external create_buffer : unit -> gap_buffer = "create_buffer"
external move_cursor_left : gap_buffer -> unit = "move_cursor_left"
external move_cursor_right : gap_buffer -> unit = "move_cursor_right"

let () =
  let buf = create_buffer () in
    move_cursor_left buf;
    move_cursor_right buf;

let () = 