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
        "path" = "/Users/neeraj/projects/sanchaar/TestCollection/collection.hcl"
      }
    ]
  }
}

assertions {
  status code {
    eq = 200
  }

  duration ms {
    gt = 100
    lt = 1000
  }

  header "Content-Type" {
    contains = "application/json"
  }

  header "Content-Type" {
    contains = "json"
  }

  // jsonpath "$.result.name" {
  //   eq = "Mohit"
  // }

  body string {
    contains = "echo.nrjais.com"
  }
}
