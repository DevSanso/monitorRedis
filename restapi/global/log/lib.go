package log

import (
	"sync"
	"io"
	"os"
	"restapi/logger"
)

var (
	onceLogInit sync.Once
	log logger.ILogger
)

type internalStdout struct {
	stdout *os.File
}

func (i *internalStdout)Write(b []byte)(int, error) {
	return i.stdout.Write(b)
}

func (i *internalStdout)Close() error {
	return nil
}

func InitLog(level logger.LogLevel, filePath *string) {
	onceLogInit.Do(func() {
		var w = make([]io.WriteCloser, 0)
		
		w = append(w, &internalStdout{stdout: os.Stdout})

		if filePath != nil {
			f,err := os.OpenFile(*filePath, os.O_CREATE | os.O_APPEND | os.O_WRONLY, os.FileMode(0644))
			if err == nil {
				w = append(w, f)
			}else {
				panic("InitLog Error :" + err.Error())
			}
		}

		log = logger.NewStdLogger(level, w...)
	})
}

func Info(message string) {
	log.Info(message)
}

func Trace(message string) {
	log.Trace(message)
}

func Err(message string) {
	log.Err(message)
}
