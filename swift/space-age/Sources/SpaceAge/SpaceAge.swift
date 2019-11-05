struct SpaceAge {
    let seconds: Int
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
    }
    
    private func compute_age(_ planet:String) -> Double {
        return (age_on_earth / (SpaceAge.ORBITAL_PERIODS[planet] ?? 1) * 100).rounded() / 100
    }
    
    var onEarth: Double {get {return compute_age("Earth") }}
    var onMercury: Double {get {return compute_age("Mercury") }}
    var onVenus: Double {get {return compute_age("Venus") }}
    var onMars: Double {get {return compute_age("Mars") }}
    var onJupiter: Double {get {return compute_age("Jupiter") }}
    var onSaturn: Double {get {return compute_age("Saturn") }}
    var onUranus: Double {get {return compute_age("Uranus") }}
    var onNeptune: Double {get {return compute_age("Neptune") }}
}
