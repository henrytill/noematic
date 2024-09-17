open Noematic.Message

let%expect_test "Uri_ext.yojson_of_t" =
  Uri_ext.(of_string "https://www.archive.org/" |> yojson_of_t)
  |> Yojson.Safe.to_string
  |> print_endline;
  [%expect {| "https://www.archive.org/" |}]

let%expect_test "Version.pp" =
  Version.(expected |> pp Format.std_formatter);
  [%expect {| (0, 1, 0) |}]

let%expect_test "Version.yojson_of_t" =
  Version.(yojson_of_t expected) |> Yojson.Safe.to_string |> print_endline;
  [%expect {| "0.1.0" |}]

let%expect_test "Response.Save.yojson_of_t" =
  Response.Save.yojson_of_t () |> Yojson.Safe.to_string |> print_endline;
  [%expect {| null |}]

let%expect_test "saveRequest" =
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
  Request.(t_of_yojson expected |> yojson_of_t) |> Yojson.Safe.to_string |> print_endline;
  [%expect
    {| {"version":"0.1.0","action":"saveRequest","payload":{"url":"https://en.wikipedia.org/wiki/Foobar","title":"Title","innerText":"Inner text"},"correlationId":"218ecc9f-a91a-4b55-8b50-2b6672daa9a5"} |}]

let%expect_test "searchRequest" =
  let expected =
    [%yojson
      {
        version = "0.1.0";
        action = "searchRequest";
        payload = { query = "quux" };
        correlationId = "218ecc9f-a91a-4b55-8b50-2b6672daa9a5";
      }]
  in
  Request.(t_of_yojson expected |> yojson_of_t) |> Yojson.Safe.to_string |> print_endline;
  [%expect
    {| {"version":"0.1.0","action":"searchRequest","payload":{"query":"quux"},"correlationId":"218ecc9f-a91a-4b55-8b50-2b6672daa9a5"} |}]

let%expect_test "saveResponse" =
  let expected =
    [%yojson
      {
        version = "0.1.0";
        action = "saveResponse";
        payload = [%aq `Null];
        correlationId = "218ecc9f-a91a-4b55-8b50-2b6672daa9a5";
      }]
  in
  Response.(t_of_yojson expected |> yojson_of_t) |> Yojson.Safe.to_string |> print_endline;
  [%expect
    {| {"version":"0.1.0","action":"saveResponse","payload":null,"correlationId":"218ecc9f-a91a-4b55-8b50-2b6672daa9a5"} |}]

let%expect_test "searchResponse" =
  let expected =
    [%yojson
      {
        version = "0.1.0";
        action = "searchResponse";
        payload =
          {
            query = "quux";
            results =
              [
                {
                  url = "https://en.wikipedia.org/wiki/Foobar";
                  title = "Title";
                  snippet = "Foo bar baz <b>quux</b>";
                };
              ];
          };
        correlationId = "218ecc9f-a91a-4b55-8b50-2b6672daa9a5";
      }]
  in
  Response.(t_of_yojson expected |> yojson_of_t) |> Yojson.Safe.to_string |> print_endline;
  [%expect
    {| {"version":"0.1.0","action":"searchResponse","payload":{"query":"quux","results":[{"url":"https://en.wikipedia.org/wiki/Foobar","title":"Title","snippet":"Foo bar baz <b>quux</b>"}]},"correlationId":"218ecc9f-a91a-4b55-8b50-2b6672daa9a5"} |}]
