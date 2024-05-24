package repo_vo

type InfoCpuVO struct {
	CollectTime string
	CpuSys float64
	CpuUser float64
	ChildCpuSys float64
	ChildCpuUser float64
}