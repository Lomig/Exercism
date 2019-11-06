# frozen_string_literal: true

# Check the type of triangle
# Length provided in an array
class Triangle
  private

  attr_reader :lengths

  public

  def initialize(lengths)
    @lengths = lengths
  end

  def equilateral?
    valid? && lengths.uniq.count == 1
  end

  def isosceles?
    valid? && lengths.uniq.count <= 2
  end

  def scalene?
    valid? && !isosceles?
  end

  def degenerate?
    small_lengths = lengths.min(2)

    valid? && small_lengths.first + small_lengths.last = lengths.max
  end

  private

  def valid?
    small_lengths = lengths.min(2)

    lengths.min.positive? &&
      small_lengths.first + small_lengths.last >= lengths.max
  end
end
