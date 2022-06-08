local jupyter_client = require("librun_jupyter")

--for k, v in ipairs(jupyter_client.list_kernels("http://localhost:8888")["data"]) do
--	print(k, v)
--
--	for k, v in pairs(jupyter_client.start_kernel("http://localhost:8888", v)) do
--		print("run kernel", k, v)
--	end
--end
--print(jupyter_client.test_run())
--print(jupyter_client.test_run()["error"])
--print(jupyter_client.list_kernels("http://localhost:8888"))
--print("--hello")
