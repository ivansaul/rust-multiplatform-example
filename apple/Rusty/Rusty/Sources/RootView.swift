import Factory
import RustyCore
import SwiftUI

struct RootView: View {
    @Injected(\.settingsViewModel) private var settings

    var body: some View {
        TabView {
            NavigationStack {
                TaskListView()
            }
            .tabItem {
                Label("Tasks", systemImage: "checkmark.circle")
            }

            NavigationStack {
                SettingsListView()
            }
            .tabItem {
                Label("Settings", systemImage: "gearshape")
            }
        }
        .preferredColorScheme(settings.colorScheme)
    }
}
