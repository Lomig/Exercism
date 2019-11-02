module ResistorColorDuo

  COLORS = %w(black brown red orange yellow green blue violet grey white)

  def self.color_code color
    COLORS.index color
  end

  def self.value colors
    # !We only decode the 2 first strips of color!
    10 * self.color_code(colors[0]) + self.color_code(colors[1])
  end
end
