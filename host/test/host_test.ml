open Noematic

let yojson = Alcotest.testable Yojson.Safe.pp Yojson.Safe.equal
let write_request oc request = Protocol.write oc request
let read_response ic = Protocol.read ic (Protocol.read_length ic)
let noematic_exe = Exe.path
let correlation_id = `String "218ecc9f-a91a-4b55-8b50-2b6672daa9a5"

let test_save () =
  let stdin, stdout = Unix.open_process (noematic_exe ^ " -test") in
  let request =
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
        correlationId = [%aq correlation_id];
      }]
  in
  write_request stdout request;
  let expected =
    [%yojson
      {
        version = "0.1.0";
        action = "saveResponse";
        payload = [%aq `Null];
        correlationId = [%aq correlation_id];
      }]
  in
  let actual = read_response stdin in
  ignore (Unix.close_process (stdin, stdout));
  Alcotest.(check yojson) "same response" expected actual

let test_search () =
  let stdin, stdout = Unix.open_process (noematic_exe ^ " -test") in
  let save_request =
    [%yojson
      {
        version = "0.1.0";
        action = "saveRequest";
        payload =
          {
            url = "https://en.wikipedia.org/wiki/Foobar";
            title = "Title";
            innerText = "Foo bar baz quux";
          };
        correlationId = [%aq correlation_id];
      }]
  in
  write_request stdout save_request;
  let _ = read_response stdin in
  let search_request =
    [%yojson
      {
        version = "0.1.0";
        action = "searchRequest";
        payload = { query = "quux"; pageNum = 0; pageLength = 10 };
        correlationId = [%aq correlation_id];
      }]
  in
  write_request stdout search_request;
  let expected_header =
    [%yojson
      {
        version = "0.1.0";
        action = "searchResponseHeader";
        payload = { query = "quux"; pageNum = 0; pageLength = 1; hasMore = false };
        correlationId = [%aq correlation_id];
      }]
  in
  let actual_header = read_response stdin in
  Alcotest.(check yojson) "same response" expected_header actual_header;
  let expected_result =
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
        correlationId = [%aq correlation_id];
      }]
  in
  let actual_result = read_response stdin in
  Alcotest.(check yojson) "same response" expected_result actual_result;
  ignore (Unix.close_process (stdin, stdout))

let test_search_quotation () =
  let stdin, stdout = Unix.open_process (noematic_exe ^ " -test") in
  let save_request =
    [%yojson
      {
        version = "0.1.0";
        action = "saveRequest";
        payload =
          {
            url = "https://en.wikipedia.org/wiki/Foobar";
            title = "Title";
            innerText = "foo bar baz quux";
          };
        correlationId = [%aq correlation_id];
      }]
  in
  write_request stdout save_request;
  let _ = read_response stdin in
  let search_request =
    [%yojson
      {
        version = "0.1.0";
        action = "searchRequest";
        payload = { query = "\"\"foo-\"***bar\"\""; pageNum = 0; pageLength = 10 };
        correlationId = [%aq correlation_id];
      }]
  in
  write_request stdout search_request;
  let expected_header =
    [%yojson
      {
        version = "0.1.0";
        action = "searchResponseHeader";
        payload = { query = "\"\"foo-\"***bar\"\""; pageNum = 0; pageLength = 1; hasMore = false };
        correlationId = [%aq correlation_id];
      }]
  in
  let actual_header = read_response stdin in
  Alcotest.(check yojson) "same response" expected_header actual_header;
  let expected =
    [%yojson
      {
        version = "0.1.0";
        action = "searchResponseSite";
        payload =
          {
            url = "https://en.wikipedia.org/wiki/Foobar";
            title = "Title";
            snippet = "<b>foo</b> <b>bar</b> baz quux";
          };
        correlationId = [%aq correlation_id];
      }]
  in
  let actual = read_response stdin in
  ignore (Unix.close_process (stdin, stdout));
  Alcotest.(check yojson) "same response" expected actual

