package service_vo

import "restapi/types/repo_vo"

type ServerCpuUsageVO struct {
	CollectTime  string
	SysCpuUsage  float64
	UserCpuUsage float64

	Time struct {
		CpuSys       float64
		CpuUser      float64
		ChildCpuSys  float64
		ChildCpuUser float64
		TotalSecond  int64
	}
}

func NewServerCpuUsageVO(list []repo_vo.ServerCpuVO) []ServerCpuUsageVO {
	if len(list) < 2 {
		return nil
	}

	ret := make([]ServerCpuUsageVO, 0)

	var cpuLeft *repo_vo.ServerCpuVO = nil
	var cpuRight *repo_vo.ServerCpuVO = nil

	size := len(list)

	for i := 0; i < size; i++ {
		if cpuLeft == nil {
			cpuLeft = &list[i]
			continue
		}
		if cpuRight == nil {
			cpuRight = &list[i]
		}

		totalSec := cpuRight.UptimeSecond - cpuLeft.UptimeSecond

		cpuSysTime := cpuRight.CpuSys - cpuLeft.CpuSys
		cpuSysUsage := (cpuSysTime / float64(totalSec)) * 100
		if cpuSysUsage > 100.0 {
			cpuSysUsage = 100
		}

		cpuUserTime := cpuRight.CpuUser - cpuLeft.CpuUser
		cpuUserUsage := (cpuUserTime / float64(totalSec)) * 100
		if cpuUserUsage > 100.0 {
			cpuUserUsage = 100
		}

		cpuChildSysTime := cpuRight.ChildCpuSys - cpuLeft.ChildCpuSys
		cpuChildSysUsage := (cpuChildSysTime / float64(totalSec)) * 100
		if cpuChildSysUsage > 100.0 {
			cpuChildSysUsage = 100
		}

		cpuChildUserTime := cpuRight.ChildCpuUser - cpuLeft.ChildCpuUser
		cpuChildUserUsage := (cpuChildUserTime / float64(totalSec)) * 100
		if cpuChildUserUsage > 100.0 {
			cpuChildUserUsage = 100
		}

		data := ServerCpuUsageVO{
			CollectTime:  cpuLeft.CollectTime,
			SysCpuUsage:  cpuSysUsage + cpuChildSysUsage,
			UserCpuUsage: cpuUserUsage + cpuChildUserUsage,
		}

		data.Time.CpuSys = cpuSysTime
		data.Time.CpuUser = cpuUserTime
		data.Time.ChildCpuSys = cpuChildSysTime
		data.Time.ChildCpuUser = cpuChildUserTime
		data.Time.TotalSecond = totalSec

		ret = append(ret, data)
		cpuLeft = nil
		cpuRight = nil
	}

	return ret
}
