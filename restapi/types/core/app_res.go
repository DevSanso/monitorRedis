package core

type ApplicationResponse struct {
	Response []byte
	Err error
	Code int
}