use super::parser::*;
use jupyter_client::*;
use mlua::prelude::*;
use mlua::Error as LuaError;
use std::sync::Arc;

fn api_error_to_lua_error(e: JupyterApiError) -> LuaError {
    LuaError::ExternalError(Arc::new(e))
}

fn parser_error_to_lua_error(e: ParserError) -> LuaError {
    LuaError::ExternalError(Arc::new(e))
}

async fn start_kernel(
    _lua: &Lua,
    (jupyter_base_url, kernel_name): (String, String),
) -> LuaResult<()> {
    let jupyter_client = get_jupyter_client(&jupyter_base_url)?;

    let kernel_req = KernelPostRequest {
        name: kernel_name,
        path: None,
    };

    jupyter_client
        .start_kernel(kernel_req)
        .await
        .map_err(api_error_to_lua_error)?;
    Ok(())
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum JupyterRunnerError {
    #[error("kernel not found {0}")]
    KernelNotFound(String),
}

fn get_jupyter_client(jupyter_base_url: &str) -> Result<JupyterClient, LuaError> {
    JupyterClient::new(jupyter_base_url, None, None).map_err(api_error_to_lua_error)
}

async fn get_kernel_client(
    jupyter_base_url: &str,
    kernel_name: &str,
) -> Result<KernelApiClient, LuaError> {
    let jupyter_client = get_jupyter_client(jupyter_base_url)?;

    let kernels = jupyter_client
        .get_kernels()
        .await
        .map_err(api_error_to_lua_error)?;

    let kernel = kernels.iter().find(|each| each.name == "rust");
    match kernel {
        None => Err(LuaError::ExternalError(Arc::new(
            JupyterRunnerError::KernelNotFound(kernel_name.to_string()),
        ))),
        Some(kernel) => Ok(jupyter_client
            .new_kernel_client(kernel)
            .map_err(api_error_to_lua_error)?),
    }
}

async fn list_kernel_names(_lua: &Lua, jupyter_base_url: String) -> LuaResult<Vec<String>> {
    let jupyter_client = get_jupyter_client(&jupyter_base_url)?;
    Ok(jupyter_client
        .get_kernels()
        .await
        .map_err(api_error_to_lua_error)?
        .into_iter()
        .map(|kernel| kernel.name)
        .collect())
}

const RESEPONSE_TABLE_KEY_PNG: &str = "png";
const RESEPONSE_TABLE_KEY_TEXT: &str = "text";

async fn run_code(
    lua: &Lua,
    (jupyter_base_url, kernel_name, code): (String, String, String),
) -> LuaResult<Option<LuaTable<'_>>> {
    let kernel_client = get_kernel_client(&jupyter_base_url, &kernel_name).await?;

    let code = if let Ok(parsable_kernel) = ParsableKernel::try_from_str(&kernel_name) {
        let parsed_code = match parsable_kernel {
            ParsableKernel::Rust => RustParser.parse(&code).map_err(parser_error_to_lua_error)?,
            ParsableKernel::Python3 => {
                return Err(parser_error_to_lua_error(ParserError::UnsuppotedKernel(
                    "python".to_string(),
                )))
            }
        };
        match parsed_code {
            Some(cell_sources) => cell_sources.as_one_line_code(),
            None => return Ok(None),
        }
    } else {
        code
    };

    let response = kernel_client
        .run_code(code.into(), None)
        .await
        .map_err(api_error_to_lua_error)?;
    let contents = response.as_content().map_err(api_error_to_lua_error)?;
    let result = match contents {
        None => None,
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

                    Some(response_table)
                }
                None => None,
            }
        }
    };

    LuaResult::Ok(result)
}

#[mlua::lua_module]
fn run_jupyter(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("start_kernel", lua.create_async_function(start_kernel)?)?;
    exports.set(
        "list_kernels",
        lua.create_async_function(list_kernel_names)?,
    )?;
    exports.set("run_code", lua.create_async_function(run_code)?)?;
    Ok(exports)
}
