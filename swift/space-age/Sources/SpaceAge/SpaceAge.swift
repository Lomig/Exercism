class SpaceAge {
    let seconds: Int
    var onEarth = 0.0, onMercury = 0.0, onVenus = 0.0, onMars = 0.0
    var onJupiter = 0.0, onSaturn = 0.0, onUranus = 0.0, onNeptune = 0.0
    
    private let age_on_earth: Double
    private static let ORBITAL_PERIODS = [ "Earth": 1.0,
                                           "Mercury": 0.2408467,
                                           "Venus": 0.61519726,
                                           "Mars": 1.8808158,
                                           "Jupiter": 11.862615,
                                           "Saturn": 29.447498,
                                           "Uranus": 84.016846,
                                           "Neptune": 164.79132 ]
    
    init(_ age: Int) {
        seconds = age
        age_on_earth = Double(age) / 31_557_600.0
        
        onEarth = compute_age("Earth")
        onMercury = compute_age("Mercury")
        onVenus = compute_age("Venus")
        onMars = compute_age("Mars")
        onJupiter = compute_age("Jupiter")
        onSaturn = compute_age("Saturn")
        onUranus = compute_age("Uranus")
        onNeptune = compute_age("Neptune")
    }
    
    private func compute_age(_ planet:String) -> Double {
        return (age_on_earth / (SpaceAge.ORBITAL_PERIODS[planet] ?? 1) * 100).rounded() / 100
    }
}
