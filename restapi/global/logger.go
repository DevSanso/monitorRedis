package global

import (
	"restapi/logger"
)

var (
	log logger.ILogger
)

func GetLogger() logger.ILogger {
	return log
}