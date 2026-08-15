//
//  Factory.swift
//  Rusty
//
//  Created by ivansaul on 8/15/26.
//

import Factory
import Foundation

extension Container {
    @MainActor
    var appearanceViewModel: Factory<AppearanceViewModel> {
        self { @MainActor in AppearanceViewModel(keyValueStorage: self.keyValueStorage()) }
            .singleton
    }

    var keyValueStorage: Factory<KeyValueStorage> {
        self { UserDefaultsStorage() }
            .onPreview { InMemoryStorage() }
            .singleton
    }
}
