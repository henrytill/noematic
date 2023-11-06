open Brr

let () =
  Console.log [ Jstr.v "Hello from the running script!" ];
  El.set_children (Document.body G.document) El.[ txt' "Hello, world!" ]
