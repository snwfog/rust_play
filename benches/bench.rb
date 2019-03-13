require 'benchmark'

array = (0...1_000_000).to_a.shuffle

def qsort(array)
  array.sort
end

Benchmark.bmbm do |b|
  b.report("sort") { qsort(array) }
end


