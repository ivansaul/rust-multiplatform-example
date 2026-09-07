//
//  Factory.swift
//  Rusty
//
//  Created by ivansaul on 8/15/26.
//

import Factory
import Foundation
import RustyCore

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

    var taskViewModel: Factory<TaskViewModel> {
        self { @MainActor in TaskViewModel(service: self.appCore().tasks()) }
    }

    var appCore: Factory<RustyCore> {
        self { fatalError() }
            .singleton
    }
}

extension Container {
    var taskCreateViewModel: Factory<TaskCreateViewModel> {
        self { @MainActor in TaskCreateViewModel(taskService: self.appCore().tasks()) }
    }
}
