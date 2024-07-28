version = "V1"
description = "Http request"
url = "https://echo.nrjais.com/:path/{{test}}"
method = "POST"
params = [
  {
    "name" = "path"
    "value" = "from_path"
  }
]
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
        "path" = "/Users/neeraj/projects/sanchaar/TestCollection/collection.hcl"
      }
    ]
  }
}

assertions {
  status code {
    eq = 201
  }

  duration ms {
    gt = 1000
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
