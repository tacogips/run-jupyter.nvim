local api = vim.api
local cmd = vim.cmd
local bo = vim.bo
local fn = vim.fn
local M = {}

local function win_info()
	local info = {}
	info.width = math.ceil(api.nvim_get_option("columns"))
	info.height = math.ceil(api.nvim_get_option("lines"))
	return info
end

local function result_window_opts(height, row_pos)
	local windows_info = win_info()
	local width = windows_info.width
	if row_pos then
		row_pos, _ = unpack(api.nvim_win_get_cursor(0))
	end

	local opts = {
		style = "minimal",
		relative = "win",
		border = "none",
		width = width,
		height = height,
		row = row_pos,
		col = 0,
		noautocmd = true,
		focusable = false,
	}
	return opts
end

local function find_result_window()
	for _, win_id in ipairs(api.nvim_list_wins()) do
		local bufnr = api.nvim_win_get_buf(win_id)

		local ft = fn.getbufvar(bufnr, "&filetype")
		if ft == "run_jupyter_result" then
			return win_id
		end
	end
	return nil
end

local function close_window_if_exists()
	local found_win_id = find_result_window()
	if found_win_id ~= nil then
		local bufnr = api.nvim_win_get_buf(found_win_id)

		api.nvim_buf_delete(bufnr, { force = true })
	end
end

local function create_result_buffer(height, row_pos)
	close_window_if_exists()

	local bufnr = api.nvim_create_buf(false, true)
	api.nvim_buf_set_option(bufnr, "filetype", "run_jupyter_result")

	local win_opts = result_window_opts(height, row_pos)
	local win = api.nvim_open_win(bufnr, false, win_opts)
	api.nvim_win_set_option(win, "winblend", 10)
	return bufnr
end

local function contents_to_table(contents)
	local output = {}
	for each in contents:gmatch("[^\r\n]+") do
		table.insert(output, each)
	end
	return output
end

local function output_contents(bufnr, contents_table)
	api.nvim_buf_set_text(bufnr, 0, 0, 0, 0, contents_table)
end

local function table_length(table)
	local count = 0
	for _ in pairs(table) do
		count = count + 1
	end
	return count
end

function M.close_result_window()
	close_window_if_exists()
end

function M.output_result(contents)
	local contents_table = contents_to_table(contents)
	local height = table_length(contents_table)
	local bufnr = create_result_buffer(height, nil)
	output_contents(bufnr, contents_table)
end

function M.output_result_with_position(contents, row_pos)
	local contents_table = contents_to_table(contents)
	local height = table_length(contents_table)
	local bufnr = create_result_buffer(height, row_pos)
	output_contents(bufnr, contents_table)
end

return M
