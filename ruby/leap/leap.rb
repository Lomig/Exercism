# frozen_string_literal: true

# Check if a year is a leap year
# Exercism challenger
module Year
  def self.leap?(year)
    return false unless (year % 4).zero?
    return true if (year % 400).zero?
    return false if (year % 100).zero?

    true
  end
end
