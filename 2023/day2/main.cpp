#include <array>
#include <algorithm>
#include <iostream>
#include <vector>
#include <string>
#include <sstream>
#include <exception>
#include <fstream>
#include <cctype>
#include <charconv>
#include <cinttypes>
#include "fmt/format.h"
#include "fmt/ranges.h"
#include "range/v3/view.hpp"
#include "range/v3/numeric.hpp"
#include "range/v3/algorithm.hpp"

using inttype = int64_t;

struct cubeset {
  inttype red = 0;
  inttype green = 0;
  inttype blue = 0;

  [[nodiscard]] inttype power() const noexcept { return red * green * blue; }
};

struct game {
  int id = 0;
  std::vector<cubeset> showings;

  [[nodiscard]] bool possible(cubeset cubes) const noexcept {
    return std::all_of(std::begin(showings), std::end(showings), [cubes](cubeset showing) {
      return showing.red <= cubes.red && showing.green <= cubes.green && showing.blue <= cubes.blue;
    });
  }

  cubeset fewest_of_each() const noexcept {
    const auto max_red = ranges::max(showings | ranges::view::transform(&cubeset::red));
    const auto max_green = ranges::max(showings | ranges::view::transform(&cubeset::green));
    const auto max_blue = ranges::max(showings | ranges::view::transform(&cubeset::blue));
    return cubeset{
      .red = max_red,
      .green = max_green,
      .blue = max_blue
    };
  }
};


struct error : std::exception {};
const auto assrt = [](bool cond) { if (!cond) { throw error{}; } };

cubeset parse_showing(std::string_view showing_str) {
  cubeset result;
  auto elements = showing_str
      | ranges::view::split(',')
      | ranges::view::transform([](auto&& range) { return std::string_view{&*std::begin(range), ranges::distance(range)}; })
      | ranges::view::transform([](std::string_view sv) {
        auto trimmed = sv | ranges::view::trim([](char c) { return std::isspace(c); });
        return std::string_view{std::begin(trimmed), ranges::distance(trimmed)};
      })
      | ranges::view::transform([](std::string_view sv) {
        return sv
            | ranges::view::split(' ')
            | ranges::view::transform([](auto&& range) { return std::string_view{&*std::begin(range), ranges::distance(range)}; })
            | ranges::to<std::vector>()
            ;
      })
  ;
  for (const auto& element: elements) {
    const auto num_str = element[0];
    const auto color_str = element[1];
    inttype num = 0;
    const auto [ptr, ec] = std::from_chars(num_str.data(), num_str.data() + num_str.size(), num);
    assrt(ec == std::errc{});

    if (color_str == "red") result.red = num;
    else if(color_str == "green") result.green = num;
    else if(color_str == "blue") result.blue = num;
    else assrt(false);
  }
  return result;
}

game parse_game(std::string_view line) {
  std::stringstream iss{std::string{line}};
  std::string game_word, colon;
  int gameid = 0;
  iss >> game_word >> gameid >> colon;
  assrt(game_word == "Game");
  assrt(colon == ":");
  std::vector<std::string> showing_strings;
  std::string showing_str;
  while(std::getline(iss, showing_str, ';')) {
    if(showing_str.empty()) break;
    if(showing_str.starts_with(' ')) showing_str.erase(0, 1);
    showing_strings.push_back(std::move(showing_str));
  }
  return game{
    .id = gameid,
    .showings = showing_strings | ranges::view::transform(parse_showing) | ranges::to<std::vector>()
  };
}

template<>
struct fmt::formatter<cubeset> {
  constexpr auto parse(format_parse_context& ctx) -> format_parse_context::iterator { return ctx.end(); }
  auto format(const cubeset& c, format_context& ctx) const -> format_context::iterator {
    return fmt::format_to(ctx.out(), "(red {}, green {}, blue {})", c.red, c.green, c.blue);
  }
};

template<>
struct fmt::formatter<game> {
  constexpr auto parse(format_parse_context& ctx) -> format_parse_context::iterator { return ctx.end(); }
  auto format(const game& g, format_context& ctx) const -> format_context::iterator {
    return fmt::format_to(ctx.out(), "Game {}: {}", g.id, g.showings);
  }
};

int main()
{
  std::ifstream ifs{"in.txt"};
  std::string line;
  std::vector<game> games;
  while(std::getline(ifs, line)) {
    game g = parse_game(line);
    std::cout << fmt::format("game: {}\n", g);
    games.push_back(std::move(g));
  }

  cubeset test{.red=12, .green=13, .blue=14};

  auto possible_games = games | ranges::view::filter([&test](const game& g) { return g.possible(test); });
  std::cout << "\npossible games:\n";
  for(const auto& g: possible_games) {
    std::cout << fmt::format("  {}\n", g);
  }

  auto ids = possible_games | ranges::view::transform([](const game& g) { return g.id; });
  auto sum_of_ids = ranges::accumulate(ids, 0);
  std::cout << fmt::format("\nsum of ids: {}\n\n", sum_of_ids);

  inttype pow_sum = 0;
  for(const auto& g: games) {
    const auto fewest = g.fewest_of_each();
    std::cout << fmt::format("  game {} fewest possible of each: {}  power: {}\n", g.id, fewest, fewest.power());
    pow_sum += fewest.power();
  }
  std::cout << "sum of powers: " << pow_sum << '\n';
  return 0;
}
