package main

import (
	"net/http"

	"restapi/handler"
)
func main() {
	http.ListenAndServe("0.0.0.0", handler.NewHandler())
}