#![allow(clippy::cargo_common_metadata)]

use lune_utils::fmt::{pretty_format_multi_value, ValueFormatConfig};
use mlua::prelude::*;
use mlua_luau_scheduler::LuaSpawnExt;

use tokio::io::{stderr, stdin, stdout, AsyncReadExt, AsyncWriteExt};

use rawkey::RawKey;

use lune_utils::TableBuilder;

mod key_inputs;
mod prompt;
mod style_and_color;

use self::key_inputs::KeyKind;
use self::prompt::{prompt, PromptOptions, PromptResult};
use self::style_and_color::{ColorKind, StyleKind};

const FORMAT_CONFIG: ValueFormatConfig = ValueFormatConfig::new()
    .with_max_depth(4)
    .with_colors_enabled(false);

/**
    Creates the `stdio` standard library module.

    # Errors

    Errors when out of memory.
*/
pub fn module(lua: &Lua) -> LuaResult<LuaTable> {
    TableBuilder::new(lua)?
        .with_function("color", stdio_color)?
        .with_function("style", stdio_style)?
        .with_function("format", stdio_format)?
        .with_async_function("write", stdio_write)?
        .with_async_function("ewrite", stdio_ewrite)?
        .with_async_function("readToEnd", stdio_read_to_end)?
        .with_async_function("prompt", stdio_prompt)?
        .with_function("isKeyPressed", stdio_is_key_pressed)?
        .build_readonly()
}

fn stdio_color(lua: &Lua, color: ColorKind) -> LuaResult<LuaValue> {
    color.ansi_escape_sequence().into_lua(lua)
}

fn stdio_style(lua: &Lua, style: StyleKind) -> LuaResult<LuaValue> {
    style.ansi_escape_sequence().into_lua(lua)
}

fn stdio_format(_: &Lua, args: LuaMultiValue) -> LuaResult<String> {
    Ok(pretty_format_multi_value(&args, &FORMAT_CONFIG))
}

async fn stdio_write(_: &Lua, s: LuaString<'_>) -> LuaResult<()> {
    let mut stdout = stdout();
    stdout.write_all(s.as_bytes()).await?;
    stdout.flush().await?;
    Ok(())
}

async fn stdio_ewrite(_: &Lua, s: LuaString<'_>) -> LuaResult<()> {
    let mut stderr = stderr();
    stderr.write_all(s.as_bytes()).await?;
    stderr.flush().await?;
    Ok(())
}

/*
    FUTURE: Figure out how to expose some kind of "readLine" function using a buffered reader.

    This is a bit tricky since we would want to be able to use **both** readLine and readToEnd
    in the same script, doing something like readLine, readLine, readToEnd from lua, and
    having that capture the first two lines and then read the rest of the input.
*/

async fn stdio_read_to_end(lua: &Lua, (): ()) -> LuaResult<LuaString> {
    let mut input = Vec::new();
    let mut stdin = stdin();
    stdin.read_to_end(&mut input).await?;
    lua.create_string(&input)
}

async fn stdio_prompt(lua: &Lua, options: PromptOptions) -> LuaResult<PromptResult> {
    lua.spawn_blocking(move || prompt(options))
        .await
        .into_lua_err()
}

fn stdio_is_key_pressed(_: &Lua, key: KeyKind) -> LuaResult<bool> {
    let rawkey = RawKey::new();

    if rawkey.is_pressed(key.key_code_object()) {
        Ok(true)
    } else {
        Ok(false)
    }
}
