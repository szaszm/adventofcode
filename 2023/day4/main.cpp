#include <iostream>
#include <fstream>
#include <sstream>
#include <vector>
#include <string_view>
#include <string>
#include <cctype>
#include <charconv>
#include "range/v3/all.hpp"
#include "fmt/format.h"
#include "fmt/ranges.h"

struct scratchcard {
  int id{};
  std::vector<int> winning;
  std::vector<int> own;

  [[nodiscard]] auto win_count() const {
    return std::count_if(std::begin(own), std::end(own), [this](int i) {
      return std::find(std::begin(winning), std::end(winning), i) != std::end(winning); });
  }

  [[nodiscard]] int score() const {
    return static_cast<int>(std::pow(2, win_count() - 1));
  }
};

template<>
struct fmt::formatter<scratchcard> {
  constexpr auto parse(format_parse_context& ctx) -> format_parse_context::iterator { return ctx.end(); }
  auto format(const scratchcard& sc, format_context& ctx) const -> format_context::iterator {
    return fmt::format_to(ctx.out(), "(#{} win:{} own:{})", sc.id, sc.winning, sc.own);
  }
};


int main()
{
  constexpr auto isspace = [](char c) { return std::isspace(c); };
  constexpr auto to_string_view = [](auto&& rng) { return std::string_view(&*rng.begin(), ranges::distance(rng)); };
  constexpr auto nonempty = [](auto&& rng) { return !rng.empty(); };
  constexpr auto parse_int = [](auto&& string) {
    int num{};
    const auto [ptr, ec] = std::from_chars(string.data(), string.data() + string.size(), num);
    assert(ec == std::errc{});
    return num;
  };

  const auto cards = [&] {
    std::ifstream ifs{"in.txt"};
    std::vector<scratchcard> res;
    for (std::string line; std::getline( ifs, line );) {
      //std::cout << line << '\n';
      std::stringstream ss{line};
      std::string ignore_word;
      int card_id{};
      ss >> ignore_word >> card_id >> ignore_word;
      ss.ignore( 1 );  // space
      std::string rest;
      std::getline( ss, rest );
      //std::cout << '\'' << ignore_word << "' " << card_id << '|' << rest << '\n';
      auto halfs = rest | ranges::view::split( '|' ) | ranges::view::transform( to_string_view ) | ranges::to<std::vector>();
      auto winning_nums = halfs[0] | ranges::view::split( ' ' ) | ranges::view::transform( to_string_view ) | ranges::view::filter( nonempty ) | ranges::view::transform( parse_int ) | ranges::to<std::vector>();
      auto own_nums = halfs[1] | ranges::view::split( ' ' ) | ranges::view::transform( to_string_view ) | ranges::view::filter( nonempty ) | ranges::view::transform( parse_int ) | ranges::to<std::vector>();
      //std::cout << fmt::format("{}\n{}\n", winning_nums, own_nums);
      res.push_back({
          .id = card_id,
          .winning = std::move(winning_nums),
          .own = std::move(own_nums)
      });
    }
    return res;
  }();
  int sum_points = 0;
  for(const auto& c: cards) {
    const auto points = c.score();
    std::cout << fmt::format("{}\tscore:{}\n", c, points);
    sum_points += points;
  }
  std::cout << fmt::format("sum score: {}\n\n", sum_points);

  std::vector<int> copies;
  copies.resize(cards.size());
  std::fill(std::begin(copies), std::end(copies), 1);
  for(size_t i = 0; i < cards.size(); ++i) {
    assert(i < copies.size());
    const auto matching = cards[i].win_count();
    for(size_t j = i + 1; j <= i + matching; ++j) {
      assert(j < copies.size());
      copies[j] += copies[i];
    }
    std::cout << fmt::format(" card {}: {} copies\n", cards[i].id, copies[i]);
  }
  const auto sum_copies = std::accumulate(std::begin(copies), std::end(copies), 0, std::plus<>{});
  std::cout << fmt::format("\nsum copies: {}\n", sum_copies);
  return 0;
}