let test_search_idempotent () =
  let stdin, stdout = Unix.open_process (noematic_exe ^ " -test") in
  (* Save *)
  let save_request =
    [%yojson
      {
        version = "0.1.0";
        action = "saveRequest";
        payload =
          {
            url = "https://en.wikipedia.org/wiki/Foobar";
            title = "Title";
            innerText = "Foo bar baz quux";
          };
        correlationId = [%aq correlation_id];
      }]
  in
  write_request stdout save_request;
  let _ = read_response stdin in
  (* Create search *)
  let search_request =
    [%yojson
      {
        version = "0.1.0";
        action = "searchRequest";
        payload = { query = "quux"; pageNum = 0; pageLength = 10 };
        correlationId = [%aq correlation_id];
      }]
  in
  (* First time *)
  write_request stdout search_request;
  let expected_header =
    [%yojson
      {
        version = "0.1.0";
        action = "searchResponseHeader";
        payload = { query = "quux"; pageNum = 0; pageLength = 1; hasMore = false };
        correlationId = [%aq correlation_id];
      }]
  in
  let actual_header = read_response stdin in
  Alcotest.(check yojson) "same response" expected_header actual_header;
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
        correlationId = [%aq correlation_id];
      }]
  in
  let actual = read_response stdin in
  Alcotest.(check yojson) "same response (first)" expected actual;
  (* Second time *)
  write_request stdout search_request;
  let actual_header = read_response stdin in
  Alcotest.(check yojson) "same response" expected_header actual_header;
  let actual = read_response stdin in
  Alcotest.(check yojson) "same response (second)" expected actual;
  ignore (Unix.close_process (stdin, stdout))

let test_remove () =
  let stdin, stdout = Unix.open_process (noematic_exe ^ " -test") in
  let save_request =
    [%yojson
      {
        version = "0.1.0";
        action = "saveRequest";
        payload =
          {
            url = "https://en.wikipedia.org/wiki/Foobar";
            title = "Title";
            innerText = "Foo bar baz quux";
          };
        correlationId = [%aq correlation_id];
      }]
  in
  write_request stdout save_request;
  let _ = read_response stdin in
  let remove_request =
    [%yojson
      {
        version = "0.1.0";
        action = "removeRequest";
        payload = { url = "https://en.wikipedia.org/wiki/Foobar" };
        correlationId = [%aq correlation_id];
      }]
  in
  write_request stdout remove_request;
  let expected_response =
    [%yojson
      {
        version = "0.1.0";
        action = "removeResponse";
        payload = [%aq `Null];
        correlationId = [%aq correlation_id];
      }]
  in
  let actual_response = read_response stdin in
  Alcotest.(check yojson) "same response" expected_response actual_response;
  let search_request =
    [%yojson
      {
        version = "0.1.0";
        action = "searchRequest";
        payload = { query = "quux"; pageNum = 0; pageLength = 10 };
        correlationId = [%aq correlation_id];
      }]
  in
  write_request stdout search_request;
  let expected_header =
    [%yojson
      {
        version = "0.1.0";
        action = "searchResponseHeader";
        payload = { query = "quux"; pageNum = 0; pageLength = 0; hasMore = false };
        correlationId = [%aq correlation_id];
      }]
  in
  let actual_header = read_response stdin in
  Alcotest.(check yojson) "same response" expected_header actual_header;
  ignore (Unix.close_process (stdin, stdout))

let tests =
  let open Alcotest in
  [
    ("Save", [ test_case "Basic" `Quick test_save ]);
    ( "Search",
      [
        test_case "Basic" `Quick test_search;
        test_case "Quotation" `Quick test_search_quotation;
        test_case "Idempotent" `Quick test_search_idempotent;
      ] );
    ("Remove", [ test_case "Remove" `Quick test_remove ]);
  ]

let () = Alcotest.run "Integration" tests
