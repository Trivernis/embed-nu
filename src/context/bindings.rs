use nu_command::*;
use nu_protocol::engine::{EngineState, StateWorkingSet};

use crate::error::CrateResult;

macro_rules! bind_commands {
            ($engine_state:expr, $( $command:expr),* $(,)? ) => {
                bind($engine_state, |working_set| {
                        $( working_set.add_decl(Box::new($command)); )*
                })
            };
        }

pub fn bind_core_commands(engine_state: &mut EngineState) -> CrateResult<()> {
    bind_commands!(
        engine_state,
        Alias,
        Ast,
        Break,
        Commandline,
        Continue,
        Debug,
        Def,
        DefEnv,
        Describe,
        Do,
        Echo,
        ErrorMake,
        ExportAlias,
        ExportCommand,
        ExportDef,
        ExportDefEnv,
        ExportExtern,
        ExportUse,
        Extern,
        For,
        Help,
        HelpOperators,
        Hide,
        HideEnv,
        If,
        Ignore,
        Overlay,
        OverlayUse,
        OverlayList,
        OverlayNew,
        OverlayHide,
        Let,
        Metadata,
        Module,
        Mut,
        Return,
        Try,
        Use,
        Version,
        While,
    )
}

pub fn bind_chart_commands(engine_state: &mut EngineState) -> CrateResult<()> {
    bind_commands!(engine_state, Histogram)
}

pub fn bind_filter_commands(engine_state: &mut EngineState) -> CrateResult<()> {
    bind_commands! {
    engine_state,
    All,
    Any,
    Append,
    Collect,
    Columns,
    Compact,
    Default,
    Drop,
    DropColumn,
    DropNth,
    Each,
    EachWhile,
    Empty,
    Every,
    Find,
    First,
    Flatten,
    Get,
    Group,
    GroupBy,
    Headers,
    Insert,
    SplitBy,
    Take,
    Merge,
    Move,
    TakeWhile,
    TakeUntil,
    Last,
    Length,
    Lines,
    ParEach,
    Prepend,
    Range,
    Reduce,
    Reject,
    Rename,
    Reverse,
    Roll,
    RollDown,
    RollUp,
    RollLeft,
    RollRight,
    Rotate,
    Select,
    Shuffle,
    Skip,
    SkipUntil,
    SkipWhile,
    Sort,
    SortBy,
    SplitList,
    Transpose,
    Uniq,
    Upsert,
    Update,
    UpdateCells,
    Where,
    Window,
    Wrap,
    Zip,    }
}

pub fn bind_misc_commands(engine_state: &mut EngineState) -> CrateResult<()> {
    bind_commands!(engine_state, History, Tutor, HistorySession)
}

pub fn bind_path_commands(engine_state: &mut EngineState) -> CrateResult<()> {
    bind_commands! {
        engine_state,
        Path,
        PathBasename,
        PathDirname,
        PathExists,
        PathExpand,
        PathJoin,
        PathParse,
        PathRelativeTo,
        PathSplit,
        PathType,
    }
}

#[cfg(windows)]
pub fn bind_system_commands(engine_state: &mut EngineState) -> CrateResult<()> {
    bind_commands! {
        engine_state,
        Benchmark,
        Complete,
        External,
        NuCheck,
        Sys,
        Ps,
        Which,
        RegistryQuery
    }
}

#[cfg(unix)]
pub fn bind_system_commands(engine_state: &mut EngineState) -> CrateResult<()> {
    bind_commands! {
        engine_state,
        Benchmark,
        Complete,
        External,
        NuCheck,
        Sys,
        Ps,
        Which,
        Exec
    }
}

pub fn bind_string_commands(engine_state: &mut EngineState) -> CrateResult<()> {
    bind_commands! {
        engine_state,
        Char,
        Decode,
        Encode,
        DecodeBase64,
        EncodeBase64,
        DetectColumns,
        Format,
        FileSize,
        Parse,
        Size,
        Split,
        SplitChars,
        SplitColumn,
        SplitRow,
        SplitWords,
        Str,
        StrCamelCase,
        StrCapitalize,
        StrCollect,
        StrContains,
        StrDistance,
        StrDowncase,
        StrEndswith,
        StrJoin,
        StrReplace,
        StrIndexOf,
        StrKebabCase,
        StrLength,
        StrLpad,
        StrPascalCase,
        StrReverse,
        StrRpad,
        StrScreamingSnakeCase,
        StrSnakeCase,
        StrStartsWith,
        StrSubstring,
        StrTrim,
        StrTitleCase,
        StrUpcase,
    }
}

pub fn bind_bit_commands(engine_state: &mut EngineState) -> CrateResult<()> {
    bind_commands! {
        engine_state,
        Bits,
        BitsAnd,
        BitsNot,
        BitsOr,
        BitsXor,
        BitsRotateLeft,
        BitsRotateRight,
        BitsShiftLeft,
        BitsShiftRight,
    }
}

