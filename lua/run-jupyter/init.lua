local config = require("run-jupyter.config")
local window = require("run-jupyter.window")
local kernel = require("run-jupyter.kernel")

local M = {}
function M.setup(user_config)
	config.build(user_config)
end

M.close_result_window = window.close_result_window

M.open_start_kernel_selection = kernel.open_start_kernel_selection

--        _, lineno_begin, colno_begin, _ = self.nvim.funcs.getpos("'[")
--        _, lineno_end,   colno_end,   _ = self.nvim.funcs.getpos("']")
--
--local jupyter_client = require("librun_jupyter")
--
--print("aaa")
--local result = jupyter_client.list_running_kernels("http://localhost:8888")
--
--print("kernel spacs --- ")
--for k, v in pairs(result) do
--	for k, v in pairs(v) do
--		print(k, "---", v)
--	end
--end
--
--local result = jupyter_client.list_kernel_names("http://localhost:8888")
--print("kernel spacs --- ")
--for k, v in pairs(result) do
--	for k, v in pairs(v) do
--		print(k, "---", v)
--	end
--end
--
----local result = jupyter_client.start_kernel("http://localhost:8888", "rust")
--
--local result = jupyter_client.run_code("http://localhost:8888", "rust", "2 * 12")
--print("cmd --- ")
--print("---", result["text"])
--for k, v in pairs(result) do
--	print(k, v)
--end
--
-- for k, v in ipairs(jupyter_client.list_kernels("http://localhost:8888")["data"]) do
-- 	print(k, v)
--
-- 	for k, v in pairs(jupyter_client.start_kernel("http://localhost:8888", v)) do
-- 		print("run kernel", k, v)
-- 	end
-- --end
-- --print(jupyter_client.test_run())
-- --print(jupyter_client.test_run()["error"])
-- --print(jupyter_client.list_kernels("http://localhost:8888"))
-- --print("--hello")
