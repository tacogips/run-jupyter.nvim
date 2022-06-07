use jupyter_client::*;
use mlua::prelude::*;
use mlua::Error as LuaError;
use std::sync::Arc;

fn to_lua_error(e: JupyterApiError) -> LuaError {
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
        .map_err(to_lua_error)?;
    Ok(())
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum JupyterRunnerError {
    #[error("kernel not found {0}")]
    KernelNotFound(String),
}

fn get_jupyter_client(jupyter_base_url: &str) -> Result<JupyterClient, LuaError> {
    JupyterClient::new(jupyter_base_url, None, None).map_err(to_lua_error)
}

async fn get_kernel_client(
    jupyter_base_url: &str,
    kernel_name: &str,
) -> Result<KernelApiClient, LuaError> {
    let jupyter_client = get_jupyter_client(jupyter_base_url)?;

    let kernels = jupyter_client.get_kernels().await.map_err(to_lua_error)?;

    let kernel = kernels.iter().find(|each| each.name == "rust");
    match kernel {
        None => Err(LuaError::ExternalError(Arc::new(
            JupyterRunnerError::KernelNotFound(kernel_name.to_string()),
        ))),
        Some(kernel) => Ok(jupyter_client
            .new_kernel_client(kernel)
            .map_err(to_lua_error)?),
    }
}

async fn list_kernel_names(_lua: &Lua, jupyter_base_url: String) -> LuaResult<Vec<String>> {
    let jupyter_client = get_jupyter_client(&jupyter_base_url)?;
    Ok(jupyter_client
        .get_kernels()
        .await
        .map_err(to_lua_error)?
        .into_iter()
        .map(|kernel| kernel.name)
        .collect())
}

async fn run_code(
    lua: &Lua,
    (jupyter_base_url, kernel_name, code): (String, String, String),
) -> LuaResult<Option<LuaTable<'_>>> {
    let kernel_client = get_kernel_client(&jupyter_base_url, &kernel_name).await?;
    let response = kernel_client
        .run_code(code.into(), None)
        .await
        .map_err(to_lua_error)?;
    let contents = response.as_content().map_err(to_lua_error)?;
    match contents {
        None => Ok(None),
        Some(contents) => {
            let mut response_table = lua.create_table()?;
            match contents {
                KernelContent::DisplayData(display_data) => {}
                KernelContent::ExecuteResultContent(result_content) => {}
            };

            Ok(Some(response_table))
        }
    }
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