pub fn bind_byte_commands(engine_state: &mut EngineState) -> CrateResult<()> {
    bind_commands! {
        engine_state,
        Bytes,
        BytesLen,
        BytesStartsWith,
        BytesEndsWith,
        BytesReverse,
        BytesReplace,
        BytesAdd,
        BytesAt,
        BytesIndexOf,
        BytesCollect,
        BytesRemove,
        BytesBuild,
    }
}

pub fn bind_file_system_commands(engine_state: &mut EngineState) -> CrateResult<()> {
    bind_commands! {
        engine_state,
        Cd,
        Cp,
        Ls,
        Mkdir,
        Mv,
        Open,
        Rm,
        Save,
        Touch,
        Glob,
        Watch,
    }
}

pub fn bind_platform_commands(engine_state: &mut EngineState) -> CrateResult<()> {
    bind_commands! {
        engine_state,
        Ansi,
        AnsiGradient,
        AnsiStrip,
        Clear,
        Du,
        KeybindingsDefault,
        Input,
        KeybindingsListen,
        Keybindings,
        Kill,
        KeybindingsList,
        Sleep,
        TermSize,
    }
}

pub fn bind_date_commands(engine_state: &mut EngineState) -> CrateResult<()> {
    bind_commands! {
        engine_state,
        Date,
        DateFormat,
        DateHumanize,
        DateListTimezones,
        DateNow,
        DateToRecord,
        DateToTable,
        DateToTimezone,
    }
}

pub fn bind_shell_commands(engine_state: &mut EngineState) -> CrateResult<()> {
    bind_commands! {
        engine_state,
        Enter,
        Exit,
        GotoShell,
        NextShell,
        PrevShell,
        Shells,
    }
}

pub fn bind_format_commands(engine_state: &mut EngineState) -> CrateResult<()> {
    bind_commands! {
        engine_state,
        From,
        FromCsv,
        FromEml,
        FromIcs,
        FromIni,
        FromJson,
        FromNuon,
        FromOds,
        FromSsv,
        FromToml,
        FromTsv,
        FromUrl,
        FromVcf,
        FromXlsx,
        FromXml,
        FromYaml,
        FromYml,
        To,
        ToCsv,
        ToHtml,
        ToJson,
        ToMd,
        ToNuon,
        ToText,
        ToToml,
        ToTsv,
        Touch,
        Use,
        Upsert,
        Where,
        ToXml,
        ToYaml,
    }
}

pub fn bind_viewer_commands(engine_state: &mut EngineState) -> CrateResult<()> {
    bind_commands! {
        engine_state,
        Griddle,
        Table,
    }
}

pub fn bind_conversion_commands(engine_state: &mut EngineState) -> CrateResult<()> {
    bind_commands! {
        engine_state,
        Fmt,
        Into,
        IntoBool,
        IntoBinary,
        IntoDatetime,
        IntoDecimal,
        IntoDuration,
        IntoFilesize,
        IntoInt,
        IntoRecord,
        IntoString,
    }
}

pub fn bind_environment_commands(engine_state: &mut EngineState) -> CrateResult<()> {
    bind_commands! {
        engine_state,
        Env,
        ExportEnv,
        LetEnv,
        LoadEnv,
        SourceEnv,
        WithEnv,
        // nu config commands have been removed as editing isn't possible
        // in this environment
    }
}

pub fn bind_math_commands(engine_state: &mut EngineState) -> CrateResult<()> {
    bind_commands! {
        engine_state,
        Math,
        MathAbs,
        MathAvg,
        MathCeil,
        MathFloor,
        MathMax,
        MathMedian,
        MathMin,
        MathMode,
        MathProduct,
        MathRound,
        MathSqrt,
        MathStddev,
        MathSum,
        MathVariance,
    }
}

pub fn bind_network_commands(engine_state: &mut EngineState) -> CrateResult<()> {
    bind_commands! {
        engine_state,
        Url,
        UrlParse,
        Port,
    }
}

pub fn bind_random_commands(engine_state: &mut EngineState) -> CrateResult<()> {
    bind_commands! {
        engine_state,
        Random,
        RandomBool,
        RandomChars,
        RandomDecimal,
        RandomDice,
        RandomInteger,
        RandomUuid,
    }
}

pub fn bind_generator_commands(engine_state: &mut EngineState) -> CrateResult<()> {
    bind_commands! {
        engine_state,
        Cal,
        Seq,
        SeqDate,
        SeqChar,
    }
}

pub fn bind_hash_commands(engine_state: &mut EngineState) -> CrateResult<()> {
    bind_commands! {
        engine_state,
        Hash,
        HashMd5::default(),
        HashSha256::default(),
    }
}

pub fn bind_experimental_commands(engine_state: &mut EngineState) -> CrateResult<()> {
    bind_commands! {
        engine_state,
        ViewSource,
        IsAdmin,
    }
}

#[inline]
fn bind<F: Fn(&mut StateWorkingSet)>(
    engine_state: &mut EngineState,
    bind_fn: F,
) -> CrateResult<()> {
    let mut working_set = StateWorkingSet::new(engine_state);
    bind_fn(&mut working_set);
    let delta = working_set.render();
    engine_state.merge_delta(delta)?;
    Ok(())
}
