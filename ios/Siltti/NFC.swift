import CoreNFC

final class NFC: NSObject, ObservableObject {
    
    let data: TransmitData
    var session: NFCTagReaderSession?
    
    enum Process {
        case status
        case read
        case write
    }
    
    var process: Process?
    @Published var error: String?
    @Published var status = ""
    @Published var log: [String] = []
    @Published var reconnect = false
    @Published var timer: TimeInterval = 15
    
    var connectTask: Task<Void, Never>?

    init(data: TransmitData) {
        self.data = data
        super.init()
    }
    
    func start(process: Process) {
        self.process = nil
        connectTask?.cancel()
        session?.invalidate()
        
        self.process = process
        error = nil
        log = []
        session = .init(pollingOption: [.iso14443], delegate: self, queue: .main)
        session?.alertMessage = "You can hold you NFC-tag to the back-top of your iPhone"
        session?.begin()
    }
    
    func stop() {
        process = nil
        session?.invalidate()
    }
}

extension NFC: NFCTagReaderSessionDelegate {
    func tagReaderSessionDidBecomeActive(_ session: NFCTagReaderSession) {
        error = nil
    }
    
    func tagReaderSession(_ session: NFCTagReaderSession, didInvalidateWithError error: any Error) {
        guard let process = self.process else { return }
        if reconnect {
            // auto-reconnect session on failure
            DispatchQueue.main.async {
                let codes: [NFCReaderError.Code] = [.readerSessionInvalidationErrorSessionTimeout, .readerSessionInvalidationErrorSystemIsBusy]
                if self.session == nil {
                    print("restarting session")
                    self.start(process: process)
                } else if let nfcError = error as? NFCReaderError, codes.contains(nfcError.code) {
                    print("system is busy")
                    self.start(process: process)
                }
                self.error = error.localizedDescription
            }
        } else {
            self.error = error.localizedDescription
        }
    }
        
    func tagReaderSession(_ session: NFCTagReaderSession, didDetect tags: [NFCTag]) {
        self.error = nil
        guard let tag = tags.first else { return }
        connectTask = Task {
            async let c: Void = await connect(session: session, to: tag)
            async let t: Void = await restartPolling(session: session, sleep: timer)
            _ = await [c, t]
        }
    }
    
    @MainActor
    func restartPolling(session: NFCTagReaderSession, sleep: TimeInterval) async {
        guard sleep > 0 else { return }
        try? await Task.sleep(for: .seconds(sleep))
        self.error = "RESTART POLLING"
        session.restartPolling()
    }
    
    @MainActor
    func connect(session: NFCTagReaderSession, to tag: NFCTag) async {
        do {
            try await session.connect(to: tag)
            
            switch tag {
            case .feliCa(let felicaTag): error = "Tag is not iso7816 (actually is FeliCa: \(felicaTag)"
            case .iso15693(let isoTag): error = "Tag is not iso7816 (actually is iso15693: \(isoTag)"
            case .miFare(let mifareTag): error = "Tag is not iso7816 (actually is MiFare: \(mifareTag)"
            case .iso7816(let isoTag):
                var counter = 0
                while true {
                    switch process {
                    case .status:
                        let status = try await isoTag.queryNDEFStatus()
                        log.append("Status: \(status.0.name), Capacity: \(status.1)")
                    case .read:
                        let message = try await isoTag.readNDEF()
                        log.append("NDEFMessage: \(message)")
                    case .write:
                        if let packet = data.makePacket() {
                            let command = NFCISO7816APDU(instructionClass: 0x00, instructionCode: 0x00,
                                                         p1Parameter: 0x00, p2Parameter: 0x00,
                                                         data: packet, expectedResponseLength: -1)
                            let response = try await isoTag.sendCommand(apdu: command)
                            log.append("\(response)")
                        } else {
                            error = "Empty packet"
                        }
                    case nil:
                        error = "Unknown operation"
                        return
                    }
                    counter += 1
                    log.append("Operations count: \(counter)")
                }
            @unknown default:
                error = "Uknown tag type"
            }
            
        } catch {
            self.error = error.localizedDescription
            if timer == 0 {
                self.session = nil
                session.invalidate()
            } else {
                connectTask?.cancel()
                session.restartPolling()
            }
        }
    }
    
    func selectAids(tag: NFCISO7816Tag) async {
        let aids: [String] = [
            "A000000003",    // GlobalPlatform
            "A000000004",    // ISO 7816 Application
            "A000000025",    // Issuer Identification
            "A000000063",    // UnionPay
            "A0000002471001", // Example AID 1 (could be a specific service)
            "A0000002472001", // Example AID 2 (could be a specific service)
            "D2760000850100", // NFC Forum Application 1
            "D2760000850101", // NFC Forum Application 2
            "D2760001180101",
            "A0000000031010", // Visa Credit
            "A0000000032010", // Visa Debit
            "A0000000041010", // MasterCard
            "A0000000250101", // American Express
            "A000000555",     // Public Transport Services
            "A000000008",     // Calypso Cards
            "A000000040",     // National ID Applications
            "A000000010"      // NFC Forum Application
        ]
        for aid in aids {
            let data = aid.hex
            let select = NFCISO7816APDU(instructionClass: 0x00, instructionCode: 0xA4, p1Parameter: 0x04, p2Parameter: 0,
                                        data: data, expectedResponseLength: -1)
            if let (_, sw1, sw2) = try? await tag.sendCommand(apdu: select) {
                if sw1 == 0x90 && sw2 == 0x00 {
                    print("\(aid) IS SUPPORTED")
                } else {
                    print("\(aid) IS NOT SUPPORTED")
                }
            } else {
                print("\(aid) fails due to entitlement")
            }
        }
    }
}
