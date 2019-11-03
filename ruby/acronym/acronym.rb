# frozen_string_literal: true

# Exercism exercise
# Create an acronym from any string
module Acronym
  def self.abbreviate(string)
    string.scan(/\b\w/).join.upcase
  end
end
