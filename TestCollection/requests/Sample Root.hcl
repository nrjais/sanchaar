version = "V1"
description = "Http request"
method = "POST"
url = "https://echo.nrjais.com/{{test}}"
queries = [
  {
    "name" = "test"
    "value" = "{{test}}"
  }
]

body {
  multipart {
    params = [
      {
        "name" = "multipart"
        "value" = "text value"
      }
    ]
    files = [
      {
        "name" = "file"
        "path" = "/Users/neeraj/projects/sanchaar/TestCollection/collection.toml"
      }
    ]
  }
}

pre_request = "test_pre.js"
