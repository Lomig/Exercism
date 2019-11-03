# frozen_string_literal: true

# Exercism Highscore exercise
# Highscore provides the highest score in the scores provided to it
class HighScores
  attr_accessor :scores

  def initialize(scores)
    @scores = scores
  end

  def latest
    @scores.last
  end

  def personal_best
    @scores.max
  end

  def personal_top_three
    @scores.sort.reverse.slice(0, 3)
  end

  def latest_is_personal_best?
    latest == personal_best
  end
end
