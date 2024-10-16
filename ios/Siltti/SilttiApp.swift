import SwiftUI

@main
struct SilttiApp: App {
    
    @StateObject var router = Router()
    let dbPath = NSTemporaryDirectory()
    let signer = Signer(tag: "AndroidKeyStore")
    
    var body: some Scene {
        WindowGroup {
            NavigationStack(path: $router.path) {
                MainScreen(model: .init(dbPath: dbPath, signer: signer))
                    .navigationDestination(for: Destination.self) { destination in
                        Group {
                            switch destination {
                            case .networks: NetworksScreen(dbPath: dbPath)
                            case .newAddress: NewAddressScreen(signer: signer)
                            case .transmit(let action): TransmitScreen(data: action)
                            }
                        }
                        .environmentObject(router)
                    }
                    .environmentObject(router)
            }
        }
    }
}
