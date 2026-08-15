import SwiftUI

struct ContentView: View {
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
        ContentView()
            .environment(AppearanceViewModel())
    }
}
