import SwiftUI

struct RootView: View {
    var body: some View {
        NavigationStack {
            SettingsView()
                .padding()
                .navigationTitle("Rusty")
        }
    }
}

#Preview {
    NavigationStack {
        RootView()
    }
}