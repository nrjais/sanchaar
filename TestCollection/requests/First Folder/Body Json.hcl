sanchaar {
  version = "V1"
  description = "Http request"
}

method = "GET"
url = "https://echo.nrjais.com"

queries {
  param = "first"
  another = {
    "disabled" = true
    "value" = "second\""
  }
}

headers {
  header = "sample"
  header-another = {
    "disabled" = true
    "value" = "sample"
  }
}

body {
  json = <<_
{
"test": "hello"
}
_
}
