import Foundation
import CoreNFC
import CommonCrypto

extension String {
    var hex: Data {
        var data = Data()
        for i in stride(from: 0, to: self.count, by: 2) {
            let startIndex = self.index(self.startIndex, offsetBy: i)
            let endIndex = self.index(startIndex, offsetBy: 2)
            let byteString = self[startIndex..<endIndex]
            if let byte = UInt8(byteString, radix: 16) {
                data.append(byte)
            }
        }
        return data
    }
}

extension Data {
    func hex(separator: String = " ") -> String {
        self.map { String(format: "%02x", $0)}.joined(separator: separator)
    }
}


final class Signer: SignByCompanion {
    
    let tag: Data
    init(tag: String) {
        self.tag = tag.data(using: .utf8)!
    }
    
    func makeSignature(data: Data) -> Data {
        guard let privateKey = getPrivateKey() else { fatalError("Not an instance of a PrivateKey") }
        
        var error: Unmanaged<CFError>?
        guard let signature = SecKeyCreateSignature(privateKey, .ecdsaSignatureMessageX962SHA256, data as CFData, &error) else {
            fatalError("Error creating signature: \(String(describing: error))")
        }
        
        return signature as Data
    }

    func exportPublicKey() -> Data {
        guard let privateKey = getPrivateKey() else { fatalError("Not an instance of a PrivateKey") }
        guard let publicKey = SecKeyCopyPublicKey(privateKey) else { fatalError("Failed to retrieve public key") }

        var error: Unmanaged<CFError>?
        guard let publicKeyData = SecKeyCopyExternalRepresentation(publicKey, &error) else {
            fatalError("Error exporting public key: \(String(describing: error))")
        }

        return publicKeyData as Data
    }
    
    // Helper methods
    private func getPrivateKey() -> SecKey? {
        
        let query: [String: Any] = [
            kSecClass as String: kSecClassKey,
            kSecAttrKeyType as String: kSecAttrKeyTypeECSECPrimeRandom,
            kSecAttrApplicationTag as String: tag,
            kSecAttrKeyClass as String: kSecAttrKeyClassPrivate,
            kSecReturnRef as String: true
        ]
        
        var item: CFTypeRef?
        let status = SecItemCopyMatching(query as CFDictionary, &item)
        if status == errSecSuccess, let item {
            return (item as! SecKey)
        } else if status == errSecItemNotFound {
            return generatePrivateKey(tag: tag)
        } else {
            print("Error retrieving private key: \(status)")
            return nil
        }
    }
    
    private func generatePrivateKey(tag: Data) -> SecKey? {
        let attributes: [String: Any] = [
            kSecAttrKeyType as String: kSecAttrKeyTypeECSECPrimeRandom,
            kSecAttrKeySizeInBits as String: 256,
            kSecPrivateKeyAttrs as String: [
                kSecAttrIsPermanent as String: true,
                kSecAttrApplicationTag as String: tag
            ]
        ]

        var error: Unmanaged<CFError>?
        guard let privateKey = SecKeyCreateRandomKey(attributes as CFDictionary, &error) else {
            print("Error generating private key: \(String(describing: error))")
            return nil
        }

        return privateKey
    }
}


extension ErrorCompanion: LocalizedError {
    public var errorDescription: String? {
        switch self {
        case .Base58PrefixFormatNotSupported(let message): message
            
        case .Base58PrefixMismatch(let message): message
            
        case .BlockHashFormat(let message): message
            
        case .Client(let message): message
            
        case .DbInternal(let message): message
            
        case .DbTransaction(let message): message
            
        case .DecimalsFormatNotSupported(let message): message
            
        case .DecodeDbAddress(let message): message
            
        case .DecodeDbKey(let message): message
            
        case .DecodeDbMetadataSpecs(let message): message
            
        case .GenesisHashFormat(let message): message
        
        case .GenesisHashLength(let message): message
            
        case .InterfaceKey(let message): message
            
        case .LoadSpecsMetadata(let message): message
            
        case .LostAddress(let message): message
            
        case .LtError(let message): message
            
        case .MetaCut(let message): message
            
        case .MetadataFetchWithoutExistingEntry(let message): message
        
        case .MetadataFormat(let message): message
            
        case .MetadataNotDecodeable(let message): message
            
        case .MetadataVersion(let message): message
            
        case .NoBase58Prefix(let message): message
            
        case .NoDecimals(let message): message
            
        case .NoExistingEntryMetadataUpdate(let message): message
            
        case .NoMetadataV15(let message): message
            
        case .NoMetaPrefix(let message): message
            
        case .NotHex(let message): message
            
        case .NotSent(let message): message
            
        case .NotSubstrate(let message): message
            
        case .NoUnit(let message): message
            
        case .OnlySr25519(let message): message
            
        case .PoisonedLockSelector(let message): message
            
        case .PropertiesFormat(let message): message
            
        case .RawMetadataNotDecodeable(let message): message
            
        case .ReceiverClosed(let message): message
            
        case .ReceiverGuardPoisoned(let message): message
            
        case .RequestSer(let message): message
            
        case .ResponseDe(let message): message
            
        case .TooShort(let message): message
            
        case .TransactionNotParsable(let message): message
            
        case .UnexpectedFetch(let message): message
            
        case .UnitFormatNotSupported(let message): message
            
        case .UnknownPayloadType(let message): message
            
        case .UpdateMetadata(let message): message
            
        }
    }
}




extension NFCTypeNameFormat {
    var name: String {
        switch self {
        case .empty: "Empty"
        case .nfcWellKnown: "NFC Well Known"
        case .media: "Media"
        case .absoluteURI: "Absolute URI"
        case .nfcExternal: "NFC External"
        case .unknown: "Unknown"
        case .unchanged: "Unchanged"
        @unknown default: "New Unknown"
        }
    }
}

extension NFCNDEFStatus {
    var name: String {
        switch self {
        case .notSupported: "Not Supported"
        case .readWrite: "Read Write"
        case .readOnly: "Read Only"
        @unknown default: "Unknown"
        }
    }
}
