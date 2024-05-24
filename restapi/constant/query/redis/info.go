package redis

const (
	InfoCpuQuery = `
	SELECT  
		collect_time, used_cpu_sys, used_cpu_user, used_cpu_sys_children, used_cpu_user_children
	FROM redis_info_cpu WHERE unqiue_id  = ? AND column_name BETWEEN TO_TIMESTAMP('yyyymmddhhmi24ss',?) AND TO_TIMESTAMP('yyyymmddhhmi24ss',?)`
)