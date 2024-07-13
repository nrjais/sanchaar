version     = "V1"
description = "Http request"
method      = "POST"
url         = "https://echo.nrjais.com/{{test}}"
queries = [
  {
    "name"  = "test"
    "value" = "{{test}}"
  }
]

body {
  multipart {
    params = [
      {
        "name"  = "multipart"
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

assertions {
  status code {
    gt = 200
    lt = 200
  }

  duration ms {
    gt = 100
    lt = 100
  }

  header "Content-Type" {
    contains = "application/json"
  }

  header "Content-Type" {
    contains = "utf8"
  }

  header "Test-Type" {
    contains = "utf8"
  }

  // jsonpath "$.result.name" {
  //   eq = "Mohit"
  // }

  body raw {
    eq = <<__
{
  "test": "test"
}
__
  }
}
