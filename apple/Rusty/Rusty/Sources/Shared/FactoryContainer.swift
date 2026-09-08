//
//  Factory.swift
//  Rusty
//
//  Created by ivansaul on 8/15/26.
//

import Factory
import RustyCore

extension Container {
    var appCore: Factory<RustyCore> {
        self { fatalError() }
            .onPreview { try! RustyCore.preview() }
            .singleton
    }

    @MainActor
    var appBootstrap: Factory<AppBootstrap> {
        self { @MainActor in AppBootstrap(storageManager: self.storageManager()) }
            .singleton
    }

    var storageManager: Factory<StorageManager> {
        self { StorageManager() }
            .singleton
    }
}

extension Container {
    var taskListViewModel: Factory<TaskListViewModel> {
        self { @MainActor in TaskListViewModel(service: self.appCore().tasks()) }
    }

    var taskCreateViewModel: Factory<CreateTaskViewModel> {
        self { @MainActor in CreateTaskViewModel(taskService: self.appCore().tasks()) }
    }
}
