import Foundation

struct Grains {
    
    static let total = 1 + 2 * (UInt(pow(Double(2), Double(63))) - 1)

    static func square(_ square_index: Int) throws -> UInt {
        guard square_index <= 64 else {
            throw GrainsError.inputTooHigh("Input[\(square_index)] invalid. Input should be between 1 and 64 (inclusive)")
        }
        
        guard square_index > 0 else {
            throw GrainsError.inputTooLow("Input[\(square_index)] invalid. Input should be between 1 and 64 (inclusive)")
        }
        
        return UInt(pow(Double(2), Double(square_index - 1)))
    }
    
    enum GrainsError: Error {
        case inputTooHigh(_ message: String)
        case inputTooLow(_ message: String)
    }
}
