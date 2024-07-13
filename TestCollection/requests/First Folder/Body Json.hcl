version     = "V1"
description = "Http request"
url         = "https://echo.nrjais.com"
method      = "GET"
queries = [
  {
    "name"  = "param"
    "value" = "first"
  },
  {
    "name"  = "param"
    "value" = "duplicate"
  },
  {
    "name"     = "another"
    "value"    = "second"
    "disabled" = true
  }
]
headers = [
  {
    "name"  = "header"
    "value" = "sample"
  },
  {
    "name"     = "header-another"
    "value"    = "sample"
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

//   body raw {
//     eq = <<__
// {
//   "test": "test"
// }
// __
//   }
}
