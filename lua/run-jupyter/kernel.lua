local jupyter_client = require("librun_jupyter")
local config = require("run-jupyter.config")
local window = require("run-jupyter.window")

local pickers = require("telescope.pickers")
local finders = require("telescope.finders")
local conf = require("telescope.config").values
local actions = require("telescope.actions")
local action_state = require("telescope.actions.state")
local status = { current = nil }

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

local function delete_kernel(kernel_id)
	return jupyter_client.delete_kernel(config.get().jupyter.endpoint, kernel_id)
end

local function get_running_kernels_or_error()
	local running_kernel_table = {}
	local running_kernel_data = get_running_kernels()
	for k, v in pairs(running_kernel_data) do
		if k == "error" then
			window.output_result("Error:\n" .. v)
			return nil
		elseif k == "data" then
			for kernel_id, kernel_name in pairs(v) do
				running_kernel_table[kernel_id] = kernel_name
			end
		end
	end

	return running_kernel_table
end

local function run_kernel_candidates()
	local result = {}

	local running_kernel_name_table = {}
	local running_kernel_table = get_running_kernels_or_error()
	if not running_kernel_table then
		return nil
	end
	if running_kernel_table["error"] ~= nil then
		return running_kernel_table
	end

	for _, name in pairs(running_kernel_table) do
		table.insert(running_kernel_name_table, name)
	end

	local all_kernels = get_all_kernel_names()
	for k, v in pairs(all_kernels) do
		if k == "error" then
			window.output_result("Error:\n" .. v)
			return
		elseif k == "data" then
			for _, kernel_name in pairs(v) do
				if running_kernel_name_table[kernel_name] ~= nil then
					table.insert(result, kernel_name .. running_kernel_surffix)
				else
					table.insert(result, kernel_name)
				end
			end
		end
	end

	return result
end

function M.open_start_kernel_selection()
	local kernel_names = run_kernel_candidates()
	if not kernel_names then
		return
	end

	local selector = function(opts)
		opts = opts or {}
		pickers.new(opts, {
			prompt_title = "start kernel",
			finder = finders.new_table({
				results = kernel_names,
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

local function get_running_kernel_array()
	local running_kernel_table = get_running_kernels_or_error()
	if not running_kernel_table then
		return nil
	end
	if running_kernel_table["error"] ~= nil then
		return running_kernel_table
	end
	local running_kernel_array = {}
	for id, name in pairs(running_kernel_table) do
		table.insert(running_kernel_array, id .. "<" .. name .. ">")
	end
	return running_kernel_array
end

function M.open_kill_kernel_selection()
	local running_kernel_array = get_running_kernel_array()

	if not running_kernel_array then
		return nil
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
					local selected_kernel_id = string.gsub(selected_kernel, "<.*", "")
					delete_kernel(selected_kernel_id)
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
