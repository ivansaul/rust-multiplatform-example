//
//  TaskCreateView.swift
//  Rusty
//
//  Created by ivansaul on 9/6/26.
//

import Factory
import RustyCore
import SwiftUI

struct CreateTaskView: View {
    @Environment(\.dismiss) var dismiss
    @Injected(\.taskCreateViewModel) private var viewModel

    var body: some View {
        @Bindable var viewModel = viewModel
        VStack {
            TextField("New task...", text: $viewModel.title)
                .padding()
                .overlay {
                    RoundedRectangle(cornerRadius: 15)
                        .stroke(Color.gray, lineWidth: 2)
                }

            Button("Add Task") {
                Task {
                    await viewModel.create_task()
                    dismiss()
                }
            }
            .buttonStyle(.borderedProminent)
        }
        .padding()
    }
}

#Preview {
    CreateTaskView()
}

@Observable
@MainActor
final class TaskCreateViewModel {
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
            title = ""
        } catch {
            state = .error(error)
        }
    }
}
