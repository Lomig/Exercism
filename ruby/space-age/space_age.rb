# frozen_string_literal: true

# Exercism exercise
# Give the age of someone in planet-years
class SpaceAge
  REVOLUTION_PER_YEAR = {
    earth: 1,
    mercury: 0.2408467,
    venus: 0.61519726,
    mars: 1.8808158,
    jupiter: 11.862615,
    saturn: 29.447498,
    uranus: 84.016846,
    neptune: 164.79132,
  }.freeze

  def initialize(age_in_seconds)
    @age_on_earth = (age_in_seconds / 31_557_600.0)
  end

  REVOLUTION_PER_YEAR.each do |planet, revolution_period|
    define_method(["on_", planet].join) do
      (@age_on_earth / revolution_period).round(2)
    end
  end
end
