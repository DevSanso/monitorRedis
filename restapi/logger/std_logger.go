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
	isClose bool
	writerCloses []io.WriteCloser
	m sync.Mutex
}

func (l *stdLogger) Err(message string) error {
	if l.setLevel > LevelError {return nil}
	l.m.Lock()
	defer l.m.Unlock()

	if l.isClose {return io.ErrClosedPipe}
	
	l.errL.Println(message)

	return nil
}

func (l *stdLogger) Info(message string) error {
	if l.setLevel > LevelInfo {return nil}
	l.m.Lock()
	defer l.m.Unlock()

	if l.isClose {return io.ErrClosedPipe}

	l.infoL.Println(message)

	return nil
}

func (l *stdLogger) Trace(message string) error {
	if l.setLevel > LevelTrace {return nil}
	l.m.Lock()
	defer l.m.Unlock()

	if l.isClose {return io.ErrClosedPipe}

	l.traceL.Println(message)

	return nil
}

func (l *stdLogger) Close() error {
	l.m.Lock()
	defer l.m.Unlock()
	if l.isClose {return nil}

	l.isClose = true

	for _,c := range l.writerCloses {
		c.Close()
	}

	return nil
}

func NewStdLogger(level LogLevel, writers ...io.WriteCloser) ILogger {
	wonly := make([]io.Writer,0)
	for _,w := range writers {
		wonly = append(wonly, w)
	}

	mutl := io.MultiWriter(wonly...)

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
		isClose: false,
		writerCloses : writers,
		m : sync.Mutex{},
	}
}
