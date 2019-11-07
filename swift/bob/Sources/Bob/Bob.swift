import Foundation

struct Bob {
    static func hey(_ interaction: String) -> String {
        // If the string is blank
        if interaction.range(of: #"^[[:blank:]]*$"#, options: .regularExpression) != nil {
            return "Fine. Be that way!"
        }
        
        // if the string is only numbers and signs, but with no ? at the end
        if interaction.range(of: #"^[[:blank:][:digit:][:punct:]]+[^\?]$"#, options: .regularExpression) != nil {
            return "Whatever."
        }
        
        // if the string is only numbers, signs, uppercase letters
        // with at least one upper letter
        if interaction.range(of: #"^[^[:lower:]]*[[:upper:]][^[:lower:]]*$"#, options: .regularExpression) != nil {
            return "Whoa, chill out!"
        }
        
        // if the string ends with a ?
        if interaction.range(of: #"\?$"#, options: .regularExpression) != nil {
            return "Sure."
        }
        
        return "Whatever."
    }
}
