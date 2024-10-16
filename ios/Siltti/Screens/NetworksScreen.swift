
import SwiftUI

struct NetworksScreen: View {
    
    let dbPath: String
    @State var address = ""
    @State var error = ""
    @State var chains: [ChainKey] = []
    @State var isUpdating = false
    
    func updateAllKeys() {
        Task { @MainActor in
            error = ""
            isUpdating = true
            do {
                while try !isUpdated(dbPath: dbPath) {
                    try? await Task.sleep(for: .seconds(1))
                }
                chains = try getAllKeys(dbPath: dbPath)
            } catch {
                self.error = error.localizedDescription
            }
            isUpdating = false
        }
    }
    
    func update(key: ChainKey) {
        do {
            error = ""
            try requestUpdateByKey(chainKey: key, dbPath: dbPath)
            updateAllKeys()
        } catch {
            self.error = error.localizedDescription
        }
    }
    
    func delete(key: ChainKey) {
        do {
            error = ""
            try deleteByKey(chainKey: key, dbPath: dbPath)
            updateAllKeys()
        } catch {
            self.error = error.localizedDescription
        }
    }
        
    var body: some View {
        ScrollView {
            VStack {
                if !error.isEmpty {
                    Text(error).foregroundStyle(.red)
                }
                TextField("Network RPC", text: $address)
                Button(action: { requestFullFetch(address: address); updateAllKeys() }) {
                    Text("Add Network").frame(minWidth: 200)
                }
                Button(action: { requestDefaults(); updateAllKeys() }) {
                    Text("Add Defaults").frame(minWidth: 200)
                }

                Section("Available Networks") {
                    ForEach(chains, id: \.self) { key in
                        Text(key)
                            .contextMenu {
                                Button(action: { update(key: key)}) {
                                    Label("Update", systemImage: "arrow.clockwise")
                                }
                                Button(role: .destructive, action: { delete(key: key)}) {
                                    Label("Delete", systemImage: "trash")
                                }
                            }
                    }
                }.padding(.top)
                
            }.padding(.horizontal)
        }
        .buttonStyle(.bordered)
        .textFieldStyle(.roundedBorder)
        .navigationTitle("Manage Networks")
        .onAppear {
            chains = (try? getAllKeys(dbPath: dbPath)) ?? []
        }.toolbar {
            ToolbarItem {
                ProgressView().opacity(isUpdating ? 1 : 0)
            }
        }
    }
}

#Preview {
    NetworksScreen(dbPath: "test_db")
}
