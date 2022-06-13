local jupyter_client = require("librun_jupyter")
local config = require("run-jupyter.config")
local window = require("run-jupyter.window")

local pickers = require("telescope.pickers")
local finders = require("telescope.finders")
local conf = require("telescope.config").values
local actions = require("telescope.actions")
local action_state = require("telescope.actions.state")
local status = { current_kernel_id = nil }

local M = {}
local running_kernel_surffix = " <running>"

local fn = vim.fn
local api = vim.api

local function get_running_kernels()
	return jupyter_client.list_running_kernels(config.get().jupyter.endpoint)
end

local function get_all_kernel_names()
	return jupyter_client.list_kernel_names(config.get().jupyter.endpoint)
end

local function start_kernel(kernel_name)
	return jupyter_client.start_kernel(config.get().jupyter.endpoint, kernel_name)
end

local function send_code_to_kernel(kernel_id, code)
	return jupyter_client.run_code(config.get().jupyter.endpoint, kernel_id, code)
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
		running_kernel_name_table[name] = name
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
						local kernel_result = start_kernel(selected_kernel)

						for k, v in kernel_result do
							if k == "error" then
								window.output_result("Error:\n" .. v)
								return nil
							elseif k == "data" then
								local kernel_id = v
								status.current_kernel_id = kernel_id
							end
						end
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
		local selecting_sign = ""
		if id == status.current_kernel_id then
			selecting_sign = "*"
		end
		table.insert(running_kernel_array, id .. "<" .. name .. ">" .. " " .. selecting_sign)
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
			prompt_title = "kernel to kill",
			finder = finders.new_table({
				results = running_kernel_array,
			}),
			sorter = conf.generic_sorter(opts),
			attach_mappings = function(prompt_bufnr, map)
				actions.select_default:replace(function()
					actions.close(prompt_bufnr)
					local selection = action_state.get_selected_entry()
					local selected_kernel = selection[1]
					local selected_kernel_id = string.gsub(selected_kernel, "<.*", "") -- TODO(tacogips) too ugly
					delete_kernel(selected_kernel_id)
					if status.current_kernel_id == selected_kernel_id then
						status.current_kernel_id = nil
					end
				end)
				return true
			end,
		}):find()
	end

	selector()
end

function M.open_switch_kernel_selection()
	local running_kernel_array = get_running_kernel_array()

	if not running_kernel_array then
		return nil
	end

	local selector = function(opts)
		opts = opts or {}
		pickers.new(opts, {
			prompt_title = "switch kernel",
			finder = finders.new_table({
				results = running_kernel_array,
			}),
			sorter = conf.generic_sorter(opts),
			attach_mappings = function(prompt_bufnr, map)
				actions.select_default:replace(function()
					actions.close(prompt_bufnr)
					local selection = action_state.get_selected_entry()
					local selected_kernel = selection[1]
					local selected_kernel_id = string.gsub(selected_kernel, "<.*", "") -- TODO(tacogips) too ugly
					status.current_kernel_id = selected_kernel_id
				end)
				return true
			end,
		}):find()
	end

	selector()
end

local function run_code(code)
	local result = {}
	if not status.current_kernel_id then
		result["error"] = "kernel not selected"
		return result
	end

	return send_code_to_kernel(status.current_kernel_id, code)
end

-- thanks to  https://github.com/ibhagwan/nvim-lua/blob/main/lua/utils.lua
local function get_visual_selection_lines()
	local _, csrow, cscol, cerow, cecol
	local mode = fn.mode()
	if mode == "v" or mode == "V" or mode == "" then
		-- if we are in visual mode use the live position
		_, csrow, cscol, _ = unpack(fn.getpos("."))
		_, cerow, cecol, _ = unpack(fn.getpos("v"))
		if mode == "V" then
			cscol, cecol = 0, 999
		end
		api.nvim_feedkeys(api.nvim_replace_termcodes("<Esc>", true, false, true), "n", true)
	else
		-- otherwise, use the last known visual position
		_, csrow, cscol, _ = unpack(vim.fn.getpos("'<"))
		_, cerow, cecol, _ = unpack(vim.fn.getpos("'>"))
	end
	-- swap vars if needed
	if cerow < csrow then
		csrow, cerow = cerow, csrow
	end
	if cecol < cscol then
		cscol, cecol = cecol, cscol
	end
	local lines = fn.getline(csrow, cerow)
	-- local n = cerow-csrow+1
	local n = #lines
	if n <= 0 then
		return ""
	end
	return table.concat(lines, "\n")
end

function M.run_selecting_code()
	local selection_code = get_visual_selection_lines()

	local result = run_code(selection_code)

	for k, v in pairs(result) do
		print("--b", k, v)
	end
	for k, v in pairs(result) do
		if k == "error" then
			window.output_result("Error:\n" .. v)
		elseif k == "text" then
			window.output_result(v)
		end
	end
end

return M
