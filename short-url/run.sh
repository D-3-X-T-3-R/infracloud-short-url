LOCAL_HOST=$"127.0.0.1"
PORT=$"8000"
OUTPUT_FILE=$"test-bed/url_map.txt"
LOG_FILE=$"test-bed/log.txt"

cargo run -- \
--local-host ${LOCAL_HOST} \
--port ${PORT} \
--url-map-file-path ${OUTPUT_FILE} 