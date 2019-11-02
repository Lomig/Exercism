module Acronym
  def self.abbreviate string
    words = string.split(/\W+/)

    words.map { |word| word.chr }.join.upcase
  end
end
