#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

#ifdef __WATCOMC__
// force SJIS mode for OpenWatcom Compiler
#define SJIS_MODE
#endif

typedef int64_t m_int;

enum {
  M_INIT_STRSIZE = 0x100,
};

// primitive types

enum {
  M_INTEGER,
  M_STRING,
  M_BOOL,
  M_NIL,
} maysick_type;

typedef struct {
  size_t size;
  size_t len;
  char * head;
} m_string;

typedef struct {
  uint16_t type;
  union {
    m_int     integer;
    m_string *string;
    bool      boolean;
  } entity;
} maysick_any;

// macro for dropping value
#define m_any_nil(a) m_any_nil_()

maysick_any m_any_int(m_int i);
maysick_any m_any_string(m_string *s);
maysick_any m_any_bool(bool b);
maysick_any m_any_nil_();

typedef struct m_arc {
  size_t        count;
  maysick_any   obj;
  struct m_arc *prev;
  struct m_arc *next;
} m_arc;

m_arc *GLOBAL_ARC = NULL;

m_arc *     m_arc_new(maysick_any obj);
maysick_any m_arc_count(maysick_any obj);
m_string *  m_arc_count_s(m_string *s);
maysick_any m_arc_detach(maysick_any obj);
void        m_arc_detach_all();
maysick_any m_arc_move(maysick_any obj);
maysick_any m_arc_autorelease(m_arc **arc, maysick_any obj);
m_string *  m_arc_autorelease_s(m_arc **arc, m_string *str);

#define __aligned(n, a) (n & (~a)) + ((n & a) != 0 ? (a + 1) : 0)
#define aligned(n, align) (__aligned((n), (align - 1)))

m_string *m_string_with_capacity(size_t size) {
  m_string *str = calloc(1, sizeof(m_string));

  str->len  = 0;
  str->size = size < M_INIT_STRSIZE ? M_INIT_STRSIZE : aligned(size, 16);
  str->head = calloc(str->size, 1);

  // count string
  m_arc_count_s(str);

  return str;
}

m_string *m_string_new() { return m_string_with_capacity(0); }

m_string *m_string_from_cstr(const char *cstr) {
  size_t    len = strlen(cstr);
  m_string *s   = m_string_with_capacity(len);

  s->len = len;
  memcpy(s->head, cstr, len);

  m_arc_move(m_any_string(s));
  return s;
}

void m_string_free(m_string *str) {
  free(str->head);
  free(str);
}

void m_string_expand(m_string *str) {
  char *head_new = calloc(str->size << 1, 1);

  memcpy(head_new, str->head, str->len);
  free(str->head);

  str->head = head_new;
  str->size = str->size << 1;
}

void m_string_expand_if_needed(m_string *str) {
  if (aligned(str->len, 16) <= str->size)
    m_string_expand(str);
}

m_string *m_string_concat(m_string *a, m_string *b) {
  m_string *c = m_string_with_capacity(a->len + b->len);

  memcpy(c->head, a->head, a->len);
  memcpy(&c->head[a->len], b->head, b->len);

  c->len = a->len + b->len;

  return c;
}

// type-cast for compiler-generated code

maysick_any m_any_int(m_int i) {
  maysick_any a;
  a.type           = M_INTEGER;
  a.entity.integer = i;
  return a;
}

maysick_any m_any_string(m_string *s) {
  maysick_any a;
  a.type          = M_STRING;
  a.entity.string = s;
  return a;
}

maysick_any m_any_bool(bool b) {
  maysick_any a;
  a.type           = M_BOOL;
  a.entity.boolean = b;
  return a;
}

maysick_any m_any_nil_() {
  maysick_any a;
  a.type = M_NIL;
  return a;
}

m_string *m_to_string(maysick_any a) {
  m_string *s;

  switch (a.type) {
  case M_STRING:
    return a.entity.string;
    break;
  case M_INTEGER:
    s = m_string_new();
    sprintf(s->head, "%lld", a.entity.integer);
    return s;
    break;
  case M_BOOL:
    return m_string_from_cstr(a.entity.boolean ? "true" : "false");
    break;
  case M_NIL:
    return m_string_from_cstr("(Nil)");
    break;
  default:
    return m_string_from_cstr("(cast error)");
    break;
  }
}

