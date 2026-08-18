# Fail closed on ordinary decimal/exponent/inf/nan numeric tokens in Metal
# source.  The lexer masks comments and strings, and rejects character
# literals/digit separators before token detection.  Type-family rejection is
# a separate gate in check.sh.

function raw_string_prefix_length(line, position,    previous, remaining) {
    if (position > 1) {
        previous = substr(line, position - 1, 1)
        if (previous ~ /[[:alnum:]_]/) {
            return 0
        }
    }

    remaining = substr(line, position)
    if (remaining ~ /^u8R"/) {
        return 4
    }
    if (remaining ~ /^(uR|UR|LR)"/) {
        return 3
    }
    if (remaining ~ /^R"/) {
        return 2
    }
    return 0
}

function ends_in_line_splice(line,    candidate, line_length) {
    candidate = line
    sub(/[[:space:]]+$/, "", candidate)
    line_length = length(candidate)
    if (line_length >= 1 && substr(candidate, line_length, 1) == "\\") {
        return 1
    }
    # In C++14 translation phase 1, ??/ is the trigraph spelling of a
    # backslash and can therefore create the same phase-2 splice.
    if (line_length >= 3 && substr(candidate, line_length - 2, 3) == "??/") {
        return 1
    }
    return 0
}

function is_required_include(line, line_number) {
    return line_number == 1 && line == "#include <metal_stdlib>"
}

function has_preprocessor_marker(source) {
    # %: is the C++ digraph for #; ??= is the pre-C++17 trigraph for #.
    return index(source, "#") != 0 || index(source, "%:") != 0 || index(source, "??=") != 0
}

function lex_code(line, filename, line_number, diagnose,    output, position, pair, char, quote, quote_kind, raw_length) {
    output = ""
    position = 1
    quote = ""
    quote_kind = ""
    lex_error = 0

    # C/C++ translation phase 2 removes backslash-newline before comments and
    # tokens are recognized.  Reject the splice at the physical-line boundary
    # so split numeric tokens cannot evade this scanner.
    if (ends_in_line_splice(line)) {
        lex_error = 1
        if (diagnose) {
            print filename ":" line_number ": MSL backslash-newline splice is forbidden" > "/dev/stderr"
        }
        return output
    }

    while (position <= length(line)) {
        pair = substr(line, position, 2)
        char = substr(line, position, 1)

        if (in_block_comment) {
            if (pair == "*/") {
                output = output "  "
                in_block_comment = 0
                position += 2
            } else {
                output = output " "
                position++
            }
        } else if (quote != "") {
            if (char == "\\") {
                output = output " "
                position++
                if (position <= length(line)) {
                    output = output " "
                    position++
                }
            } else if (char == quote) {
                output = output " "
                quote = ""
                quote_kind = ""
                position++
            } else {
                output = output " "
                position++
            }
        } else if (pair == "/*") {
            output = output "  "
            in_block_comment = 1
            block_filename = filename
            block_line_number = line_number
            position += 2
        } else if (pair == "//") {
            break
        } else if ((raw_length = raw_string_prefix_length(line, position)) != 0) {
            lex_error = 1
            if (diagnose) {
                print filename ":" line_number ": MSL raw string literal prefix is forbidden" > "/dev/stderr"
            }
            return output
        } else if (char == "\"") {
            output = output " "
            quote = char
            quote_kind = "string"
            position++
        } else if (char == "'") {
            # Apostrophes are both character delimiters and C++ digit
            # separators.  The frozen kernels need neither, so reject every
            # occurrence outside comments and double-quoted strings.
            lex_error = 1
            if (diagnose) {
                print filename ":" line_number ": MSL apostrophe is forbidden" > "/dev/stderr"
            }
            return output
        } else {
            output = output char
            position++
        }
    }

    if (quote != "") {
        lex_error = 1
        if (diagnose) {
            print filename ":" line_number ": unterminated MSL " quote_kind " literal" > "/dev/stderr"
        }
    }
    return output
}

function has_forbidden_number(source,    decimal, hexfloat, special, lowered) {
    decimal = "(^|[^[:alnum:]_.])((([0-9][0-9_]*\\.[0-9_]*|\\.[0-9][0-9_]*)([eE][+-]?[0-9_]+)?)|([0-9][0-9_]*[eE][+-]?[0-9_]+))"
    hexfloat = "(^|[^[:alnum:]_.])0[xX](([0-9a-fA-F_]+(\\.[0-9a-fA-F_]*)?)|(\\.[0-9a-fA-F_]+))[pP][+-]?[0-9_]+"
    special = "(^|[^[:alnum:]_])(inf|infinity|nan)([^[:alnum:]_]|$)"
    lowered = tolower(source)
    return source ~ decimal || source ~ hexfloat || lowered ~ special
}

function self_check(label, line, want_number, want_lex_error, want_open_comment,    source, got_number) {
    in_block_comment = 0
    lex_error = 0
    source = lex_code(line, "<MSL scanner self-test>", 1, 0)
    got_number = has_forbidden_number(source)
    if (got_number != want_number || lex_error != want_lex_error || in_block_comment != want_open_comment) {
        print "MSL scanner self-test failed: " label > "/dev/stderr"
        self_test_failed = 1
    }
}

