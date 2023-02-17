package main

import (
	"encoding/json"
	"net/http"
)

type Person struct {
	Name string `json:"name"`
	Age  int    `json:"age"`
}

func main() {
	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		person := Person{Name: "Alice", Age: 30}
		json.NewEncoder(w).Encode(person)
	})

	http.ListenAndServe(":1200", nil)
}
