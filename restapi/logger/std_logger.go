package logger

import (
	"io"
	"log"
	"sync"
)

type stdLogger struct {
	infoL  *log.Logger
	errL   *log.Logger
	traceL *log.Logger

	setLevel LogLevel
	m sync.Mutex
}

func (l *stdLogger) Err(message string) error {
	if l.setLevel > LevelError {return nil}
	l.m.Lock()
	defer l.m.Unlock()
	
	l.errL.Println(message)

	return nil
}

func (l *stdLogger) Info(message string) error {
	if l.setLevel > LevelInfo {return nil}
	l.m.Lock()
	defer l.m.Unlock()

	l.infoL.Println(message)

	return nil
}

func (l *stdLogger) Trace(message string) error {
	if l.setLevel > LevelTrace {return nil}
	l.m.Lock()
	defer l.m.Unlock()

	l.traceL.Println(message)

	return nil
}

func NewStdLogger(level LogLevel, writers ...io.Writer) ILogger {
	mutl := io.MultiWriter(writers...)

	var errL *log.Logger = nil
	var infoL *log.Logger = nil
	var traceL *log.Logger = nil

	const errPrifix = "ERROR: "
	const infPrifix = "INFO:  "
	const traPrifix = "TRACE: "

	if level <= LevelError {
		errL = log.New(mutl, errPrifix, log.Ldate)
	}
	if level <= LevelInfo {
		infoL = log.New(mutl, infPrifix, log.Ldate)
	}
	if level <= LevelTrace {
		traceL = log.New(mutl, traPrifix, log.Ldate)
	}

	return &stdLogger{
		errL:     errL,
		infoL:    infoL,
		traceL:   traceL,
		setLevel: level,
		m : sync.Mutex{},
	}
}
