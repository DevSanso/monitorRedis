package errs

import (
	"fmt"
)

type BadRequestError struct {
	Ip string
	Url string
	Msg string
}

func (e *BadRequestError)Error() string {
	return fmt.Sprintf("[BadRequestError] : (id:%s,url:%s,msg:%s)",e.Ip, e.Url, e.Msg)
}