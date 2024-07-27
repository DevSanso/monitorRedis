package redis

const (
	KeyUsageTopTenHundredQuery = `
	SELECT key_name, collect_time, usage_byte, expired_sec FROM redis_key WHERE link_key = ? AND collect_time BETWEEN ? AND ?;
	`
)