# frozen_string_literal: true

# Class Matrix - Simple representation of a Matrix
class Matrix
  attr_reader :rows

  def initialize(matrix_as_string)
    @rows = string_matrix_to_rows(matrix_as_string)
  end

  def columns
    (0..rows.first.count).inject([]) do |cols, i|
      cols << rows.map { |row| row[i] }
    end
  end

  private

  def string_matrix_to_rows(matrix_as_string)
    matrix_as_string
      .split("\n")
      .map { |x| x.split(" ").map(&:to_i) }
  end
end
