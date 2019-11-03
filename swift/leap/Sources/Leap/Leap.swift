class Year {
    let calendarYear: Int
    
    init(calendarYear: Int) {
        self.calendarYear = calendarYear
    }

    var isLeapYear: Bool {
        if self.calendarYear % 4 != 0 { return false }
        if self.calendarYear % 400 == 0 { return true }
        if self.calendarYear % 100 == 0 { return false }
        
        return true
    }
}
