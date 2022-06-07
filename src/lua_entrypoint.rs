use super::parser::*;
use jupyter_client::*;
use mlua::prelude::*;
use mlua::Error as LuaError;
use std::sync::Arc;
use thiserror::Error;
use tokio::runtime::Runtime; // 0.3.5

#[derive(Error, Debug)]
pub enum JupyterRunnerError {
    #[error("kernel not found {0}")]
    KernelNotFound(String),

    #[error("api error :{0}")]
    JupyterApiError(#[from] JupyterApiError),

    #[error("parse error :{0}")]
    ParserError(#[from] ParserError),
}

const RESEPONSE_TABLE_KEY_ERROR: &str = "error";
const RESEPONSE_TABLE_KEY_DATA: &str = "data";
const RESEPONSE_TABLE_KEY_PNG: &str = "png";
const RESEPONSE_TABLE_KEY_TEXT: &str = "text";

fn to_error_table(lua: &Lua, e: JupyterRunnerError) -> LuaResult<LuaTable<'_>> {
    let response_table = lua.create_table()?;
    response_table.set(RESEPONSE_TABLE_KEY_ERROR, e.to_string())?;
    Ok(response_table)
}

fn empty_table(lua: &Lua) -> LuaResult<LuaTable<'_>> {
    let response_table = lua.create_table()?;
    Ok(response_table)
}

fn start_kernel(
    lua: &Lua,
    (jupyter_base_url, kernel_name): (String, String),
) -> LuaResult<LuaTable<'_>> {
    match get_jupyter_client(&jupyter_base_url) {
        Err(e) => Ok(to_error_table(&lua, e)?),
        Ok(jupyter_client) => {
            let kernel_req = KernelPostRequest {
                name: kernel_name,
                path: None,
            };

            match Runtime::new()
                .unwrap()
                .block_on(jupyter_client.start_kernel(kernel_req))
            {
                Ok(()) => Ok(empty_table(&lua)?),
                Err(e) => Ok(to_error_table(&lua, e.into())?),
            }
        }
    }
}

fn get_jupyter_client(jupyter_base_url: &str) -> Result<JupyterClient, JupyterRunnerError> {
    let client = JupyterClient::new(jupyter_base_url, None, None)?;
    Ok(client)
}

async fn get_kernel_client(
    jupyter_base_url: &str,
    kernel_name: &str,
) -> Result<Option<KernelApiClient>, JupyterRunnerError> {
    let jupyter_client = get_jupyter_client(jupyter_base_url)?;

    let kernels = jupyter_client.get_kernels().await?;

    let kernel = kernels.iter().find(|each| each.name == kernel_name);
    match kernel {
        None => Err(JupyterRunnerError::KernelNotFound(kernel_name.to_string())),
        Some(kernel) => Ok(Some(jupyter_client.new_kernel_client(kernel)?)),
    }
}

fn list_kernels(lua: &Lua, jupyter_base_url: String) -> LuaResult<LuaTable<'_>> {
    let jupyter_client = match get_jupyter_client(&jupyter_base_url) {
        Err(e) => return Ok(to_error_table(&lua, e.into())?),
        Ok(jupyter_client) => jupyter_client,
    };

    match Runtime::new()
        .unwrap()
        .block_on(jupyter_client.get_kernels())
    {
        Err(e) => Ok(to_error_table(&lua, e.into())?),
        Ok(kernels) => {
            let kernel_names = kernels
                .into_iter()
                .map(|kernel| kernel.name)
                .collect::<Vec<String>>();

            let response_table = lua.create_table()?;
            response_table.set(RESEPONSE_TABLE_KEY_DATA, kernel_names)?;
            Ok(response_table)
        }
    }
}

fn run_code(
    lua: &Lua,
    (jupyter_base_url, kernel_name, code): (String, String, String),
) -> LuaResult<LuaTable<'_>> {
    let kernel_client = match Runtime::new()
        .unwrap()
        .block_on(get_kernel_client(&jupyter_base_url, &kernel_name))
    {
        Err(e) => return Ok(to_error_table(&lua, e.into())?),
        Ok(kernel_client) => match kernel_client {
            None => {
                return Ok(to_error_table(
                    &lua,
                    JupyterRunnerError::KernelNotFound(code.to_string()),
                )?)
            }
            Some(kernel_client) => kernel_client,
        },
    };

    let code = if let Ok(parsable_kernel) = ParsableKernel::try_from_str(&kernel_name) {
        let parsed_code = match parsable_kernel {
            ParsableKernel::Rust => match RustParser.parse(&code) {
                Ok(parsed_code) => parsed_code,
                Err(e) => return Ok(to_error_table(&lua, e.into())?),
            },
            ParsableKernel::Python3 => {
                return Ok(to_error_table(
                    &lua,
                    ParserError::UnsuppotedKernel("python".to_string()).into(),
                )?)
            }
        };
        match parsed_code {
            Some(cell_sources) => cell_sources.as_one_line_code(),
            None => return Ok(empty_table(&lua)?),
        }
    } else {
        code
    };

    let response = match Runtime::new()
        .unwrap()
        .block_on(kernel_client.run_code(code.into(), None))
    {
        Ok(response) => response,
        Err(e) => return Ok(to_error_table(&lua, e.into())?),
    };

    let contents = match response.as_content() {
        Ok(contents) => contents,
        Err(e) => return Ok(to_error_table(&lua, e.into())?),
    };

    let result = match contents {
        None => empty_table(&lua)?,
        Some(contents) => {
            let content_data = match contents {
                KernelContent::DisplayData(display_data) => Some(display_data.data),
                KernelContent::ExecuteResultContent(result_content) => Some(result_content.data),
                _ => None,
            };

            match content_data {
                Some(data) => {
                    let response_table = lua.create_table()?;
                    if let Some(image) = data.image_png {
                        response_table.set(RESEPONSE_TABLE_KEY_PNG, image)?;
                    } else if let Some(text_plain) = data.text_plain {
                        response_table.set(RESEPONSE_TABLE_KEY_TEXT, text_plain)?;
                    }
                    response_table
                }
                None => empty_table(&lua)?,
            }
        }
    };

    LuaResult::Ok(result)
}

#[mlua::lua_module]
fn librun_jupyter(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;

    exports.set("start_kernel", lua.create_function(start_kernel)?)?;
    exports.set("list_kernels", lua.create_function(list_kernels)?)?;
    exports.set("run_code", lua.create_function(run_code)?)?;
    Ok(exports)
}
