local config = require("run-jupyter.config")
local window = require("run-jupyter.window")
local kernel = require("run-jupyter.kernel")

local M = {}
function M.setup(user_config)
	config.build(user_config)
end

M.close_result_window = window.close_result_window
M.open_start_kernel_selection = kernel.open_start_kernel_selection
M.open_kill_kernel_selection = kernel.open_kill_kernel_selection
M.open_switch_kernel_selection = kernel.open_switch_kernel_selection
M.run_selecting_code = kernel.run_selecting_code

return M
