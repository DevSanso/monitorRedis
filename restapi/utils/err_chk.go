package utils

import (
	"reflect"
)

func CheckTypeIs[T any](t any) bool {
	var temp T
	eqT := reflect.TypeOf(temp)
	eqT2 := reflect.TypeOf(t)

	return eqT.String() == eqT2.String()
}

func CheckErrorIs[T error](err error) bool {
	return CheckTypeIs[T](err)
}
