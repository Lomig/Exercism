module Complement

  RNA_TRANSCRIPTION = { 'G' => 'C',
                        'C' => 'G',
                        'T' => 'A',
                        'A' => 'U' }

  def self.of_dna(nucleotides)

    nucleotides.chars.map { |nucleotide| RNA_TRANSCRIPTION[nucleotide] }.join
  end
end
