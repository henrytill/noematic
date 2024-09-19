open Noematic.Message

let yojson = Alcotest.testable Yojson.Safe.pp Yojson.Safe.equal

module Uri_ext = struct
  let uri = Alcotest.testable Uri_ext.pp Uri_ext.equal

  let roundtrip () =
    let expected = Uri_ext.of_string "https://www.archive.org/" in
    Alcotest.(check uri) "same uri" expected Uri_ext.(yojson_of_t expected |> t_of_yojson)
end

module Version = struct
  let version = Alcotest.testable Version.pp Version.equal

  let roundtrip () =
    Alcotest.(check version)
      "same version"
      Version.expected
      Version.(yojson_of_t expected |> t_of_yojson)
end

module Request = struct
  let roundtrip_save_request () =
    let expected =
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
        }]
    in
    Alcotest.(check yojson) "same json" expected Request.(t_of_yojson expected |> yojson_of_t)

  let roundtrip_search_request () =
    let expected =
      [%yojson
        {
          version = "0.1.0";
          action = "searchRequest";
          payload = { query = "quux"; pageNum = 0; pageLength = 10 };
          correlationId = "218ecc9f-a91a-4b55-8b50-2b6672daa9a5";
        }]
    in
    Alcotest.(check yojson) "same json" expected Request.(t_of_yojson expected |> yojson_of_t)
end

module Response = struct
  let roundtrip_save_response () =
    let expected =
      [%yojson
        {
          version = "0.1.0";
          action = "saveResponse";
          payload = [%aq `Null];
          correlationId = "218ecc9f-a91a-4b55-8b50-2b6672daa9a5";
        }]
    in
    Alcotest.(check yojson) "same json" expected Response.(t_of_yojson expected |> yojson_of_t)

  let roundtrip_search_response_header () =
    let expected =
      [%yojson
        {
          version = "0.1.0";
          action = "searchResponseHeader";
          payload = { query = "quux"; pageNum = 0; pageLength = 10; hasMore = true };
          correlationId = "218ecc9f-a91a-4b55-8b50-2b6672daa9a5";
        }]
    in
    Alcotest.(check yojson) "same json" expected Response.(t_of_yojson expected |> yojson_of_t)

  let roundtrip_search_response_site () =
    let expected =
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
        }]
    in
    Alcotest.(check yojson) "same json" expected Response.(t_of_yojson expected |> yojson_of_t)
end

let tests =
  let open Alcotest in
  [
    ("Uri_ext", [ test_case "roundtrip" `Quick Uri_ext.roundtrip ]);
    ("Version", [ test_case "roundtrip" `Quick Version.roundtrip ]);
    ( "Request",
      [
        test_case "roundtrip saveRequest" `Quick Request.roundtrip_save_request;
        test_case "roundtrip searchRequest" `Quick Request.roundtrip_search_request;
      ] );
    ( "Response",
      [
        test_case "roundtrip saveResponse" `Quick Response.roundtrip_save_response;
        test_case "roundtrip searchResponseHeader" `Quick Response.roundtrip_search_response_header;
        test_case "roundtrip searchResponseSite" `Quick Response.roundtrip_search_response_site;
      ] );
  ]

let () = Alcotest.run "Message" tests