m_int m_to_integer(maysick_any a) {
  int i;

  switch (a.type) {
  case M_STRING:
    sscanf(a.entity.string->head, "%d", &i);
    return (m_int)i;
    break;
  case M_INTEGER:
    return a.entity.integer;
    break;
  case M_BOOL:
    return a.entity.boolean;
    break;
  case M_NIL:
    return 0;
    break;
  default:
    return 0;
    break;
  }
}

// auto reference counter

m_arc *m_arc_new(maysick_any obj) {
  m_arc *arc = calloc(1, sizeof(m_arc));
  arc->obj   = obj;
  arc->count = 1;
  return arc;
}

m_arc *m_arc_find(maysick_any obj, m_arc *arc) {
  if (obj.type != M_STRING)
    return NULL;

  if (!arc)
    return NULL;

  if (obj.entity.string == arc->obj.entity.string) {
    return arc;
  }

  return m_arc_find(obj, arc->next);
}

maysick_any m_arc_count(maysick_any obj) {
  m_arc *a;

  if (obj.type != M_STRING)
    return obj;

  a = m_arc_find(obj, GLOBAL_ARC);

  if (!a) {
    m_arc *b = m_arc_new(obj);
    b->prev  = NULL;
    b->next  = GLOBAL_ARC;
    if (GLOBAL_ARC) {
      GLOBAL_ARC->prev = b;
    }
    GLOBAL_ARC = b;
    return obj;
  }

  a->count++;
  return obj;
}

m_string *m_arc_count_s(m_string *s) {
  m_arc_count(m_any_string(s));
  return s;
}

maysick_any m_arc_move(maysick_any obj) {
  m_arc *a;

  if (obj.type != M_STRING)
    return obj;

  a = m_arc_find(obj, GLOBAL_ARC);

  if (!a)
    return obj;

  a->count--;

  return obj;
}

maysick_any m_arc_detach(maysick_any obj) {
  m_arc *a;

  if (obj.type != M_STRING)
    return obj;

  a = m_arc_find(obj, GLOBAL_ARC);

  if (!a)
    return obj;

  a->count--;

  if (a->count == 0) {
    // purge from list
    if (a->prev) {
      a->prev->next = a->next;
      a->next->prev = a->prev;
      m_string_free(a->obj.entity.string);
      free(a);
    } else {
      GLOBAL_ARC = a->next;
      m_string_free(a->obj.entity.string);
      free(a);
    }
  }

  return m_any_nil();
}

void m_arc_detach_all() {
  while (GLOBAL_ARC) {
    m_arc *a   = GLOBAL_ARC;
    GLOBAL_ARC = a->next;
    m_string_free(a->obj.entity.string);
    free(a);
  }
}

maysick_any m_arc_autorelease(m_arc **arc, maysick_any obj) {
  m_arc *a;

  if (obj.type != M_STRING)
    return obj;

  m_arc_count(obj);

  // m_arc_count(obj);
  a = m_arc_find(obj, *arc);

  if (!a) {
    m_arc *b = m_arc_new(obj);
    b->prev  = NULL;
    b->next  = *arc;
    if (b->next) {
      b->next->prev = b;
    }
    *arc = b;
    return obj;
  }

  a->count++;

  return obj;
}

m_string *m_arc_autorelease_s(m_arc **arc, m_string *s) {
  m_arc_autorelease(arc, m_any_string(s));
  return s;
}

void m_arc_release(m_arc *arc) {
  while (arc) {
    m_arc *a = arc;
    arc      = a->next;
    m_arc_detach(a->obj);
    free(a);
  }
}

// prototype of built-in functions

void      _mS__print(m_string *msg);
void      _mS__println(m_string *msg);
m_int     _m_i_random();
m_string *_m_S_getchar();
m_string *_m_S_readline();

m_int     _m_i_slen(m_string *str);
m_int     _mSi_i_char_at(m_string *str, m_int pos);
m_string *_mi_S_char_from(m_int c);
m_string *_mi_S_integer_as_hex(m_int c);

// implementation of built-in functions

void _mS__print(m_string *msg) { printf("%s", msg->head); }
void _mS__println(m_string *msg) { printf("%s\n", msg->head); }

m_int _m_i_random() { return (m_int)rand(); }

