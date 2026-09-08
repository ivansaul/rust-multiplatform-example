//
//  TaskListViewModel.swift
//  Rusty
//
//  Created by ivansaul on 9/3/26.
//

import Foundation
import RustyCore

@Observable
@MainActor
class TaskListViewModel {
    private(set) var tasks: AsyncValue<[TaskItem]> = .idle
    private(set) var onAction: AsyncValue<Void> = .idle
    var showErrorAlert: Bool = false

    private let service: TaskService

    init(service: TaskService) {
        self.service = service
    }

    func toggle_completion(taskId: String) async {
        guard
            case let .data(currentTasks) = tasks,
            let task = currentTasks.first(where: { $0.id == taskId })
        else { return }

        do {
            let updatedTask = task.completed
                ? try await service.reopenTask(id: taskId)
                : try await service.completeTask(id: taskId)

            let updatedTasks = currentTasks.map { $0.id == taskId ? updatedTask : $0 }
            tasks = .data(updatedTasks)
        } catch {
            handleError(error)
        }
    }

    func deleteTask(taskId: String) async {
        onAction = .loading
        do {
            try await service.deleteTask(id: taskId)
            await refresh()
        } catch {
            handleError(error)
        }
    }

    func listTasks() async {
        tasks = .loading
        do {
            tasks = try .data(await service.listTasks())
        } catch {
            tasks = .error(error)
        }
    }

    func refresh() async {
        do {
            let res = try await service.listTasks()
            tasks = .data(res)
        } catch {
            handleError(error)
        }
    }
}

extension TaskListViewModel {
    private func handleError(_ error: Error) {
        onAction = .error(error)
        showErrorAlert = true
    }
}
