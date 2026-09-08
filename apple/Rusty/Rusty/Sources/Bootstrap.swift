//
//  Bootstrap.swift
//  Rusty
//
//  Created by ivansaul on 9/7/26.
//

import Factory
import Foundation
import RustyCore

@MainActor
@Observable
final class AppBootstrap {
    private(set) var state: AsyncValue<RustyCore> = .idle

    private let storageManager: StorageManager

    init(storageManager: StorageManager) {
        self.storageManager = storageManager
    }

    func start() async {
        state = .loading
        do {
            let core = try RustyCore(context: appContext())
            registerCore(core)
            state = .data(core)
        } catch {
            state = .error(error)
            print(error)
        }
    }
}

extension AppBootstrap {
    private func registerCore(_ core: RustyCore) {
        Container.shared.appCore.register { core }
    }

    private func appContext() -> AppContext {
        AppContext(
            storage: Storage(
                appSupport: storageManager.applicationSupport.path(),
                cache: storageManager.caches.path(),
                documents: storageManager.documents.path()
            )
        )
    }
}
