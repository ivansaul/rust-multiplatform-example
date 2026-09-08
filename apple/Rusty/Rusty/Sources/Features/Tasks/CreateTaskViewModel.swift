//
//  CreateTaskViewModel.swift
//  Rusty
//
//  Created by ivansaul on 9/7/26.
//

import Foundation
import RustyCore

@Observable
@MainActor
final class CreateTaskViewModel {
    private(set) var state: AsyncValue<Void> = .idle
    var title: String = ""

    private let taskService: TaskService

    init(taskService: TaskService) {
        self.taskService = taskService
    }

    func create_task() async {
        state = .loading
        do {
            let input = CreateTaskItem(title: title)
            _ = try await taskService.createTask(input: input)
            title.removeAll()
        } catch {
            state = .error(error)
        }
    }
}
