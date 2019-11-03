struct SumOfMultiples {
    static func toLimit(_ limit: Int, inMultiples: [Int]) -> Int {
        let all_multiples = inMultiples.reduce([]) { result, n in
            return result + multiples(of: n, upTo: limit)
        }
        return Set(all_multiples).reduce(0, +)
    }
    
    static private func multiples( of: Int, upTo: Int) -> [Int] {
        let multiple = of
        let limit = upTo
        
        if multiple == 0 || limit <= multiple { return [0] }

        let numbers: [Int] = Array(1...(1 + limit / multiple))
        
        return numbers.reduce([]) { list_of_multiples, n in
            if n * multiple >= limit {
                return list_of_multiples
            } else {
                return list_of_multiples + [n * multiple]
            }
        }
    }
}
