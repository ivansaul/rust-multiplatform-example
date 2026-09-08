//
//  TaskListView.swift
//  Rusty
//
//  Created by ivansaul on 9/3/26.
//

import Factory
import RustyCore
import SwiftUI

struct TaskListView: View {
    @Injected(\.taskListViewModel) private var viewModel
    @State private var showSheet: Bool = false

    var body: some View {
        @Bindable var viewModel = viewModel
        List {
            switch viewModel.tasks {
            case .loading:
                ProgressView()
            case let .error(error):
                Text(error.localizedDescription)
            case let .data(tasks):
                ForEach(tasks, id: \.id) { task in
                    TaskRowView(
                        task: task,
                        onToggle: { Task { await viewModel.toggle_completion(taskId: task.id) } },
                        onDelete: { Task { await viewModel.deleteTask(taskId: task.id) } }
                    )
                }
            case .idle:
                EmptyView()
            }
        }
        .listStyle(.plain)
        .navigationTitle("Tasks")
        .alert("onActionError", isPresented: $viewModel.showErrorAlert) {
            Button("Cancelar", role: .cancel) {}
        }
        .toolbar {
            ToolbarItem(placement: .topBarTrailing) {
                Button {
                    showSheet.toggle()
                } label: {
                    Image(systemName: "plus.circle.dashed")
                }
            }
        }
        .sheet(isPresented: $showSheet, onDismiss: {
            Task { await viewModel.refresh() }
        }, content: {
            CreateTaskView()
                .presentationDetents([.fraction(0.4)])
                .presentationDragIndicator(.visible)
        })
        .task {
            await viewModel.listTasks()
        }
    }
}

#Preview {
    NavigationStack {
        TaskListView()
    }
}
