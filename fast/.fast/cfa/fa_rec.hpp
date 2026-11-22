// fast-anything recorder for C++ (C++14), force-included with -include for one run.
#ifndef FAST_ANYTHING_REC_HPP
#define FAST_ANYTHING_REC_HPP
#ifdef __cplusplus
#include <algorithm>
#include <array>
#include <chrono>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <map>
#include <string>
#include <tuple>
#include <type_traits>
#include <unordered_map>
#include <utility>
#include <vector>

namespace fa_rec {
// ---- JSON for fast-anything's C++ recorder and bridge (C++14). Generated; don't edit.
inline bool fa_utf8(const std::string &s) {
    size_t i = 0;
    while (i < s.size()) {
        unsigned char c = (unsigned char) s[i];
        int n = c < 0x80 ? 0 : (c >> 5) == 6 ? 1 : (c >> 4) == 14 ? 2 : (c >> 3) == 30 ? 3 : -1;
        if (n < 0 || i + n >= s.size() + (n == 0)) return n == 0;
        for (int k = 1; k <= n; k++)
            if (((unsigned char) s[i + k] & 0xC0) != 0x80) return false;
        i += n + 1;
    }
    return true;
}
inline bool enc_str(std::string &o, const std::string &s) {
    if (!fa_utf8(s)) return false;
    o += '"';
    for (unsigned char c : s) {
        if (c == '"' || c == '\\') { o += '\\'; o += (char) c; }
        else if (c < 0x20) { char t[8]; std::snprintf(t, sizeof t, "\\u%04x", c); o += t; }
        else o += (char) c;
    }
    o += '"';
    return true;
}
inline bool enc(std::string &o, bool v) { o += v ? "true" : "false"; return true; }
template <class T> typename std::enable_if<std::is_integral<T>::value, bool>::type enc(std::string &o, T v) { o += std::to_string(v); return true; }
template <class T> typename std::enable_if<std::is_floating_point<T>::value, bool>::type enc(std::string &o, T v) {
    if (!std::isfinite((double) v)) return false;
    char t[40];
    std::snprintf(t, sizeof t, "%.17g", (double) v);
    if (!std::strpbrk(t, ".eE")) std::strcat(t, ".0");
    o += t;
    return true;
}
inline bool enc(std::string &o, const std::string &v) { return enc_str(o, v); }
inline bool enc(std::string &o, const char *v) { if (!v) { o += "null"; return true; } return enc_str(o, v); }
template <class A, class B> bool enc(std::string &o, const std::pair<A, B> &v);
template <class T, class Al> bool enc(std::string &o, const std::vector<T, Al> &v);
template <class T, size_t N> bool enc(std::string &o, const std::array<T, N> &v);
template <class T, class C, class Al> bool enc(std::string &o, const std::map<std::string, T, C, Al> &v);
template <class T, class H, class E, class Al> bool enc(std::string &o, const std::unordered_map<std::string, T, H, E, Al> &v);
template <class It> bool enc_seq(std::string &o, It a, It b) {
    o += '[';
    for (It i = a; i != b; ++i) {
        if (i != a) o += ',';
        if (!enc(o, *i)) return false;
    }
    o += ']';
    return true;
}
template <class T, class Al> bool enc(std::string &o, const std::vector<T, Al> &v) { return enc_seq(o, v.begin(), v.end()); }
template <class T, size_t N> bool enc(std::string &o, const std::array<T, N> &v) { return enc_seq(o, v.begin(), v.end()); }
template <class A, class B> bool enc(std::string &o, const std::pair<A, B> &v) {
    o += '[';
    if (!enc(o, v.first)) return false;
    o += ',';
    if (!enc(o, v.second)) return false;
    o += ']';
    return true;
}
template <class M> bool enc_map(std::string &o, const M &m) {
    std::vector<const std::string *> keys;
    for (auto &kv : m) keys.push_back(&kv.first);
    std::sort(keys.begin(), keys.end(), [](const std::string *a, const std::string *b) { return *a < *b; });
    o += '{';
    for (size_t i = 0; i < keys.size(); i++) {
        if (i) o += ',';
        if (!enc_str(o, *keys[i])) return false;
        o += ':';
        if (!enc(o, m.at(*keys[i]))) return false;
    }
    o += '}';
    return true;
}
template <class T, class C, class Al> bool enc(std::string &o, const std::map<std::string, T, C, Al> &v) { return enc_map(o, v); }
template <class T, class H, class E, class Al> bool enc(std::string &o, const std::unordered_map<std::string, T, H, E, Al> &v) { return enc_map(o, v); }
inline bool enc_all(std::string &) { return true; }
template <class A, class... R> bool enc_all(std::string &o, const A &a, const R &...r) {
    if (o.size() > 1) o += ',';
    return enc(o, a) && enc_all(o, r...);
}

// a small DOM, for the bridge's replies
struct V {
    int kind = 0; // 0 null, 1 bool, 2 number, 3 string, 4 array, 5 object
    bool b = false;
    double num = 0;
    long long i = 0;
    bool is_int = false;
    std::string s;
    std::vector<V> arr;
    std::vector<std::pair<std::string, V>> obj;
    const V *get(const char *k) const {
        for (auto &kv : obj)
            if (kv.first == k) return &kv.second;
        return nullptr;
    }
};
inline void ws(const char *&p) { while (*p == ' ' || *p == '\n' || *p == '\r' || *p == '\t') p++; }
inline V parse(const char *&p) {
    V v;
    ws(p);
    if (*p == '{') {
        v.kind = 5;
        p++;
        ws(p);
        if (*p == '}') { p++; return v; }
        while (true) {
            ws(p);
            V k = parse(p);
            ws(p);
            p++; // :
            v.obj.emplace_back(k.s, parse(p));
            ws(p);
            if (*p++ == '}') return v;
        }
    }
    if (*p == '[') {
        v.kind = 4;
        p++;
        ws(p);
        if (*p == ']') { p++; return v; }
        while (true) {
            v.arr.push_back(parse(p));
            ws(p);
            if (*p++ == ']') return v;
        }
    }
    if (*p == '"') {
        v.kind = 3;
        p++;
        while (*p && *p != '"') {
            if (*p != '\\') { v.s += *p++; continue; }
            p++;
            char e = *p++;
            if (e == 'n') v.s += '\n';
            else if (e == 't') v.s += '\t';
            else if (e == 'r') v.s += '\r';
            else if (e == 'b') v.s += '\b';
            else if (e == 'f') v.s += '\f';
            else if (e == 'u') {
                unsigned cp = (unsigned) std::strtoul(std::string(p, 4).c_str(), nullptr, 16);
                p += 4;
                if (cp >= 0xD800 && cp < 0xDC00 && p[0] == '\\' && p[1] == 'u') {
                    unsigned lo = (unsigned) std::strtoul(std::string(p + 2, 4).c_str(), nullptr, 16);
                    p += 6;
                    cp = 0x10000 + ((cp - 0xD800) << 10) + (lo - 0xDC00);
                }
                if (cp < 0x80) v.s += (char) cp;
                else if (cp < 0x800) { v.s += (char) (0xC0 | (cp >> 6)); v.s += (char) (0x80 | (cp & 63)); }
                else if (cp < 0x10000) { v.s += (char) (0xE0 | (cp >> 12)); v.s += (char) (0x80 | ((cp >> 6) & 63)); v.s += (char) (0x80 | (cp & 63)); }
                else { v.s += (char) (0xF0 | (cp >> 18)); v.s += (char) (0x80 | ((cp >> 12) & 63)); v.s += (char) (0x80 | ((cp >> 6) & 63)); v.s += (char) (0x80 | (cp & 63)); }
            } else v.s += e;
        }
        p++;
        return v;
    }
    if (std::strncmp(p, "true", 4) == 0) { v.kind = 1; v.b = true; p += 4; return v; }
    if (std::strncmp(p, "false", 5) == 0) { v.kind = 1; p += 5; return v; }
    if (std::strncmp(p, "null", 4) == 0) { p += 4; return v; }
    v.kind = 2;
    const char *start = p;
    while (*p && std::strchr("+-0123456789.eE", *p)) p++;
    std::string num(start, p);
    v.is_int = num.find_first_of(".eE") == std::string::npos;
    v.num = std::strtod(num.c_str(), nullptr);
    v.i = v.is_int ? std::strtoll(num.c_str(), nullptr, 10) : (long long) v.num;
    return v;
}
inline void from(const V &v, bool &out) { out = v.b; }
template <class T> typename std::enable_if<std::is_integral<T>::value>::type from(const V &v, T &out) { out = (T) (v.is_int ? v.i : (long long) v.num); }
template <class T> typename std::enable_if<std::is_floating_point<T>::value>::type from(const V &v, T &out) { out = (T) (v.is_int ? (double) v.i : v.num); }
inline void from(const V &v, std::string &out) { out = v.s; }
template <class A, class B> void from(const V &v, std::pair<A, B> &out);
template <class T, class Al> void from(const V &v, std::vector<T, Al> &out);
template <class T, size_t N> void from(const V &v, std::array<T, N> &out);
template <class T, class C, class Al> void from(const V &v, std::map<std::string, T, C, Al> &out);
template <class T, class H, class E, class Al> void from(const V &v, std::unordered_map<std::string, T, H, E, Al> &out);
template <class T, class Al> void from(const V &v, std::vector<T, Al> &out) {
    out.clear();
    out.reserve(v.arr.size());
    for (auto &x : v.arr) { T t{}; from(x, t); out.push_back(std::move(t)); }
}
template <class T, size_t N> void from(const V &v, std::array<T, N> &out) {
    for (size_t i = 0; i < N && i < v.arr.size(); i++) from(v.arr[i], out[i]);
}
template <class A, class B> void from(const V &v, std::pair<A, B> &out) {
    if (v.arr.size() == 2) { from(v.arr[0], out.first); from(v.arr[1], out.second); }
}
template <class T, class C, class Al> void from(const V &v, std::map<std::string, T, C, Al> &out) {
    out.clear();
    for (auto &kv : v.obj) { T t{}; from(kv.second, t); out.emplace(kv.first, std::move(t)); }
}
template <class T, class H, class E, class Al> void from(const V &v, std::unordered_map<std::string, T, H, E, Al> &out) {
    out.clear();
    for (auto &kv : v.obj) { T t{}; from(kv.second, t); out.emplace(kv.first, std::move(t)); }
}


struct State {
    bool inside = false;
    std::vector<std::string> kept, derived;
    long long calls = 0, skipped = 0, nanos = 0, derive_started = 0;
    size_t edges = 0, edge_calls = 0;
    std::map<std::string, bool> seen;
    std::map<size_t, bool> grown;
    size_t limit = 0;
    unsigned long long rng = 0xFA57;
};
inline State &st() { static State s; return s; }
inline long long now() {
    return (long long) std::chrono::duration_cast<std::chrono::nanoseconds>(std::chrono::steady_clock::now().time_since_epoch()).count();
}
inline unsigned long long next() {
    auto &r = st().rng;
    r ^= r << 13;
    r ^= r >> 7;
    r ^= r << 17;
    return r;
}
inline void save() {
    auto &s = st();
    if (const char *path = std::getenv("FAST_ANYTHING_RECORD_OUT")) {
        if (FILE *f = std::fopen(path, "wb")) {
            for (auto &l : s.kept) std::fprintf(f, "%s\n", l.c_str());
            for (auto &l : s.derived) std::fprintf(f, "%s\n", l.c_str());
            std::fclose(f);
        }
    }
    if (const char *path = std::getenv("FAST_ANYTHING_RECORD_META")) {
        if (FILE *f = std::fopen(path, "wb")) {
            std::fprintf(f, "{\"calls\":%lld,\"kept\":%zu,\"skipped\":%lld,\"unsupported\":[%s],\"original_ns\":%.3f}", s.calls, s.kept.size(), s.skipped,
                         s.skipped ? "\"a value JSON can't hold\"" : "", s.calls ? (double) s.nanos / (double) s.calls : 0.0);
            std::fclose(f);
        }
    }
}
inline bool &registered() {
    static bool r = (std::atexit(save), true);
    return r;
}
inline bool derive_ok() {
    auto &s = st();
    const char *d = std::getenv("FAST_ANYTHING_RECORD_DERIVE");
    if (d && d[0] == '0') return false;
    if (!s.derive_started) s.derive_started = now();
    return s.derived.size() < 200 + s.edges && now() - s.derive_started < 3000000000LL;
}
inline bool edge_ok() {
    auto &s = st();
    const char *d = std::getenv("FAST_ANYTHING_RECORD_DERIVE");
    if (d && d[0] == '0') return false;
    if (!s.derive_started) s.derive_started = now();
    return s.edges < 100 && now() - s.derive_started < 1500000000LL;
}
inline long long edge_left() { return 1500000000LL - (now() - st().derive_started); }

// Edge cases: inputs where ports and originals tend to part ways, of the same type as the recorded value.
template <class T> typename std::enable_if<!std::is_arithmetic<T>::value, std::vector<T>>::type edges(const T &) { return {}; }
template <class T> typename std::enable_if<std::is_arithmetic<T>::value, std::vector<T>>::type edges(const T &v) {
    std::vector<T> out;
    if (std::is_same<T, bool>::value) return out;
    std::vector<double> xs = {0, 1};
    if (std::is_signed<T>::value) xs.push_back(-1);
    if (std::is_floating_point<T>::value) xs.push_back(sizeof(T) >= 8 ? 1e300 : 1e30);
    for (double x : xs)
        if ((T) x != v) out.push_back((T) x);
    return out;
}
inline std::vector<std::string> edges(const std::string &v) {
    size_t mid = 0, chars = 0;
    for (size_t i = 0; i < v.size(); i++) chars += ((unsigned char) v[i] & 0xC0) != 0x80;
    for (size_t i = 0, c = 0; i <= v.size(); i++) {
        if (i == v.size() || ((unsigned char) v[i] & 0xC0) != 0x80) {
            if (c == chars / 2) { mid = i; break; }
            c++;
        }
    }
    // an accent, CJK, an emoji with a modifier, a combining mark (UTF-8 bytes)
    const std::string a = "\xC3\xA9", cjk = "\xE6\x97\xA5\xE6\x9C\xAC", emoji = "\xF0\x9F\x91\x8D\xF0\x9F\x8F\xBD", comb = "e\xCC\x81";
    return {v.substr(0, mid) + a + v.substr(mid), v + cjk, emoji + v, v + comb, ""};
}
template <class T, class Al> std::vector<std::vector<T, Al>> edges(const std::vector<T, Al> &v) {
    std::vector<std::vector<T, Al>> out;
    if (v.empty()) return out;
    out.emplace_back();
    auto es = edges(v[0]);
    for (size_t k = 0; k < es.size() && k < 4; k++) {
        out.push_back(v);
        out.back()[0] = es[k];
    }
    return out;
}
// A short version of a recorded value, so edge cases stay cheap when the real inputs are big.
template <class T> T small(const T &v) { return v; }
inline std::string small(const std::string &v) {
    size_t i = 0, chars = 0;
    for (; i < v.size(); i++)
        if (((unsigned char) v[i] & 0xC0) != 0x80 && chars++ == 64) break;
    return v.substr(0, i);
}
template <class T, class Al> std::vector<T, Al> small(const std::vector<T, Al> &v) {
    return v.size() > 8 ? std::vector<T, Al>(v.begin(), v.begin() + 8) : v;
}
template <class Tup, size_t... I> Tup small_tuple(const Tup &t, std::index_sequence<I...>) { return Tup(small(std::get<I>(t))...); }

template <class T> bool bigger(const T &, int, T &) { return false; }
inline bool bigger(const std::string &v, int factor, std::string &out) {
    if (v.empty() || v.size() * (size_t) factor > 4096 * 4) return false;
    out.clear();
    for (int k = 0; k < factor; k++) out += v;
    return true;
}
template <class T, class Al> bool bigger(const std::vector<T, Al> &v, int factor, std::vector<T, Al> &out) {
    if (v.empty() || v.size() * (size_t) factor > 4096) return false;
    out.clear();
    for (int k = 0; k < factor; k++) out.insert(out.end(), v.begin(), v.end());
    return true;
}

static const size_t SLICES[] = {0, 1, 2, 3, 5, 8, 13, 21, 34, 55};
template <class T> std::vector<T> variants(const T &) { return {}; }
inline std::vector<std::string> variants(const std::string &v) {
    std::vector<size_t> starts;
    for (size_t i = 0; i < v.size(); i++)
        if (((unsigned char) v[i] & 0xC0) != 0x80) starts.push_back(i);
    std::vector<std::string> out;
    size_t n = starts.size();
    starts.push_back(v.size());
    if (n < 2) return out;
    for (size_t len : SLICES) {
        if (len >= n) break;
        for (int rep = 0; rep < (len < 2 ? 1 : 3); rep++) {
            size_t off = (size_t) (next() % (n - len + 1));
            out.push_back(v.substr(starts[off], starts[off + len] - starts[off]));
        }
    }
    return out;
}
template <class T, class Al> std::vector<std::vector<T, Al>> variants(const std::vector<T, Al> &v) {
    std::vector<std::vector<T, Al>> out;
    size_t n = v.size();
    if (n < 2) return out;
    for (size_t len : SLICES) {
        if (len >= n) break;
        for (int rep = 0; rep < (len < 2 ? 1 : 3); rep++) {
            size_t off = (size_t) (next() % (n - len + 1));
            out.emplace_back(v.begin() + off, v.begin() + off + len);
        }
    }
    return out;
}

template <class F, class Tup, size_t... I> auto apply(F &f, Tup &t, std::index_sequence<I...>) -> decltype(f(std::get<I>(t)...)) {
    return f(std::get<I>(t)...);
}
template <class Tup, size_t... I> bool encode_tuple(std::string &o, const Tup &t, std::index_sequence<I...>) { return enc_all(o, std::get<I>(t)...); }
template <size_t I, class F, class Tup, class Seq> void derive_one(F &f, const Tup &args, Seq seq) {
    for (auto &smaller : variants(std::get<I>(args))) {
        if (!derive_ok()) return;
        Tup next_args = args;
        std::get<I>(next_args) = smaller;
        std::string in = "[";
        if (!encode_tuple(in, next_args, seq)) continue;
        in += "]";
        try {
            auto r = apply(f, next_args, seq);
            std::string out;
            if (enc(out, r)) st().derived.push_back("{\"args\":" + in + ",\"result\":" + out + ",\"derived\":true}");
        } catch (...) {
            // outside the function's domain: not a fair test
        }
    }
}
template <class F, class Tup, class Seq> long long edge_try(F &f, const Tup &next_args, Seq seq) {
    std::string in = "[";
    if (!encode_tuple(in, next_args, seq)) return -1;
    in += "]";
    auto &s = st();
    if (s.seen[in]) return -1;
    s.seen[in] = true;
    try {
        Tup copy = next_args;
        long long t = now();
        auto r = apply(f, copy, seq);
        long long took = now() - t;
        std::string out;
        if (!enc(out, r)) return -1;
        s.derived.push_back("{\"args\":" + in + ",\"result\":" + out + ",\"derived\":\"edge\"}");
        s.edges++;
        return took;
    } catch (...) {
        return -1;  // outside the function's domain: not a fair test
    }
}
template <size_t I, class F, class Tup, class Seq> void edge_one(F &f, const Tup &args, Seq seq) {
    const Tup base = small_tuple(args, seq);
    for (auto &e : edges(std::get<I>(base))) {
        if (!edge_ok()) return;
        Tup next_args = base;
        std::get<I>(next_args) = e;
        edge_try(f, next_args, seq);
    }
    auto &s = st();
    typename std::decay<decltype(std::get<I>(args))>::type big;
    if (s.grown[I] || !bigger(std::get<I>(base), 4, big)) return;
    // much bigger inputs, only while their time (assumed quadratic) fits the budget
    s.grown[I] = true;
    long long took;
    try {
        Tup copy = base;
        long long t = now();
        apply(f, copy, seq);
        took = now() - t;
    } catch (...) {
        return;
    }
    // 4x bigger each step, from the short version of the input, up to 4096 items
    for (int factor : {4, 16, 64, 256, 512}) {
        if (!edge_ok() || !bigger(std::get<I>(base), factor, big) || took * 16 > edge_left() / 2) return;
        Tup next_args = base;
        std::get<I>(next_args) = big;
        took = edge_try(f, next_args, seq);
        if (took < 0) return;
    }
}
template <class F, class Tup, size_t... I> void derive_all(F &f, const Tup &args, std::index_sequence<I...> seq) {
    if (st().edge_calls++ < 20) {
        int edge_dummy[] = {0, (edge_one<I>(f, args, seq), 0)...};
        (void) edge_dummy;
    }
    int dummy[] = {0, (derive_one<I>(f, args, seq), 0)...};
    (void) dummy;
}

template <class F, class... A> auto call(F f, const A &...a) -> decltype(f(a...)) {
    auto &s = st();
    registered();
    s.inside = true;
    struct Reset {
        ~Reset() { st().inside = false; }
    } reset;
    if (!s.limit) {
        const char *l = std::getenv("FAST_ANYTHING_RECORD_LIMIT");
        s.limit = l && std::atoi(l) > 0 ? (size_t) std::atoi(l) : 500;
    }
    s.calls++;
    size_t slot = s.kept.size() < s.limit ? s.kept.size() : (size_t) (next() % (unsigned long long) s.calls);
    std::string in = "[";
    bool ok = slot < s.limit && enc_all(in, a...);
    in += "]";
    long long t = now();
    auto r = f(a...);
    s.nanos += now() - t;
    if (slot < s.limit) {
        std::string out;
        if (ok && enc(out, r)) {
            std::string line = "{\"args\":" + in + ",\"result\":" + out + "}";
            if (slot < s.kept.size()) s.kept[slot] = line;
            else s.kept.push_back(line);
            if (derive_ok()) {
                std::tuple<typename std::decay<A>::type...> args(a...);
                derive_all(f, args, std::index_sequence_for<A...>{});
            }
        } else {
            s.skipped++;
        }
    }
    return r;
}
} // namespace fa_rec
#endif
#endif
