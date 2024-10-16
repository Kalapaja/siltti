import SwiftUI

enum Destination: Hashable, Identifiable {
    case newAddress
    case networks
    case transmit(Action)
    
    var id: String { "\(self)"}
    
    static func ==(lhs: Self, rhs: Self) -> Bool {
        lhs.id == rhs.id
 
    }
    func hash(into hasher: inout Hasher) {
        hasher.combine(id)
    }
}

final class Router: ObservableObject {
    @Published var path: [Destination] = []
    
    func push(_ destination: Destination) {
        path.append(destination)
    }
}
