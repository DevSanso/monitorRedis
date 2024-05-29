package logger

type ILogger interface {
	Info(message string) error
	Err(message string) error
	Trace(message string) error
}

const (
	LevelTrace = iota
	LevelInfo
	LevelError
	LevelNone
)

type LogLevel int
