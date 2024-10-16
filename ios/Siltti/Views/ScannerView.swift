import SwiftUI
import AVFoundation

public struct ScannerView: UIViewRepresentable {
    
    public typealias Completion = (ScanResult) -> Void
    
    public enum ScanResult {
        case string(String)
        case data(Data)
    }
    
    let types: [AVMetadataObject.ObjectType]
    let completion: Completion
    
    public init(types: [AVMetadataObject.ObjectType], completion: @escaping Completion) {
        self.types = types
        self.completion = completion
    }
    
    let session = AVCaptureSession()
    let output = AVCaptureMetadataOutput()
    
    func configure(sesssion: AVCaptureSession, output: AVCaptureMetadataOutput) {
        if  let device = AVCaptureDevice.default(for: .video),
            let input = try? AVCaptureDeviceInput(device: device) {
            if session.canAddInput(input) {
                session.addInput(input)
            }
            if session.canAddOutput(output) {
                session.addOutput(output)
            }
        }
    }
    
    public func makeUIView(context: Context) -> some UIView {
        
        configure(sesssion: session, output: output)
        if output.availableMetadataObjectTypes.contains(types) {
            output.metadataObjectTypes = types
        }
        
        let view = VideoView()
        view.previewLayer.session = session
        view.previewLayer.videoGravity = .resizeAspectFill
        
        output.setMetadataObjectsDelegate(context.coordinator, queue: .main)
        
        DispatchQueue.global(qos: .userInitiated).async {
            session.startRunning()
        }
        return view
    }
    
    public func updateUIView(_ uiView: UIViewType, context: Context) {
    }
    
    public func makeCoordinator() -> Coordinator {
        Coordinator(completion: completion)
    }
    
    public class Coordinator: NSObject, AVCaptureMetadataOutputObjectsDelegate {
        let completion: Completion
        init(completion: @escaping Completion) {
            self.completion = completion
        }
        
        public func metadataOutput(_ output: AVCaptureMetadataOutput, didOutput metadataObjects: [AVMetadataObject], from connection: AVCaptureConnection) {
            let object = metadataObjects.first as? AVMetadataMachineReadableCodeObject
            
            if let string = object?.stringValue {
                completion(.string(string))
            } else if let descriptor = object?.descriptor as? CIQRCodeDescriptor {
                let payload = descriptor.errorCorrectedPayload
                let rawData = descriptor.removeQrProtocolData(payload)
                completion(.data(rawData))
            }
        }
    }
}

#Preview {
    ScannerView(types: [.qr]) { address in}
}

final class VideoView: UIView {
    override class var layerClass: AnyClass {
        AVCaptureVideoPreviewLayer.self
    }
    
    var previewLayer: AVCaptureVideoPreviewLayer {
        layer as! AVCaptureVideoPreviewLayer
    }
}

extension CIQRCodeDescriptor {
    func removeQrProtocolData(_ input: Data) -> Data {
        var halves = input.halfBytes()
        var batch = takeBatch(&halves)
        var output = batch
        while !batch.isEmpty {
            batch = takeBatch(&halves)
            output.append(contentsOf: batch)
        }
        return Data(output)
    }
    
    private func takeBatch(_ input: inout [HalfByte]) -> [UInt8] {
        let version = self.symbolVersion
        let characterCountLength = version > 9 ? 16 : 8
        let mode = input.remove(at: 0)
        var output = [UInt8]()
        switch mode.value {
            // TODO If there is not only binary in the QRCode, then cases should be added here.
        case 0x04: // Binary
            let charactersCount: UInt16
            if characterCountLength == 8 {
                charactersCount = UInt16(input.takeUInt8())
            } else {
                charactersCount = UInt16(input.takeUInt16())
            }
            for _ in 0..<charactersCount {
                output.append(input.takeUInt8())
            }
            return output
        case 0x00: // End of data
            return []
        default:
            return []
        }
    }
}

fileprivate struct HalfByte {
    let value: UInt8
}

fileprivate extension [HalfByte] {
    mutating func takeUInt8() -> UInt8 {
        let left = self.remove(at: 0)
        let right = self.remove(at: 0)
        return UInt8(left, right)
    }
    
    mutating func takeUInt16() -> UInt16 {
        let first = self.remove(at: 0)
        let second = self.remove(at: 0)
        let third = self.remove(at: 0)
        let fourth = self.remove(at: 0)
        return UInt16(first, second, third, fourth)
    }
}

fileprivate extension Data {
    func halfBytes() -> [HalfByte] {
        var result = [HalfByte]()
        self.forEach { (byte: UInt8) in
            result.append(contentsOf: byte.halfBytes())
        }
        return result
    }
    
    init(_ halves: [HalfByte]) {
        var halves = halves
        var result = [UInt8]()
        while halves.count > 1 {
            result.append(halves.takeUInt8())
        }
        self.init(result)
    }
}

fileprivate extension UInt8 {
    func halfBytes() -> [HalfByte] {
        [HalfByte(value: self >> 4), HalfByte(value: self & 0x0F)]
    }
    
    init(_ left: HalfByte, _ right: HalfByte) {
        self.init((left.value << 4) + (right.value & 0x0F))
    }
}

fileprivate extension UInt16
{
    init(_ first: HalfByte, _ second: HalfByte, _ third: HalfByte, _ fourth: HalfByte) {
        let first = UInt16(first.value) << 12
        let second = UInt16(second.value) << 8
        let third = UInt16(third.value) << 4
        let fourth = UInt16(fourth.value) & 0x0F
        let result = first + second + third + fourth
        self.init(result)
    }
}
