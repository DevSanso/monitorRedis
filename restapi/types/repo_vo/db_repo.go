package repo_vo

type DbKeyMemUsage struct{
	CollectTime string
	KeyName string
	UsageByte int
	ExpiredSec int
}

type DbKeySpaceInfo struct{
	CollectTime string
	DbName string
	Keys int
	Expired int
	AvgTTL int
}