function self_check_spliced_number(    first_error, second_error) {
    in_block_comment = 0
    lex_error = 0
    lex_code("auto x = 1e\\", "<MSL scanner self-test>", 1, 0)
    first_error = lex_error
    lex_code("5;", "<MSL scanner self-test>", 2, 0)
    second_error = lex_error
    if (!first_error || second_error || in_block_comment) {
        print "MSL scanner self-test failed: two-line exponent splice" > "/dev/stderr"
        self_test_failed = 1
    }
}

function self_check_preprocessor(label, line, line_number, want_rejected,    source, rejected) {
    in_block_comment = 0
    lex_error = 0
    source = lex_code(line, "<MSL scanner self-test>", line_number, 0)
    rejected = lex_error || (line_number == 1 && !is_required_include(line, line_number)) || (has_preprocessor_marker(source) && !is_required_include(line, line_number))
    if (rejected != want_rejected || in_block_comment) {
        print "MSL scanner self-test failed: " label > "/dev/stderr"
        self_test_failed = 1
    }
}

BEGIN {
    self_check("leading-dot decimal", "auto x = .5;", 1, 0, 0)
    self_check("leading-dot exponent", "auto x = .5e2;", 1, 0, 0)
    self_check("leading-dot hex float", "auto x = 0x.8p1;", 1, 0, 0)
    self_check("line marker in string", "static_assert(true, \"//\"); auto x = 1.5;", 1, 0, 0)
    self_check("block marker in string", "const char *s = \"/*\"; auto x = .5;", 1, 0, 0)
    self_check("character literal rejected", "static_assert('//'); auto x = .5e2;", 0, 1, 0)
    self_check("escaped quote before marker", "const char *s = \"\\\"//\"; auto x = 1.5;", 1, 0, 0)
    self_check("escaped character rejected", "auto c = '\\\\'; auto x = .5;", 0, 1, 0)
    self_check("digit-separator bypass", "auto x = 1'2.3'4;", 0, 1, 0)
    self_check("raw string balancing bypass", "const char *a = R\"( \" )\"; auto x = 1.5; const char *b = R\"( \" )\";", 0, 1, 0)
    self_check("u8 raw string prefix", "const char *s = u8R\"(raw)\";", 0, 1, 0)
    self_check("u raw string prefix", "const char *s = uR\"(raw)\";", 0, 1, 0)
    self_check("U raw string prefix", "const char *s = UR\"(raw)\";", 0, 1, 0)
    self_check("L raw string prefix", "const char *s = LR\"(raw)\";", 0, 1, 0)
    self_check_spliced_number()
    self_check("CRLF exponent splice", "auto x = 1e\\\r", 0, 1, 0)
    self_check("whitespace exponent splice", "auto x = 1e\\ \t", 0, 1, 0)
    self_check("trigraph exponent splice", "auto x = 1e??/", 0, 1, 0)
    self_check("numbers only in string", "const char *s = \"1.5 // /*\";", 0, 0, 0)
    self_check("numbers only in comments", "/* 1.5 .5e2 0x.8p1 */ uint x = 1; // nan", 0, 0, 0)
    self_check("unterminated string", "const char *s = \"unterminated", 0, 1, 0)
    self_check("unterminated character", "auto c = 'x;", 0, 1, 0)
    self_check("unterminated block comment", "uint x = 1; /* unterminated", 0, 0, 1)
    self_check_preprocessor("required metal include", "#include <metal_stdlib>", 1, 0)
    self_check_preprocessor("quoted hidden header", "#include \"hidden.h\"", 2, 1)
    self_check_preprocessor("include-next directive", "#include_next <hidden.h>", 2, 1)
    self_check_preprocessor("import directive", "#import \"hidden.h\"", 2, 1)
    self_check_preprocessor("other directive", "#define HIDDEN 1", 2, 1)
    in_block_comment = 0
    lex_error = 0
    if (self_test_failed) {
        failed = 1
    }
}

FNR == 1 {
    if (NR > 1 && in_block_comment) {
        print block_filename ":" block_line_number ": unterminated MSL block comment before " FILENAME > "/dev/stderr"
        failed = 1
    }
    in_block_comment = 0
}

{
    source = lex_code($0, FILENAME, FNR, 1)
    if (lex_error) {
        failed = 1
    }
    if (FNR == 1 && !is_required_include($0, FNR)) {
        print FILENAME ":1: first physical line must be exactly #include <metal_stdlib>" > "/dev/stderr"
        failed = 1
    }
    if (has_preprocessor_marker(source) && !is_required_include($0, FNR)) {
        print FILENAME ":" FNR ": MSL preprocessor directive is forbidden" > "/dev/stderr"
        failed = 1
    }
    if (has_forbidden_number(source)) {
        print FILENAME ":" FNR ": " source > "/dev/stderr"
        failed = 1
    }
}

END {
    if (in_block_comment) {
        print block_filename ":" block_line_number ": unterminated MSL block comment" > "/dev/stderr"
        failed = 1
    }
    exit failed
}
