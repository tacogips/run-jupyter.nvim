local jupyter_client = require("librun_jupyter")
local config = require("run-jupyter.config")
local window = require("run-jupyter.window")

local pickers = require("telescope.pickers")
local finders = require("telescope.finders")
local conf = require("telescope.config").values
local actions = require("telescope.actions")
local action_state = require("telescope.actions.state")

local M = {}
local running_kernel_surffix = " <running>"

local function get_running_kernels()
	return jupyter_client.list_running_kernels(config.get().jupyter.endpoint)
end

local function get_all_kernel_names()
	return jupyter_client.list_kernel_names(config.get().jupyter.endpoint)
end

local function start_kernel(kernel_name)
	return jupyter_client.start_kernel(config.get().jupyter.endpoint, kernel_name)
end

local function interrupt_kernel(kernel_name)
	return jupyter_client.interrupt_kernel(config.get().jupyter.endpoint, kernel_name)
end

local function get_running_kernels_or_error()
	local running_kernel_table = {}
	local running_kernel_data = get_running_kernels()
	for k, v in pairs(running_kernel_data) do
		if k == "error" then
			window.output_result("Error:\n" .. v)
			return
		elseif k == "data" then
			for k, v in pairs(v) do
				running_kernel_table[v] = v
			end
		end
	end

	return running_kernel_table
end

local function running_kernel_candidates()
	local result = {}

	local running_kernel_table = get_running_kernels_or_error()
	if running_kernel_table["error"] ~= nil then
		return running_kernel_table
	end

	local all_kernels = get_all_kernel_names()
	for k, v in pairs(all_kernels) do
		if k == "error" then
			window.output_result("Error:\n" .. v)
			return
		elseif k == "data" then
			for k, v in pairs(v) do
				if running_kernel_table[v] ~= nil then
					table.insert(result, v .. running_kernel_surffix)
				else
					table.insert(result, v)
				end
			end
		end
	end

	return result
end

function M.open_start_kernel_selection()
	local selector = function(opts)
		opts = opts or {}
		pickers.new(opts, {
			prompt_title = "start kernel",
			finder = finders.new_table({
				results = running_kernel_candidates(),
			}),
			sorter = conf.generic_sorter(opts),
			attach_mappings = function(prompt_bufnr, map)
				actions.select_default:replace(function()
					actions.close(prompt_bufnr)
					local selection = action_state.get_selected_entry()
					local selected_kernel = selection[1]
					if not string.find(selected_kernel, running_kernel_surffix) then
						start_kernel(selected_kernel)
					end
				end)
				return true
			end,
		}):find()
	end

	selector()
end

function M.open_interrupt_kernel_selection()
	local running_kernel_table = get_running_kernels_or_error()
	if running_kernel_table["error"] ~= nil then
		return running_kernel_table
	end
	local running_kernel_array = {}
	for k, v in pairs(running_kernel_table) do
		table.insert(running_kernel_array, v)
	end

	local selector = function(opts)
		opts = opts or {}
		pickers.new(opts, {
			prompt_title = "interrupt kernel",
			finder = finders.new_table({
				results = running_kernel_array,
			}),
			sorter = conf.generic_sorter(opts),
			attach_mappings = function(prompt_bufnr, map)
				actions.select_default:replace(function()
					actions.close(prompt_bufnr)
					local selection = action_state.get_selected_entry()
					local selected_kernel = selection[1]
					interrupt_kernel(selected_kernel)
				end)
				return true
			end,
		}):find()
	end

	selector()
end

return M
--window.output_result("aaa\nbbb")
--window.close_result_window()
