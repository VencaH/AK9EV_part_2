def main [x: int] {
    generate {|i| if $i < $x {{out: $i, next: ($i + 1) }}} 0 | reduce --fold [] { |_, acc| ./target/release/evol_heuristics 2 | parse "({χ},{h0_rejected})" | append $acc}
    generate {|i| if $i < $x {{out: $i, next: ($i + 1) }}} 0 | reduce --fold [] { |_, acc| ./target/release/evol_heuristics 10 | parse "({χ},{h0_rejected})" | append $acc}
    generate {|i| if $i < $x {{out: $i, next: ($i + 1) }}} 0 | reduce --fold [] { |_, acc| ./target/release/evol_heuristics 20 | parse "({χ},{h0_rejected})" | append $acc}
}
