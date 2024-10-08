let yojson = Alcotest.testable Yojson.Safe.pp Yojson.Safe.equal

module type JSON = sig
  type t

  val t_of_yojson : Yojson.Safe.t -> t
  val yojson_of_t : t -> Yojson.Safe.t
end

let make_roundtrips (module M : JSON) (name : string) (cases : (string * Yojson.Safe.t) list) =
  let f (name, ex) =
    let thunk () = Alcotest.(check yojson) "same json" ex M.(t_of_yojson ex |> yojson_of_t) in
    Alcotest.test_case (Printf.sprintf "roundtrip %s" name) `Quick thunk
  in
  (name, List.map f cases)

let roundtrips_request =
  make_roundtrips
    (module Noematic.Message.Request)
    "Request"
    [
      ( "saveRequest",
        [%yojson
          {
            version = "0.1.0";
            action = "saveRequest";
            payload =
              {
                url = "https://en.wikipedia.org/wiki/Foobar";
                title = "Title";
                innerText = "Inner text";
              };
            correlationId = "218ecc9f-a91a-4b55-8b50-2b6672daa9a5";
          }] );
      ( "searchRequest",
        [%yojson
          {
            version = "0.1.0";
            action = "searchRequest";
            payload = { query = "quux"; pageNum = 0; pageLength = 10 };
            correlationId = "218ecc9f-a91a-4b55-8b50-2b6672daa9a5";
          }] );
    ]

let roundtrips_response =
  make_roundtrips
    (module Noematic.Message.Response)
    "Response"
    [
      ( "saveResponse",
        [%yojson
          {
            version = "0.1.0";
            action = "saveResponse";
            payload = [%aq `Null];
            correlationId = "218ecc9f-a91a-4b55-8b50-2b6672daa9a5";
          }] );
      ( "searchResponseHeader",
        [%yojson
          {
            version = "0.1.0";
            action = "searchResponseHeader";
            payload = { query = "quux"; pageNum = 0; pageLength = 10; hasMore = true };
            correlationId = "218ecc9f-a91a-4b55-8b50-2b6672daa9a5";
          }] );
      ( "searchResponseSite",
        [%yojson
          {
            version = "0.1.0";
            action = "searchResponseSite";
            payload =
              {
                url = "https://en.wikipedia.org/wiki/Foobar";
                title = "Title";
                snippet = "Foo bar baz <b>quux</b>";
              };
            correlationId = "218ecc9f-a91a-4b55-8b50-2b6672daa9a5";
          }] );
    ]

let () = Alcotest.run "Message" [ roundtrips_request; roundtrips_response ]
