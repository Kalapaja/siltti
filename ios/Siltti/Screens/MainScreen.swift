import SwiftUI
import AVFoundation

@MainActor
final class MainModel: ObservableObject {
    
    @Published var authorized: Bool?
    @Published var status = ""
    @Published var scanned = ""
    @Published var frames: Frames?
    @Published var transmit: Action?
    
    @EnvironmentObject var router: Router
    
    let dbPath: String
    let signer: Signer
    let collection = Collection()
    
    init(dbPath: String, signer: Signer) {
        self.dbPath = dbPath
        self.signer = signer
    }

    func authorize() async {
        authorized = await AVCaptureDevice.requestAccess(for: .video)
    }
    
    func clear() {
        status = ""
        transmit = nil
        do {
            try collection.clean()
            refreshFrames()
        } catch {
            status = error.localizedDescription
        }
    }
    
    
    var packets: [String] = []
    
    func scanned(data: Data, router: Router) {
        guard router.path.isEmpty else { return }
        do {
            let result = try collection.processFrame(rawFrame: data)
            if let payload = result.payload {
                let action = try Action.newPayload(payload: payload, dbPath: dbPath, signatureMaker: signer)
                router.push(.transmit(action))
                stack(action: action)
                self.clear()
            } else {
                refreshFrames()
            }
        } catch {
            status = error.localizedDescription
        }
    }
    
    func stack(action: Action) {
        if let packet = action.makePacket()?.hex() {
            if !packets.contains(packet) {
                packets.append(packet)
                self.stack(action: action)
            }
        }
    }
    
    
    func chunk(data: Data, size: Int = 254) -> [Data] {
        var chunks: [Data] = []
        data.withUnsafeBytes { bytes in
            let pointer = UnsafeMutableRawPointer(mutating: bytes)
            let total = data.endIndex
            var offset = 0
            
            while offset < total {
                let chunkSize = offset + size > total ? total - offset : size
                let chunk = Data(bytesNoCopy: pointer, count: chunkSize, deallocator: .none)
                offset += chunkSize
                chunks.append(chunk)
            }
        }
        return chunks
    }
    
    func refreshFrames() {
        do {
            self.frames = try collection.frames()
        } catch {
            status = error.localizedDescription
        }
    }
    
    func sendBlankPayload(router: Router) {
        do {
            let action = try Action.newKampelaStop(signatureMaker: signer)
            router.push(Destination.transmit(action))
        } catch {
            status = error.localizedDescription
        }
    }
}

struct MainScreen: View {
    @StateObject var model: MainModel
    @EnvironmentObject var router: Router
    
    var body: some View {
        VStack {
            Group {
                cameraView
                    .ignoresSafeArea(.all)
                footerView
                    .padding(.horizontal)
            }.frame(maxHeight: .infinity)
        }
        .task {
            await model.authorize()
        }
    }
    
    @ViewBuilder
    var cameraView: some View {
        if model.authorized == true {
            ScannerView(types: [.qr]) { result in
                model.scanned = ""
                switch result {
                case .data(let data):
                    model.scanned(data: data, router: router)
                case .string(let string):
                    model.scanned = string
                }
            }
        } else {
            Button("Open settings and allow camera permission") {
                if let url = URL(string: UIApplication.openSettingsURLString) {
                    UIApplication.shared.open(url)
                }
            }
        }
    }
    
    var footerView: some View {
        VStack {
            if let frames = model.frames {
                Text("\(frames.current) / \(frames.total)").frame(maxWidth: .infinity)
                ProgressView(value: CGFloat(frames.current), total: CGFloat(frames.total))
            }
            
            Text(model.status).foregroundStyle(.red)
            
            if model.frames != nil || model.transmit != nil || !model.status.isEmpty {
                Button("Clear", role: .destructive) { model.clear() }
            }
            if !model.scanned.isEmpty {
                Text(model.scanned).frame(maxWidth: .infinity, alignment: .leading)
            }

//            ScrollView {
//                Text(model.scanned)
//                    .padding()
//                    .frame(maxWidth: .infinity, alignment: .leading)
//            }
//            .border(Color.black)
            
            Spacer()
            Group {
                NavigationLink(value: Destination.newAddress) {
                    Text("New address").frame(minWidth: 200)
                }
                NavigationLink(value: Destination.networks) {
                    Text("Manage Networks").frame(minWidth: 200)
                }
                Button(action: { model.sendBlankPayload(router: router)}) {
                    Text("Send blank payloads").frame(minWidth: 200)
                }
            }
        }
        .buttonStyle(.bordered)
    }
}

#Preview {
    MainScreen(model: .init(dbPath: NSTemporaryDirectory(), signer: Signer(tag: "privew")))
}

extension Data {
    var latin: String? {
        String(data: self, encoding: .isoLatin1)
    }
}

