package main

import (
	"encoding/base64"
	"os"
	"unicode/utf8"

	"github.com/gofiber/fiber/v2"
	"github.com/gofiber/fiber/v2/middleware/cors"
)

type EchoResponse struct {
	Method  string            `json:"method"`
	Host    string            `json:"host"`
	Path    string            `json:"path"`
	Queries map[string]string `json:"queries"`
	Headers map[string]string `json:"headers"`

	Body     string `json:"body"`
	BodyType string `json:"bodyType"`
}

func main() {
	app := fiber.New()

	// CORS: Allow all origins, all headers, all methods
	app.Use(cors.New(cors.Config{
		AllowOrigins:     "*",
		AllowHeaders:     "*",
		AllowMethods:     "*",
		ExposeHeaders:    "*",
		AllowCredentials: true,
	}))

	app.Use("", func(c *fiber.Ctx) error {
		body, bodyType := readBody(c)
		return c.JSON(EchoResponse{
			Method:   c.Method(),
			Host:     c.Hostname(),
			Path:     c.Path(),
			Queries:  c.Queries(),
			Headers:  flatten(c.GetReqHeaders()),
			Body:     body,
			BodyType: bodyType,
		})
	})

	port := ":3003"
	if os.Getenv("PORT") != "" {
		port = ":" + os.Getenv("PORT")
	}

	app.Listen(port)
}

func readBody(c *fiber.Ctx) (string, string) {
	raw := c.BodyRaw()
	if !utf8.Valid(raw) {
		return base64.StdEncoding.EncodeToString(raw), "base64"
	}
	return string(raw), "text"
}

func flatten(headers map[string][]string) map[string]string {
	flattened := make(map[string]string)
	for k, v := range headers {
		flattened[k] = v[0]
	}
	return flattened
}
