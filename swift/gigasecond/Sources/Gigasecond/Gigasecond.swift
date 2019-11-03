import Foundation

class Gigasecond {
    
    let birthDate: Date?
    let gigaDate: Date?
    
    static let dateFormatter: DateFormatter = {
        let f = DateFormatter()
        f.dateFormat = "yyyy-MM-dd'T'HH:mm:ss"
        f.timeZone = TimeZone(identifier:"GMT")
        return f
    }()
    
    init?(from: String) {
        guard let birthDate = Gigasecond.dateFormatter.date(from: from) else {return nil}
        self.birthDate = birthDate
        
        self.gigaDate = (self.birthDate!) + 1_000_000_000
    }
    
    var description: String {
        Gigasecond.dateFormatter.string(from: self.gigaDate!)
    }
}
