# frozen_string_literal: true

# Exercism exercise
# Check if a sentence is a pangram
module Pangram
  def self.pangram?(sentence)
    downcase_sentence = sentence.downcase

    ("a".."z").each do |letter|
      return false unless downcase_sentence.include? letter
    end

    true
  end
end
