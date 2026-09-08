import SwiftUI

struct RootView: View {
    var body: some View {
        NavigationStack {
            TaskListView()
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
