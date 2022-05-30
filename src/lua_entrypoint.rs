use mlua::prelude::*;

async fn start_kernel(
    _lua: &Lua,
    (jupyter_base_url, kernel_name): (String, String),
) -> LuaResult<String> {
    //let result = Runtime::new().unwrap().block_on(build_client_and_request(
    //    &cmd_name,
    //    server::default_socket_path(),
    //    input,
    //));
    //match result {
    //    Ok(result) => Ok(result.output),
    //    Err(e) => Ok(format!("error:{}", e)),
    //}
    unimplemented!()
}

async fn list_kernels(_lua: &Lua, jupyter_base_url: String) -> LuaResult<Vec<String>> {
    unimplemented!()
}

async fn send_code(
    _lua: &Lua,
    (jupyter_url, kernel, code): (String, String, String),
) -> LuaResult<LuaTable<'_>> {
    unimplemented!()
}

#[mlua::lua_module]
fn run_jupyter(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("start_kernel", lua.create_async_function(start_kernel)?)?;
    exports.set("list_kernels", lua.create_async_function(list_kernels)?)?;
    exports.set("send_code", lua.create_async_function(send_code)?)?;
    Ok(exports)
}
