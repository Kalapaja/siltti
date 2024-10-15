import SwiftUI

typealias TransmitData = Action

struct TransmitScreen: View {
    @StateObject var nfc: NFC
    @Environment(\.dismiss) var dismiss
    
    init(data: TransmitData) {
        _nfc = .init(wrappedValue: NFC(data: data))
    }
    
    var body: some View {
        VStack {
            ScrollView {
                if let error = nfc.error {
                    Text(error).foregroundStyle(.red)
                } else {
                    Text(nfc.status).foregroundStyle(.green)
                }
                
                Text(nfc.log.reversed().joined(separator: "\n")).padding(.top)
            }
            VStack {
                
                Text("Polling timer: \(String(describing: Int(nfc.timer))) seconds")
                Slider(value: $nfc.timer, in: 0...100)
                
                Toggle("Autorecconect session", isOn: $nfc.reconnect)
            }
            .padding()
            .padding(.horizontal)
            
            HStack {
                Group {
                    Button("Status") {
                        nfc.start(process: .status)
                    }
                    Button("Read") {
                        nfc.start(process: .read)
                    }
                    Button("Write") {
                        nfc.start(process: .write)
                    }
                }.frame(maxWidth: .infinity)
            }
            .buttonStyle(.bordered)
        }
        .navigationTitle("Transmit")
        .onDisappear {
            nfc.stop()
        }
    }
}
