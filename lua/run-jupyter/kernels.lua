local jupyter_client = require("librun_jupyter")
local config = require("run-jupyter.config")
local window = require("run-jupyter.window")

local M = {}

local function get_running_kernels()
	return jupyter_client.list_running_kernels(config.jupyter.endpoint)
end

local function get_all_kernel_names()
	return jupyter_client.list_kernel_names(config.jupyter.endpoint)
end

function M.open_start_kernel_selection()
	local running_kernel_table = {}
	local result = {}
	local running_kernel_data = get_running_kernels()
	for k, v in pairs(running_kernel_data) do
		if k == "error" then
			window.output_result("Error:\n" .. v)
			return
		end
		running_kernel_table[v] = true
	end

	local all_kernels = get_all_kernel_names()
	for k, v in pairs(all_kernels) do
		if k == "error" then
			window.output_result("Error:\n" .. v)
			return
		end
		if running_kernel_table[v] ~= nil then
			result.insert(k, v .. " (running)")
		else
			result.insert(k, v)
		end
	end

	return result
end

return M

--window.output_result("aaa\nbbb")
--window.close_result_window()
