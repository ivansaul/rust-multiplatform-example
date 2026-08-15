import SwiftUI

@main
struct RustyApp: App {
    @State private var appearanceVM = AppearanceViewModel()

    var body: some Scene {
        WindowGroup {
            ContentView()
                .environment(appearanceVM)
                .preferredColorScheme(appearanceVM.theme.colorScheme)
        }
    }
}
