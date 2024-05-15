package errs

import (
	"fmt"
	"strings"
)

type OOMError struct {
	Ip string
	Url string
	LimitSize int
	AllocSize int
}

func (e *OOMError)Error() string {
	return fmt.Sprintf("[OOMError] : (id:%s,url:%s,limit:%f kb,alloc:%f kb)",e.Ip, e.Url, float64(e.LimitSize) / 1024.0, float64(e.AllocSize) /1024.0)
}

type InternalError struct {
	Ip string
	Url string
	Err error
}

func (e *InternalError)Error() string {
	return fmt.Sprintf("[InternalError] : (id:%s,url:%s,internal:%s)",e.Ip, e.Url, e.Err.Error())
}

type ServerDbConnFailedError struct {
	Source error
	Server string
}

func (e *ServerDbConnFailedError)Error() string {
	return fmt.Sprintf("[ServerDbConnFailedError] : (Server:%s,Source:%s)",e.Server, e.Source.Error())
}

type ServerDbConnExcuteError struct {
	Server string
	Source error
	ObjectNames []string
}

func (e *ServerDbConnExcuteError)Error() string {
	return fmt.Sprintf("[ServerDbConnExcuteError] : (Server:%s,Source:%s,Objects:%s)",e.Server, e.Source.Error(), strings.Join(e.ObjectNames,"|"))
}

type NoDataError struct {
	SelectDataName string
	UnqiueKey string
}

func (e *NoDataError)Error() string {
	return fmt.Sprintf("[NoDataError] : (data:%s,key:%s)", e.SelectDataName, e.UnqiueKey)
}