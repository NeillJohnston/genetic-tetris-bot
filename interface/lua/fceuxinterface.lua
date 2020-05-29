local socket = require("socket.core")

client = socket.tcp()
client:connect("127.0.0.1", 10000)
client:send("string")