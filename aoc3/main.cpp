#include <algorithm>
#include <iostream>
#include <fstream>
#include <vector>
#include <filesystem>
#include <cassert>
#include <optional>
#include <cctype>
#include <charconv>
#include "fmt/format.h"
#include "fmt/ranges.h"

namespace fs = std::filesystem;

struct point {
  int x{};
  int y{};
};
struct part_number {
  point position;
  std::string_view digits;
  std::optional<int> value;

  [[nodiscard]] bool adjacent(point p) const noexcept {
    return position.x - 1 <= p.x && p.x <= (position.x + static_cast<int>(digits.size()))
        && position.y - 1 <= p.y && p.y <= (position.y + 1);
  }
};

template<>
struct fmt::formatter<point> {
  constexpr auto parse(format_parse_context& ctx) -> format_parse_context::iterator { return ctx.end(); }
  auto format(const point& p, format_context& ctx) const -> format_context::iterator {
    return fmt::format_to(ctx.out(), "({}, {})", p.x, p.y);
  }
};

template<>
struct fmt::formatter<part_number> {
  constexpr auto parse(format_parse_context& ctx) -> format_parse_context::iterator { return ctx.end(); }
  auto format(const part_number& p, format_context& ctx) const -> format_context::iterator {
    return fmt::format_to(ctx.out(), "part{}:'{}'/{}", p.position, p.digits, p.value.value());
  }
};


int main()
{
  const fs::path in_file_name{"in.txt"};
  const auto file_size = fs::file_size(in_file_name);
  const auto file_contents = [=] {
    std::vector<char> buf;
    buf.resize(file_size);
    std::ifstream ifs{in_file_name};
    ifs.read(buf.data(), static_cast<std::streamsize>(file_size));
    assert(ifs);
    return buf;
  }();
  const auto line_length = std::distance(std::begin(file_contents), std::find(std::begin(file_contents), std::end(file_contents), char{'\n'}));
  const auto lines = file_size / (line_length+1);
  std::cout << "lines: " << lines << "\n";
  const auto at = [&](int x, int y) -> const char* {
    if (x < 0 || y < 0) return nullptr;
    if (x >= line_length) return nullptr;
    if (y >= lines) return nullptr;
    return &file_contents.at(y * (line_length+1) + x);
  };
  const auto isdigit = [](char c) { return std::isdigit(c); };
  const auto issymbol = [&](char c) { return !isdigit(c) && c != '.' && c != '\n' && c != '\0'; };

  std::vector<part_number> part_numbers;
  for(int y = 0; y < lines; ++y) {
    int start = -1;
    for(int x = 0; x < line_length; ++x) {
      const char c = *at(x, y);
      if (isdigit(c) && start == -1) start = x;
      if (!isdigit(c) && start != -1) {
        part_numbers.push_back({
          .position = {start, y},
          .digits = std::string_view{at(start, y), static_cast<size_t>(x - start)}
        });
        start = -1;
      }
      if (isdigit(c) && start != -1 && x == (line_length - 1)) {
        part_numbers.push_back({
            .position = {start, y},
            .digits = std::string_view{at(start, y), static_cast<size_t>(x + 1 - start)}
        });
      }
    }
  }
  for(auto& pn: part_numbers) {
    int num{};
    const auto [ptr, ec] = std::from_chars(pn.digits.data(), pn.digits.data() + pn.digits.size(), num);
    assert(ec == std::errc{});
    pn.value = num;
  }
  std::cout << fmt::format("{}\n", part_numbers);
  const auto symbol_around = [&](part_number pn) {
    for(int y = pn.position.y - 1; y <= (pn.position.y + 1); ++y) {
      for(int x = pn.position.x - 1; x <= (pn.position.x + static_cast<int>(pn.digits.size())); ++x) {
        assert(pn.adjacent(point{x, y}));
        const char* pc = at(x, y);
        //std::cout << fmt::format("{}  {}  {}\n", point{x, y}, pn, pc ? *pc : '.');
        if (!pc) continue;
        if (issymbol(*pc)) return true;
      }
    }
    return false;
  };
  int sum = 0;
  for(const auto& pn: part_numbers) {
    std::cout << fmt::format("{}  ", pn);
    if (symbol_around(pn)) {
      std::cout << "+++\n";
      sum += *pn.value;
    } else {
      std::cout << "   \n";
    }
  }
  std::cout << fmt::format("sum value: {}\n", sum);
  int gear_ratio_sum = 0;
  for(int y = 0; y < lines; ++y) {
    for (int x = 0; x < line_length; ++x) {
      const char* pc = at(x, y);
      if (!pc || *pc != '*') continue;
      std::vector<part_number> adjacent_parts;
      std::copy_if(std::begin(part_numbers), std::end(part_numbers), std::back_inserter(adjacent_parts), [&](const part_number& pn) {
        return pn.adjacent(point{x, y}); });
      if (adjacent_parts.size() != 2) continue;
      const auto ratio = *adjacent_parts[0].value * *adjacent_parts[1].value;
      std::cout << fmt::format("gear: {}  ratio: {}\n", adjacent_parts, ratio);
      gear_ratio_sum += ratio;
    }
  }
  std::cout << fmt::format("sum of gear ratios: {}\n", gear_ratio_sum);
  return 0;
}
