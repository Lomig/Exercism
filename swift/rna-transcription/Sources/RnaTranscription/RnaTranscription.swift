struct Nucleotide {
    static let transcriptionTable: [Character: Character] = [
        "G": "C",
        "C": "G",
        "T": "A",
        "A": "U"
    ]
    
    let nucleotides: String
    
    init(_ nucleotides: String) {
        self.nucleotides = nucleotides
    }
    
    func complementOfDNA() throws -> String {
        let transcription = try nucleotides.reduce("") { result, nucleotide in
            guard let transcript = Nucleotide.transcriptionTable[nucleotide] else {
                throw RnaTranscription.TranscriptionError.invalidNucleotide(reason: "\(nucleotide) is not a valid Nucleotide")
            }
            return "\(result)\(transcript)"
        }
        return transcription
    }
}

struct RnaTranscription {
    enum TranscriptionError: Error {
        case invalidNucleotide(reason: String)
    }
}
