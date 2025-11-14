-- ws_sync.lua
function request()
    -- 升级为 WebSocket 连接，发送 Markdown 增量内容
    local path = "/api/sync/ws"
    local headers = {["Upgrade"] = "websocket", ["Connection"] = "Upgrade"}
    return wrk.format("GET", path, headers)
end

-- WebSocket 消息处理：模拟每 100ms 发送一次增量更新
function websocket(ws)
    while true do
        local delta = cjson.encode({
            doc_id = math.random(1, 100),
            delta_content = "**Updated content**" .. math.random(1000),
            timestamp = os.time()
        })
        ws:send(delta)
        wrk.sleep(0.1) -- 100ms 间隔
    end
end