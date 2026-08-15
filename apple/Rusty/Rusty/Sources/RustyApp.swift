import Factory
import SwiftUI

@main
struct RustyApp: App {
    @Injected(\.appearanceViewModel) private var appearanceViewModel: AppearanceViewModel

    var body: some Scene {
        WindowGroup {
            RootView()
                .preferredColorScheme(appearanceViewModel.theme.colorScheme)
        }
    }
}
