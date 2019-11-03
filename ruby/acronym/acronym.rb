# frozen_string_literal: true

# Exercism exercise
# Create an acronym from any phrase
module Acronym
  def self.abbreviate(phrase)
    phrase.scan(/\b\w/).join.upcase
  end
end
