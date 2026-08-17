# Conservative no-float scan for the exact M0/M1 TOML/JSON manifest set.
#
# It removes ordinary single-line basic/literal strings and comments, then
# scans every remaining token on every line, including continuation lines in
# arrays and inline tables.  It conservatively rejects numeric dotted keys or
# fractional datetimes too.  Triple-quoted or unterminated strings fail closed:
# admitting those would require a TOML lexer rather than silently weakening
# this line-oriented gate.  The same stripped-token rule applies to the pinned
# JSON Lake manifest; JSON has no comments or single-quoted strings, so this is
# a conservative superset of its syntax.

function bare_toml(line,    out, quote, escaped, i, ch) {
    out = ""
    quote = ""
    escaped = 0
    for (i = 1; i <= length(line); i++) {
        ch = substr(line, i, 1)
        if (quote == "\"") {
            if (escaped) {
                escaped = 0
            } else if (ch == "\\") {
                escaped = 1
            } else if (ch == "\"") {
                quote = ""
            }
        } else if (quote == "'") {
            if (ch == "'") {
                quote = ""
            }
        } else if (ch == "#") {
            break
        } else if (ch == "\"" || ch == "'") {
            quote = ch
        } else {
            out = out ch
        }
    }
    if (quote != "") {
        unsupported_string = 1
    }
    return out
}

{
    unsupported_string = 0
    if (index($0, "\"\"\"") != 0 || index($0, "'''" ) != 0) {
        printf "%s:%d: unsupported multiline TOML string in no-float scope\n", FILENAME, FNR
        failed = 1
        next
    }
    bare = bare_toml($0)
    if (unsupported_string) {
        printf "%s:%d: unterminated TOML string in no-float scope\n", FILENAME, FNR
        failed = 1
        next
    }
    if (bare ~ /(^|[^[:alnum:]_])([+-]?(([0-9][0-9_]*\.[0-9_]+([eE][+-]?[0-9_]+)?)|([0-9][0-9_]*[eE][+-]?[0-9_]+)|(inf|nan)))([^[:alnum:]_]|$)/) {
        printf "%s:%d: bare TOML float-like token: %s\n", FILENAME, FNR, $0
        failed = 1
    }
}

END {
    exit failed
}
