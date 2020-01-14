# frozen_string_literal: true

# Class Phrase holds a phrase
# Can count words within this phrase
class Phrase
  private

  attr_reader :phrase

  public

  def initialize(phrase)
    @phrase = phrase
  end

  def word_count
    regex = /\b([[:alnum:]]+(\'[[:alpha:]]+)?)\b/
    phrase.scan(regex)
          .each_with_object(Hash.new(0)) { |word, h| h[word[0].downcase] += 1 }
  end
end
