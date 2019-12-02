# frozen_string_literal: true

# Class Matrix - Simple representation of a Matrix
class Matrix
  attr_reader :rows

  def initialize(matrix_as_string)
    @rows = string_matrix_to_rows(matrix_as_string)
  end

  def columns
    rows.transpose
  end

  private

  def string_matrix_to_rows(matrix_as_string)
    matrix_as_string
      .lines
      .map { |x| x.split.map(&:to_i) }
  end
end
