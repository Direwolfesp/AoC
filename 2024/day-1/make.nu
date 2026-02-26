#!/usr/bin/env nu

def log_cmd [cmd: string] {
  print $cmd
  run-external ($cmd | split row ' ')
}

def "main build" [] {
  glob *.{zig,cpp} --no-dir | par-each {|f|
    if ($f | path basename) =~ ".zig" {
      log_cmd $"zig build-exe -O ReleaseFast ($f)"
    } else if ($f | path basename) =~ ".cpp" {
      log_cmd $"g++ ($f) -O3 -o ($f | path parse | get stem)"
    }
  }
  | ignore
}

def "main clean" [] {
  glob * --no-dir | where (file $it) =~ "ELF 64-bit" | rm ...$in
}

def main [] {}
