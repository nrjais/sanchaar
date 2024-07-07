meta {
  version = "V1"
  description = "Http request"
}

method = "POST"
url = "https://echo.nrjais.com/{{test}}"

queries {
  test = "{{test}}"
}

body {
  form {
    multipart = "text value"
  }

  files {
    file = "/Users/neeraj/projects/sanchaar/TestCollection/collection.toml"
  }
}

pre_request = "test_pre.js"
