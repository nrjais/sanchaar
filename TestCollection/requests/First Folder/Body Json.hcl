version = "V1"
description = "Http request"
method = "GET"
url = "https://echo.nrjais.com"
queries = [
  {
    "name" = "param"
    "value" = "first"
  },
  {
    "name" = "param"
    "value" = "duplicate"
  },
  {
    "name" = "another"
    "value" = "second"
    "disabled" = true
  }
]
headers = [
  {
    "name" = "header"
    "value" = "sample"
  },
  {
    "name" = "header-another"
    "value" = "sample"
    "disabled" = true
  }
]

body {
  json = <<_
{
"test": "hello"
}
_
}
