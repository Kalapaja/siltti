import SwiftUI

struct NewAddressScreen: View {
    
    let signer: Signer
    @State var address = ""
    @State var hasPassword = false
    @State var error = ""
    @EnvironmentObject var router: Router

    var body: some View {
        VStack {
            TextField("New address:", text: $address)
                .textFieldStyle(.roundedBorder)
            
            //Toggle("Has password", isOn: $hasPassword)
            
            if !error.isEmpty {
                Text(error).foregroundStyle(.red)
            }
            Spacer()

            Button(action: {
                do {
                    let action = try Action.newDerivation(cutPath: address, hasPwd: hasPassword, signatureMaker: signer)
                    router.push(.transmit(action))
                } catch {
                    self.error = error.localizedDescription
                }
            }) {
                Text("Send").frame(minWidth: 200)
            }
            .buttonStyle(.bordered)
        }
        .padding()
        .navigationTitle("New Address")
        
    }
}

#Preview {
    NewAddressScreen(signer: Signer(tag: "preview")).environmentObject(Router())
}