m_string *_m_S_getchar() {
  m_string *s = m_string_new();
  int       c = getchar();
  int       i;

  if (c == -1) {
    m_arc_move(m_any_string(s));
    return s; // EOF fallback
  }

#ifdef SJIS_MODE
  // Watcom C (which may use Shift_JIS)
  if (c <= 0x7F || (0xA1 <= c && 0xDF <= c)) {
    s->head[0] = c;
    s->len     = 1;
  } else {
    s->head[0] = c;
    s->head[1] = getchar();
    s->len     = 2;
  }
#else
  // gcc or compatible (which uses UTF-8 internally)
  if (c <= 0x7F) {
    s->head[0] = c;
    s->len     = 1;
  } else { // might be UTF-8
    uint32_t cnt = __builtin_clz((~c) & 0xFF) - 24;
    s->len       = cnt;
    s->head[0]   = (char)c;
    for (i = 1; i < cnt; i++) {
      s->head[i] = getchar();
    }
  }
#endif

  m_arc_move(m_any_string(s));
  return s;
}

m_string *_m_S_readline() {
  m_string *str = m_string_new();

  m_arc_move(m_any_string(str));
  return str;
}

m_int _m_i_slen(m_string *str) {
  size_t         i = 0, j = 0;
  unsigned char *s = str->head;

  for (i = 0; i < str->len; i++) {
    char c = s[j];
#ifdef SJIS_MODE
    if (c <= 0x7F || (0xA1 <= c && 0xDF <= c)) {
      j++;
    } else {
      j += 2;
    }
#else
    if (c <= 0x7F)
      j++;
    else {
      j += __builtin_clz((~c) & 0xFF) - 24;
    }
#endif
  }

  return j;
}

m_int _mSi_i_char_at(m_string *str, m_int pos) {
  size_t         i = 0, j = 0, cnt, ch;
  unsigned char *s = str->head;
  unsigned char  c;

  for (i = 0; i < pos; i++) {
    char c = s[j];
#ifdef SJIS_MODE
    if (c <= 0x7F || (0xA1 <= c && 0xDF <= c)) {
      j++;
    } else {
      j += 2;
    }
#else
    if (c <= 0x7F)
      j++;
    else {
      j += __builtin_clz((~c) & 0xFF) - 24;
    }
#endif
  }

#ifdef SJIS_MODE
  c = s[j];
  if (c <= 0x7F || (0xA1 <= c && 0xDF <= c)) {
    ch = c;
  } else {
    ch = c | (s[j + 1] << 8);
  }
#else
  c   = s[j];
  cnt = __builtin_clz((~c) & 0xFF) - 25;
  ch  = c & (0x3F >> cnt);

  if (c <= 0x7F)
    return (m_int)c;

  for (i = 0; i < cnt; i++) {
    ch = (ch << 6) | (s[++j] & 0x3F);
  }
#endif

  return (m_int)ch;
}

m_string *_mi_S_char_from(m_int c) {
  m_string *s = m_string_new();
#ifdef SJIS_MODE
  if (c <= 0xFF) {
    s->head[0] = c;
    s->len     = 1;
  } else {
    s->head[0] = c & 0xFF;
    s->head[1] = c >> 8;
    s->len     = 2;
  }
#else
  if (c <= 0x7F) {
    s->head[0] = c;
    s->len     = 1;
    return s;
  }

  if (c <= 0x07FF) {
    s->len     = 2;
    s->head[0] = 0xC0 | (c >> 6);
    s->head[1] = 0x80 | (c & 0x3F);
  } else if (c <= 0xFFFF) {
    s->len     = 3;
    s->head[0] = 0xE0 | (c >> 12);
    s->head[1] = 0x80 | ((c >> 6) & 0x3F);
    s->head[2] = 0x80 | (c & 0x3F);
  } else if (c <= 0x1FFFFF) {
    s->len     = 4;
    s->head[0] = 0xF0 | (c >> 18);
    s->head[1] = 0x80 | ((c >> 12) & 0x3F);
    s->head[2] = 0x80 | ((c >> 6) & 0x3F);
    s->head[3] = 0x80 | (c & 0x3F);
  }
#endif

  m_arc_move(m_any_string(s));
  return s;
}

m_string *_mi_S_integer_as_hex(m_int c) {
  m_string *s = m_string_new();
  sprintf(s->head, "%X", c);

  m_arc_move(m_any_string(s));
  return s;
}

// this symbol will be emitted by maysick compiler
void m_entry();

int main(int argc, const char **argv) {
  // initialize randomizer
  srand((unsigned)time(NULL));

  // jump to the entry point of maysick program.
  m_entry();

  // free all objects
  m_arc_detach_all();

  return 0;
}

// -- and the actual program goes --
