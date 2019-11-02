module Year

  def self.leap? year
    return false unless year % 4 == 0
    return true if year % 400 == 0
    return false if year % 100 == 0

    true
  end
end
