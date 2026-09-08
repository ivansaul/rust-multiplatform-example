import Factory
import RustyCore
import SwiftUI

@main
struct RustyApp: App {
    @Injected(\.appBootstrap) private var bootstrap

    var body: some Scene {
        WindowGroup {
            Group {
                switch bootstrap.state {
                case .data:
                    RootView()
                case .error:
                    Text("Some Error Ocurred")
                default:
                    ProgressView()
                }
            }
            .task { await bootstrap.start() }
        }
    }
}
