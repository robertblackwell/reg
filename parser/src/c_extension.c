/**
 * This file contains c functions that extend the functions provided by http-parser.
 * Generally they provide more convenient Rust access the data in struct http_parser
 * and thus extend the functions provided by http-parser
 */
#include "../http-parser/http_parser.h"

/*
Realigns a bit field struct in a predictable way.
*/
uint32_t http_get_struct_flags(const http_parser *state)
{
  return state->status_code |
    (state->method << 16) |
    (state->http_errno << 24) |
    (state->upgrade << 31);
}

int ex_http_parser_errno(const http_parser* p)
{
    return p->http_errno;
}

int ex_http_parser_method(const http_parser* p)
{
    return p->method;
}
int ex_http_parser_status_code(const http_parser* p)
{
    return p->status_code;
}
int ex_http_parser_is_upgrade(const http_parser* p)
{
    return p->upgrade;
}
///
/// these _set functions are for testing only
///
void ex_http_parser_errno_set(http_parser* p, enum http_errno errno)
{
    p->http_errno = errno;
}
void ex_http_parser_method_set(http_parser* p, enum http_method m)
{
    p->method = m;
}
void ex_http_parser_status_code_set(http_parser* p, int sc)
{
    p->status_code = sc;
}
void ex_http_parser_is_upgrade_set(http_parser* p, int upg)
{
    p->upgrade = upg;
}

size_t ex_http_parser_struct_sizeof()
{
    return sizeof(struct http_parser);
}

size_t ex_http_parser_settings_struct_sizeof()
{
    return sizeof(struct http_parser_settings);
}
