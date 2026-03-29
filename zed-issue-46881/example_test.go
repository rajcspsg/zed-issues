package example_test

import (
	"fmt"
	"testing"
)

func TestSomething(t *testing.T) {
	tests := []struct {
		name string
	}{
		{name: "test_1"},
		{name: "test_2"},
		{name: "test_3"},
		{name: "test_4"},
		{name: "test_5"},
		{name: "test_6"},
		{name: "test_7"},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			fmt.Println(tt.name)
		})
	}
}
