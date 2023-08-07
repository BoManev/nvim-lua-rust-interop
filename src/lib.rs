use mlua::prelude::{Lua, LuaResult, LuaTable};

fn greet_people(lua: &Lua, names: Vec<String>) -> LuaResult<LuaTable> {
    let strings = lua.create_table()?;
    for (i, name) in names.into_iter().enumerate() {
        // i + 1 because Lua indexing starts at 1 instead of 0
        strings.raw_insert((i + 1).try_into().unwrap(), format!("Hello {}!", name))?;
    }

    Ok(strings)
}

#[mlua::lua_module]
fn norgberg(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("greet_people", lua.create_function(greet_people)?)?;
    Ok(exports)
}
