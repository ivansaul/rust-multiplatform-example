//
//  TaskListViewModel.swift
//  Rusty
//
//  Created by ivansaul on 9/3/26.
//

import Foundation
import RustyCore

//@Observable
//@MainActor
//class TaskListViewModel {
//    private(set) var state: AsyncValue<[TaskItem]> = .idle
//
//    private let service: TaskService
//
//    init(service: TaskService) {
//        self.service = service
//    }
//
//    func load() async {
//        state = .loading
//        do {
//            state = try .data(await service.listTasks())
//        } catch {
//            state = .error(error)
//        }
//    }
//}

@Observable
@MainActor
class TaskListViewModel {
    private(set) var tasks: AsyncValue<[TaskItem]> = .idle

    private let service: TaskService

    init(service: TaskService) {
        self.service = service
    }

    func toggle_completed(taskId: String) async {
        guard
            case let .data(tasks) = tasks,
            let index = tasks.firstIndex(where: { $0.id == taskId })
        else { return }

        let task = tasks[index]

        do {
            if task.completed {
                _ = try await service.reopenTask(id: taskId)
            } else {
                _ = try await service.completeTask(id: taskId)
            }
            await listTasks()
        } catch {
            print(error)
        }
    }

    func deleteTask(taskId: String) async {
        do {
            try await service.deleteTask(id: taskId)
            await listTasks()
        } catch {
            print(error)
        }
    }

    func listTasks() async {
        tasks = .loading
        do {
            let res = try await service.listTasks()
            tasks = .data(res)
        } catch {
            print(error)
            tasks = .error(error)
        }
    }
}
