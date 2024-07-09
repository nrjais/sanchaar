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
  status {
    gt = 200
    lt = 200
  }

  status {
    lt = 200
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

/*
{
  "body": {
    "eq": "{\n  \"test\": \"test\n}\n"
  },
  "header": {
    "Content-Type": [
      {
        "contains": "application/json"
      },
      {
        "eq": "utf8"
      }
    ]
  },
  "jsonpath": {
    "$.result.name": {
      "equal": "Mohit"
    }
  },
  "status": {
    "gt": 200
  }
}
*/
