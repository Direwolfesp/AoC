#!/bin/env nu

# Advent of Code 2024 Day 1 Part A
def main [file: path] {
    open $file 
    | parse "{left}   {right}"
    | into int left right 
    | let nums
    | ($nums.left | sort)
    | zip ($nums.right | sort)
    | flatten
    | chunks 2
    | each { $in.0 - $in.1 | math abs }
    | math sum
}
