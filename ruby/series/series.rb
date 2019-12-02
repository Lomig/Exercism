# frozen_string_literal: true

# Series - Create slices of n elements from a series
class Series
  private

  attr_reader :series

  public

  def initialize(string)
    @series = string.split("")
  end

  def slices(num)
    raise ArgumentError if num > series.count

    (0..series.count)
      .each_with_object([]) { |index, result| result << series[index, num] }
      .select { |x| x.count == num }
      .map(&:join)
  end
end
